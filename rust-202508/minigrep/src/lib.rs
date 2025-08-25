use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");

        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})

    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {


    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    let search_results = search(&config.query, &contents);
    for line in search_results {
        println!("{}", line);
    }
    println!("searching end");
    Ok(())


}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        println!("this is lines:{line}");
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}