#![allow(unused_variables)]
use rustb::{parse_config, Config};
use std::{env, process};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing argument: {}", err);
        process::exit(1)
    });

    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong on opening the file");
    println!("Content of the file:\n{}", contents);
}
