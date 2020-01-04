#![allow(unused_variables)]
use rustb::{Config, run};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing argument: {}", err);
        process::exit(1)
    });

    println!("In file {}", config.filename);
    println!("Searching for {}", config.query);
    run(config)
}
