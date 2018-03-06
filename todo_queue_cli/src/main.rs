extern crate app_dirs;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate rustyline;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate todo_queue_lib;

mod list;
mod app;
mod error;

fn main() {
    let app_config = app::AppConfig::load_config_from_default_location().unwrap();
    let app = app_config.launch().unwrap();
}
