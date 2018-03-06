use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use app_dirs::{self, AppDataType, AppInfo};
use serde_json;
use error::*;
use list::NativeList;
use todo_queue_lib::query::{Filter, Query};
use todo_queue_lib::list::{Item, ItemDesc, List};
use todo_queue_lib::script;

const APP_INFO: AppInfo = AppInfo {
    name: "todo_queue",
    author: "R Miller",
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    config_path: PathBuf,
    list_paths: HashMap<String, PathBuf>,
    default_list: Option<String>,
}

pub struct App {
    lists: HashMap<String, NativeList>,
    config: AppConfig,
}

impl AppConfig {
    fn default_with_path(config_path: PathBuf) -> Self {
        Self {
            config_path,
            list_paths: HashMap::default(),
            default_list: None,
        }
    }

    pub fn load(config_path: PathBuf) -> Result<Self> {
        if !config_path.exists() {
            let app = Self::default_with_path(config_path);
            app.save_pretty().context(ErrorKind::SaveConfig)?;
            Ok(app)
        } else {
            let config_file = File::open(config_path).context(ErrorKind::LoadConfig)?;
            Ok(serde_json::from_reader(config_file).context(ErrorKind::LoadConfig)?)
        }
    }

    pub fn save_pretty(&self) -> Result<()> {
        if !self.config_path.exists() {
            let config_dir = self.config_path.parent().unwrap();
            fs::create_dir_all(config_dir).context(ErrorKind::SaveConfig)?;
        }
        let config_file = File::create(&self.config_path).context(ErrorKind::SaveConfig)?;
        serde_json::to_writer_pretty(config_file, self).context(ErrorKind::LoadConfig)?;
        Ok(())
    }

    pub fn launch(self) -> Result<App> {
        Ok(App {
            lists: self.list_paths
                .iter()
                .map(|(name, path)| Ok((name.clone(), NativeList::load(&path)?)))
                .collect::<Result<HashMap<_, _>>>()
                .context(ErrorKind::Launch)?,
            config: self,
        })
    }
}

impl App {
    pub fn get_file_in_config<P: AsRef<Path>>(&self, name: P) -> PathBuf {
        self.config.config_path.parent().unwrap().join(name)
    }

    pub fn add_list<S: Into<String>, P: Into<PathBuf>>(&mut self, name: S, path: P) -> Result<()> {
        let name = name.into();
        let path = path.into();

        if self.lists.contains_key(&name) {
            Err(ListAlreadyExists(name.clone())).context(ErrorKind::AddList)?;
        }

        let list = NativeList::load(path.clone()).context(ErrorKind::AddList)?;
        self.lists.insert(name.clone(), list);

        if self.lists.len() == 1 {
            self.config.default_list = Some(name.clone());
        }

        self.config.list_paths.insert(name, path);

        Ok(())
    }

    pub fn rm_list(&mut self, name: &str) -> Result<()> {
        self.lists
            .remove(name)
            .ok_or_else(|| NoSuchListExists(name.into()))
            .context(ErrorKind::RmList)?;
        self.config.list_paths.remove(name).unwrap();

        Ok(())
    }

    pub fn get_list(&self, name: Option<String>) -> Result<(String, &NativeList)> {
        let name = name.or_else(|| self.config.default_list.clone())
            .ok_or_else(|| NoListSelected)
            .context(ErrorKind::GetList)?;

        Ok(self.lists
            .get(&name)
            .map(|list| (name.clone(), list))
            .ok_or_else(|| NoSuchListExists(name.clone()))
            .context(ErrorKind::GetList)?)
    }

    pub fn get_list_mut(&mut self, name: Option<String>) -> Result<(String, &mut NativeList)> {
        let name = name.or_else(|| self.config.default_list.clone())
            .ok_or_else(|| NoListSelected)
            .context(ErrorKind::GetList)?;

        Ok(self.lists
            .get_mut(&name)
            .map(|list| (name.clone(), list))
            .ok_or_else(|| NoSuchListExists(name.clone()))
            .context(ErrorKind::GetList)?)
    }

