use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    path: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new<T: Iterator<Item = String>>(mut args: T) -> Self {
        args.next();
        
        let query = args.next().unwrap();
        let path = args.next().unwrap();
        let case_insensitive = env::var("INSENSITIVE").map(|v| v == "1").unwrap_or(false);
        Self {
            query,
            path,
            case_insensitive,
        }
    } 

    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Self, &'static str> {
        args.next();
        
        let query = args.next().ok_or("Did not get a query string")?;
        let path = args.next().ok_or("Did not get a path string")?;
        let case_insensitive = env::var("INSENSITIVE").map(|v| v == "1").unwrap_or(false);
        Ok(Self {
            query,
            path,
            case_insensitive,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(&self.path)?;

        let result = if self.case_insensitive {
            Self::search_case_insensitive(&self.query, &content)
        } else {
            Self::search_case_sensitive(&self.query, &content)
        };

        for line in result {
            println!("{}", line);
        }
        
        Ok(())
    }

    fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents.lines().filter(|&l| l.contains(query)).collect()
    }

    fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let pattern = query.to_lowercase();
        contents.lines().filter(|&l| l.to_lowercase().contains(&pattern)).collect()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
Safe, Fast, Productive.
Select 3.";
        assert_eq!(vec!["Safe, Fast, Productive."], Config::search_case_sensitive(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "dUcT";
        let contents = "\
Rust:
Safe, Fast, Productive.
Select 3.";
        assert_eq!(vec!["Safe, Fast, Productive."], Config::search_case_insensitive(query, contents));
    }
}