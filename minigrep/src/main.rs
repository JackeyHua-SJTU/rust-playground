use std::{env, process::exit};

use minigrep::Config;

fn main() {
    let args = env::args();
    
    let conf = Config::build(args).unwrap_or_else(|x| {
        eprintln!("Error: {}", x);
        exit(1);
    });

    if let Err(err) = conf.run() {
        eprintln!("Error: {}", err);
        exit(1);
    }
}

