use rustraptor::{Config, run};

fn main() {
    //
    // Collects the command line arguments into a vector
    //
    let args: Vec<String> = std::env::args().collect();

    //
    // Passes the vector `args` to the parse_config function
    //
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}
