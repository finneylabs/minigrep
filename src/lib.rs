use std::fs;
use std::error::Error;

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
    let content = fs::read_to_string(config.filename)?;
    println!("{}", content);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_new() {
        let args = ["program_name".to_string(), "bad".to_string(), "man".to_string()];
        if let Ok(config) = Config::new(&args) {
            assert_eq!(config.query, "bad");
            assert_eq!(config.filename, "man");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn config_new_no_enough_args() {
        let args = ["program_name".to_string()];
        if let Err(err) = Config::new(&args) {
            assert_eq!(err, "not enough arguments")
        } else {
            assert!(false);
        }
    }
}