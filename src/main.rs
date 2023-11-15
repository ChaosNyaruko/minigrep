use std::process;
use std::env;
use minigrep::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let config = Config::build(&args).unwrap_or_else(|err| { 
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("query: {}, file: {}", config.query, config.file_path);
    if let Err(e) = minigrep::run(config) {
        println!("Application err {e}");
        process::exit(1);
    }
}
