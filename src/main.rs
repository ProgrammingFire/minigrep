use minigrep::run;
use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Query: {}", config.query);
    println!("File Path: {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {}", e)
    }
}
