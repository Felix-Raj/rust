pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        Config {
            filename: args[1].clone(),
            query: args[2].clone(),
        }
    }
}
