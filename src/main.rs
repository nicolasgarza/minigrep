use std::process;
use clap::{command, Arg, ArgAction};

use minigrep::Config;


fn main() {
    let matches = command!()
        .arg(Arg::new("string")
            .required(true)
            .index(1)
            .help("string to search for"))
        .arg(Arg::new("file")
            .required(true)
            .index(2)
            .help("file to search in"))
        .arg(Arg::new("ignore_case")
            .short('i')
            .long("ignore-case")
            .action(ArgAction::SetTrue)
            .help("ignore case distinctions"))
        .arg(Arg::new("invert")
            .short('v')
            .long("invert-matches")
            .action(ArgAction::SetTrue)
            .help("select non-matching lines"))
        .arg(Arg::new("line_number")
            .short('n')
            .long("line-numbers")
            .action(ArgAction::SetTrue)
            .help("prefix matches with their line numbers"))
        .arg(Arg::new("recursive_search")
            .short('r')
            .long("recursive")
            .action(ArgAction::SetTrue)
            .help("recursively search subdirectories"))
        .get_matches();
        
    let config = Config {
        query: matches.get_one::<String>("string").unwrap().to_string(),
        file_path: matches.get_one::<String>("file").unwrap().to_string(),
        ignore_case: matches.get_flag("ignore_case"),
        invert: matches.get_flag("invert"),
        line_number: matches.get_flag("line_number"),
        recursive_search: matches.get_flag("recursive_search"),
    };

    minigrep::run_grep(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
}


