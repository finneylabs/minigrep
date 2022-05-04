use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(val) => val,
            None => return Err("didn't get a query string"),
        };

        let filename = match args.next() {
            Some(val) => val,
            None => return Err("didn't get a filename string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    //#[test]
    //fn config_new() {
    //   let args = ["program_name".to_string(), "bad".to_string(), "man".to_string()];
    //   if let Ok(config) = Config::new(&args) {
    //       assert_eq!(config.query, "bad");
    //       assert_eq!(config.filename, "man");
    //   } else {
    //       assert!(false);
    //   }
    //}

    //#[test]
    //fn config_new_no_enough_args() {
    //    let args = ["program_name".to_string()];
    //    if let Err(err) = Config::new(&args) {
    //        assert_eq!(err, "not enough arguments")
    //    } else {
    //        assert!(false);
    //    }
    //}

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape"],
            search_case_insensitive(query, content)
        );
    }
}