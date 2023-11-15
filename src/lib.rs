use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?; // .expect("File should be read into content");

    let res: Vec<&str>;
    if config.ignore_case {
        res = search_case_insensitive(&config.query, &content);
    } else {
        res = search(&config.query, &content);
    }
    for line in res {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argument");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, content)
        );
    }
}
