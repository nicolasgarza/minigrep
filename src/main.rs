use std::process;
use clap::{command, Arg, ArgAction};
use std::fs;

use minigrep::Config;

fn main() {
    let matches = command!()
        .arg(Arg::new("command")
            .action(ArgAction::Set)
            .required(true)
            // .index(1)
        )
        .arg(Arg::new("args")
            .action(ArgAction::Append)
            // .last(true)
        )
        .get_matches();
        
    
    let command = matches.get_one::<String>("command").expect("required");
    let args = matches.get_many::<String>("args")
                                    .unwrap_or_default()
                                    .cloned()
                                    .collect::<Vec<String>>();

    
        let config = Config::build(args.into_iter()).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
        if let Err(e) = minigrep::run_grep(config) {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
}


