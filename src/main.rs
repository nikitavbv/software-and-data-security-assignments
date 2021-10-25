use std::path::Path;
use env_logger::Env;

use log::info;

mod tasks;
mod utils;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    if !Path::new("tasks/task1/output").exists() {
        info!("Running task1");
        tasks::task1::run();
    }

    if !Path::new("tasks/task2/output").exists() {
        info!("Running task2");
        tasks::task2::run();
    }

    info!("Finished running tasks");
}
