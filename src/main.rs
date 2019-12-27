#![allow(unused_variables)]
use rustb::{parse_config, Config};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let config = Config::new(&args);

    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong on opening the file");
    println!("Content of the file:\n{}", contents);
}
