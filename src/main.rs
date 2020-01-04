#![allow(unused_variables)]
use rustb::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument: {}", err);
        process::exit(1)
    });

    println!("In file {}", config.filename);
    println!("Searching for {}\n\n", config.query);

    if let Err(e) = rustb::run(config) {
        println!("Application error {}", e);
        process::exit(1)
    }
}
