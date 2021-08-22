use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); //consume the binary name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query string parameter"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no file name parameter"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename).expect("error reading specified file");

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_err() {
        let v = vec![String::from("ohai")];
        let c = Config::new(&v);
        assert!(c.is_err()); // assert error exists
    }

    #[test]
    fn config_new() {
        let v = vec![
            String::from("ohai"),
            String::from("cli"),
            String::from("file"),
        ];
        let c = Config::new(&v);
        assert!(c.is_ok()); // assert error exists
    }

    #[test]
    fn case_sensitve() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive,
Pick three.
Duct tape.";
        assert_eq! {
            vec!["Rust:","Trust me."],
            search(query, contents),
        }
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["saafe, fast, productive"], search(query, contents));
    }
}
