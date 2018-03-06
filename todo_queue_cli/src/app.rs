use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;

use app_dirs::{self, AppDataType, AppInfo};

const APP_INFO: AppInfo = AppInfo {
    name: "todo_queue",
    author: "R Miller",
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct App {
    lists: HashMap<String, PathBuf>,
}

impl App {
    pub fn load_config_from_default_location() -> Self {
        let config_dir = app_dirs::app_root(AppDataType::UserConfig, &APP_INFO);
        println!("{:?}", config);
        Self::default()
    }
}
