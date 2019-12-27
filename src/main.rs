use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let filename = &args[1];
    let search_for = &args[2];

    println!("In file {}", filename);

    let contents = read_to_string(filename)
        .expect("Something went wrong on opening the file");
    println!("Content of the file:\n{}",contents);
}