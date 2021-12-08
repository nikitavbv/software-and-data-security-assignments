use std::collections::HashMap;
use std::path::Path;
use env_logger::Env;
use hex::encode;
use indicatif::ProgressIterator;

use log::info;
use rand::prelude::{IteratorRandom, SliceRandom};

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

    if !Path::new("tasks/task3/output").exists() {
        info!("Running task3");
        tasks::task3::run();
    }

    if !Path::new("tasks/task4/output").exists() {
        info!("Running task4");
        tasks::task4::run();
    }

    if !Path::new("tasks/task5/output").exists() {
        info!("Running task5");
        tasks::task5::run();
    }

    if !Path::new("tasks/task6/output").exists() {
        info!("Running task6");
        tasks::task6::run();
    }

    info!("Finished running tasks");
}
