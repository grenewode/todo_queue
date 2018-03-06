use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use app_dirs::{self, AppDataType, AppInfo};
use serde_json;
use error::*;
use list::NativeList;

const APP_INFO: AppInfo = AppInfo {
    name: "todo_queue",
    author: "R Miller",
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    config_path: PathBuf,
    list_paths: HashMap<String, PathBuf>,
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
        }
    }

    pub fn load_config_from_default_location() -> Result<Self> {
        let config_path = app_dirs::get_app_root(AppDataType::UserConfig, &APP_INFO)
            .context(ErrorKind::LoadConfig)?
            .join("config.json");

        Self::load(config_path)
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
