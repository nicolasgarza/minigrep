use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub invert: bool,
    pub line_number: bool,
    pub recursive_search: bool,
}

pub fn run_grep(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match fs::read_to_string(&config.file_path) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };

    let results = if config.ignore_case {
        search_case_insensitive(&config, &contents)
    } else {
        search(&config, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    if !config.invert {
        contents
        .lines()
        .filter(|line| line.contains(config.query.as_str()))
        .collect()
    } else {
        contents
        .lines()
        .filter(|line| !line.contains(config.query.as_str()))
        .collect()
    }
    
}

pub fn search_case_insensitive<'a>(config: &Config, contents: &'a str) -> Vec<&'a str> {
    let binding = config.query.to_lowercase();
    let query = binding.as_str();

    if !config.invert {
        contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
    } else {
        contents
        .lines()
        .filter(|line| !line.to_lowercase().contains(query))
        .collect()
    }
    

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = String::from("duct");
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(
                &Config {
                    query,
                    file_path: String::from(""),
                    ignore_case: false,
                    invert: false,
                    line_number: false,
                    recursive_search: false
                },
                contents
            )
        );
    }

    #[test]
    fn case_insensitive() {
        let query = String::from("rUsT");
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(
                &Config {
                    query,
                    file_path: String::from(""),
                    ignore_case: false,
                    invert: false,
                    line_number: false,
                    recursive_search: false
                },
                contents
            )
        );
    }
}
