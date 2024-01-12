use std::error::Error;
use walkdir::WalkDir;
use std::fs;
use std::path::Path;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub invert: bool,
    pub line_number: bool,
    pub recursive_search: bool,
}

pub fn run_grep(config: Config) -> Result<(), Box<dyn Error>> {
    if config.recursive_search {
        let path = Path::new(&config.file_path);
        if path.exists() && path.is_dir() {
            recursive_search(config)?;
            return Ok(())
        }
    }
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if !config.ignore_case {
        search_with_options(&config, &contents, |line, query| line.contains(query))
    } else {
        search_with_options(&config, &contents, |line, query| line.to_lowercase().contains(query))
    };

    for (line_number, line) in results {
        if !config.line_number {
            println!("{}", line);
        } else {
            println!("{} | {}", line_number, line);
        }
    }

    Ok(())
}

pub fn search_with_options<'a, F>(config: &Config, contents: &'a str, mut matcher:F ) -> Vec<(usize, &'a str)>
where
    F: FnMut(&str, &str) -> bool 
{
    let query = if config.ignore_case {
        config.query.to_lowercase()
    } else {
        config.query.clone()
    };

    contents
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let match_condition = matcher(line, &query);
            if (match_condition && !config.invert) || (!match_condition && config.invert) {
                Some((index + 1, line))
            } else {
                None
            }
        })
        .collect()
}


fn recursive_search(config: Config) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(&config.file_path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let contents = fs::read_to_string(entry.path())?;
            let results = if config.ignore_case {
                search_with_options(&config, &contents, |line, query| line.to_lowercase().contains(query))
            } else {
                search_with_options(&config, &contents, |line, query| line.contains(query))
            };

            for (line_number, line) in results {
                if config.line_number {
                    println!("{}:{}: {}", entry.path().display(), line_number, line);
                } else {
                    println!("{}: {}", entry.path().display(), line);
                }
            }
        }
    }
    Ok(())
}