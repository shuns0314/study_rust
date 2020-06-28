extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });

    if let Err(_e) = minigrep::run(config) {
        process::exit(1);
    }
}
