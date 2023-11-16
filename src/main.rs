use std::process;
use std::env;
use minigrep::{self, Config};

fn main() {
    // let dir = env::current_dir().unwrap();
    // let exe = env::current_exe().unwrap();
    // eprintln!("{:?}, {:?}", dir, exe);
    // dbg!(&args);
    let config = Config::build(env::args()).unwrap_or_else(|err| { 
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application err {e}");
        process::exit(1);
    }
}
