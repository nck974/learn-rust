use std::error::Error;
use std::{env, fs};

/// Read a file and search for a string
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?; // Note the ? which may return an Error

    let results = if config.ignore_case {
        println!("Searching case insensitive");
        search_case_insensitive(&config.query, &content)
    } else {
        println!("Searching case sensitive");
        search_case_sensitive(&config.query, &content)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Return the lines matching a query not case sensitive
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    search(&query, &content, false)
}

/// Return the lines matching a query not case sensitive
pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    search(&query, &content, true)
}

/// Return the lines that match a query from a text
fn search<'a>(query: &str, content: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut matched_lines = Vec::new();
    for line in content.lines() {
        if case_sensitive {
            if line.contains(query) {
                matched_lines.push(line);
            }
        } else {
            if line.to_lowercase().contains(query.to_lowercase().as_str()) {
                matched_lines.push(line);
            }
        }
    }
    matched_lines
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Create a configuration object from the input of the user
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("{}", ignore_case);

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    /// Verify single parameter search
    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(&query, &content)
        );
    }

    #[test]
    fn case_sensitive_search() {
        let query = "Pick";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Pick three."], search_case_sensitive(&query, &content));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "pick";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["Pick three."],
            search_case_insensitive(&query, &content)
        );
    }
}
