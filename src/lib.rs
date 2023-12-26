use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run_grep(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a> (
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// ls

use std::fs::{DirEntry, ReadDir};
use std::time::SystemTime;
use chrono::{DateTime, Utc};

pub fn run_ls(entries: ReadDir) {
    for entry in entries.filter_map(Result::ok) {
        construct_path_str(entry);
    }
}

fn construct_path_str(entry: DirEntry) {
    let path = entry.path().to_string_lossy().to_string();
    let mut object_permissions = String::new(); 
    let mut last_modified_date = String::new(); 
    let mut file_size = String::new();
    
    if let Ok(metadata) = entry.metadata() {

        object_permissions = if metadata.permissions().readonly() {
            "Read-only".to_string()
        } else {
            "Writable".to_string()
        };

        if let Ok(time) = metadata.modified() {
            last_modified_date = format_system_time(time);
        }

        if metadata.is_file() {
            file_size = metadata.len().to_string();
        }
    }
    println!("{}", print_object(path, object_permissions, last_modified_date, file_size))
    
}

fn print_object(path: String, object_permissions: String, last_modified_date: String, file_size: String) -> String{
    let formatted_perms = format!("{:<width$}", object_permissions, width = 14);
    let formatted_mod_date = format!("{:>width$}", last_modified_date, width = 20);
    let formatted_size = format!("{:>width$}", file_size, width = 14);
    let formatted_path = format!("{:>width$}", path, width=30);
    format!("{}{}{}{}", formatted_perms, formatted_mod_date, formatted_size, formatted_path)
}

fn format_system_time(result: SystemTime) -> String {
    let datetime: DateTime<Utc> = result.into();

    datetime.format("%m/%d/%Y %I:%M %p").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}