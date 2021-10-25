use std::env::args;

mod tasks;
mod utils;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() <= 1 {
        eprintln!("please specify task to run");
        return;
    }

    match &args[1] {
        other => eprintln!("Unknown task: {}", other),
    }
}