    pub fn cli_show<Q: Into<Query>>(&self, query: Q) {
        let query = query.into();

        for (name, list) in self.lists.iter() {
            println!("{}: {}", name, list.get_path().to_str().unwrap_or("..."));
            for id in query.select(list) {
                let item = list.get(&id).unwrap();
                println!("\t{}", item.get_name());
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        self.config.save_pretty().context(ErrorKind::SaveApp)?;
        for (_, list) in self.lists.iter() {
            list.save_pretty().context(ErrorKind::SaveApp)?;
        }
        Ok(())
    }
}

pub fn run_cli() -> Result<App> {
    use clap::{App as Cli, Arg, SubCommand as Cmd};

    let cli = Cli::new("TodoQueue")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A simple task list tool")
        .arg(
            Arg::with_name("CONFIGURATION")
                .help("Sets the configuration file to use")
                .takes_value(true)
                .required(false)
                .short("c"),
        )
        .subcommand(
            Cmd::with_name("attach")
                .about("Attaches a new list to TodoQueue")
                .arg(Arg::with_name("NAME").help("The name of the list to add").takes_value(true).required(true))
                .arg(
                    Arg::with_name("PATH")
                        .help("Specify the path the the list's state file. If no path is given, the list will be placed at APP_CONFIG_DIR/LIST_NAME.json")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("DEFAULT")
                        .help("Sets this list as the default list")
                        .takes_value(false)
                        .long("--default")
                        .short("-d"),
                ),
        )
        .subcommand(
            Cmd::with_name("detach")
                .about("Detaches a list from TodoQueue")
                .arg(Arg::with_name("NAME").help("The name of the list to remove").takes_value(true).required(true))
        )
        .subcommand(Cmd::with_name("show").arg(Arg::with_name("QUERY").required(false).takes_value(true).last(true)))
        .subcommand(Cmd::with_name("add")
        .about("Adds a new item to a list")
        .arg(
            Arg::with_name("LIST")
            .help("The name of the list to add an item to.")
            .long("--list").short("-l").takes_value(true))
            .arg(
                Arg::with_name("ITEM").help("The specification of the item to add").required(true).takes_value(true))
                )
        .get_matches();

    // Get the path to use for configuration
    let config_path = cli.value_of("CONFIGURATION")
        .map(PathBuf::from)
        .ok_or(())
        .or_else(|_| {
            app_dirs::get_app_root(AppDataType::UserConfig, &APP_INFO)
                .map(|path| path.join("config.json"))
        })
        .context(ErrorKind::Cli)?;

    // Load the application configuration
    let app_config = AppConfig::load(config_path).context(ErrorKind::Cli)?;

    // Launch the application
    let mut app = app_config.launch().context(ErrorKind::Cli)?;

    if let Some(add_args) = cli.subcommand_matches("attach") {
        let name = add_args.value_of("NAME").unwrap();
        let list_path = add_args
            .value_of("PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                let mut path = app.get_file_in_config(name);
                path.set_extension("json");
                path
            });

        app.add_list(name, list_path).context(ErrorKind::Cli)?;
        app.save().context(ErrorKind::Cli)?;
    } else if let Some(rm_args) = cli.subcommand_matches("detach") {
        let name = rm_args.value_of("NAME").unwrap();

        app.rm_list(name).context(ErrorKind::Cli)?;
        app.save().context(ErrorKind::Cli)?;
    } else if let Some(show) = cli.subcommand_matches("show") {
        let query_str = show.value_of("QUERY").unwrap_or("all");
        let query = script::parse_query(query_str).unwrap();
        app.cli_show(query);
    } else if let Some(add) = cli.subcommand_matches("add") {
        let (_, list) = app.get_list_mut(add.value_of("LIST").map(String::from))
            .context(ErrorKind::Cli)?;
        let item = ItemDesc::from(add.value_of("ITEM").unwrap().to_string());
        list.add(item);

        list.save_pretty().context(ErrorKind::Cli)?;
    }

    Ok(app)
}
