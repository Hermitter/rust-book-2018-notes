// cargo run --bin ch12_building_a_cli_tool test ./assets/poem.txt
pub mod lib;
mod tests;
use lib::Config;
use std::env;
use std::process;

fn main() {
    // return cli args as a vector of strings
    let args: Vec<String> = env::args().collect();
    println!("Captured Args: {:?}", args);

    // extract the regex query and filename to scan
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
