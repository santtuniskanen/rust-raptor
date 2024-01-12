use std::{env, process};
use rustraptor::Config;

fn main() {
    //
    // Collects the command line arguments into a vector
    //
    let args: Vec<String> = env::args().collect();

    //
    // Passes the vector `args` to the parse_config function
    //
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsin arguments: {err}");
        process::exit(1);
        });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    if let Err(e) = rustraptor::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
