use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file text
    let contents = fs::read_to_string(config.filename)?;

    // search with/without case sensitivity
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // print each search match
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// User args for grep-like command
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: &args[1],
            filename: &args[2],
            // set based on if environment variable is present.
            // ex. $ CASE_INSENSITIVE=1 cargo run --bin ch12_building_a_cli_tool to ./assets/poem.txt
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    // separate text by line
    for line in contents.lines() {
        // push lines, with query, to results
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = vec![];

    // separate text by line
    for line in contents.lines() {
        // push lines, with query, to results
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
