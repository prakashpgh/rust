/*
cargo run -- ag1, arg2

-- => tells what follows are the arguments

lib.rs => library crate => we can refer the functions using the 
*/


use cli::Config;
use std::env;
use std::process;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem...");
        std::process::exit(1);
    });

    if let Err(e) = cli::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

