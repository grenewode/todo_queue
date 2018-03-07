extern crate app_dirs;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;
extern crate rand;
extern crate rustyline;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate todo_queue_lib;

mod list;
mod app;
mod error;

fn main() {
    let _ = app::run_cli().unwrap();
}
