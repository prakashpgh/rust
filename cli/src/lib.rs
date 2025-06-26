use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_insensitive: bool,
}

fn string_to_bool(s: &str) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" | "t" | "yes" | "y" | "1" => Some(true),
        "false" | "f" | "no" | "n" | "0" => Some(false),
        _ => None, // Or you could return a Result<bool, Error> for more detailed error handling
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments...");
        }
        let mut case_insensitive = false;
        let query = args[1].clone();
        let file_path = args[2].clone();
        if args.len() > 3 {
            case_insensitive = string_to_bool(&args[3].clone()).unwrap();
        }
        Ok(Config {
            query,
            file_path,
            case_insensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        let query_to_match = if case_insensitive {
            query.to_lowercase()
        } else {
            query.to_string()
        };
        if line.to_lowercase().contains(&query_to_match) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    //println!("file contents:\n {}", contents);
    for line in search(&config.query, &contents, config.case_insensitive) {
        println!("{line}")
    }
    Ok(())
}
