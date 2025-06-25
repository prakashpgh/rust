use std::env;
use std::fs;

fn read_file(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("read the file");
    println!("file contents:\n {}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments...");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

use std::process;
use std::error::Error;


fn run(config : Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("file contents:\n {}", contents);
    Ok(())
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem...");
        std::process::exit(1);
    });

    run(config);
}

/*
cargo run -- ag1, arg2

-- => tells what follows are the arguments

*/
