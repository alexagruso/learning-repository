use std::{env, error::Error, fs};

pub struct QueryConfig {
    pattern: String,
    file_name: String,
    ignore_case: bool,
}

impl QueryConfig {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        let pattern = match args.next() {
            Some(pattern) => pattern,
            None => return Err("Didn't receive a pattern string"),
        };

        let file_name = match args.next() {
            Some(file_name) => file_name,
            None => return Err("Didn't receive a file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(QueryConfig {
            pattern,
            file_name,
            ignore_case,
        })
    }
}

pub fn run(query_config: &QueryConfig) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(&query_config.file_name)?;

    let results = if query_config.ignore_case {
        search_case_insensitive(&query_config.pattern, &file_contents)
    } else {
        search(&query_config.pattern, &file_contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(&pattern))
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(pattern, contents)
        );
    }
}
