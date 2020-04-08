use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // print file content
    let contents = fs::read_to_string(config.filename)?;

    println!("File Contents:\n{}", contents);

    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: &args[1],
            filename: &args[2],
        })
    }
}
