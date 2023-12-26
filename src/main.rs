use std::process;
use clap::Parser;
use std::fs;

use minigrep::Config;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    command: String,

    #[clap(value_parser)]
    other_args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    if cli.other_args.len() == 0{
        eprintln!("Not a proper cli command");
        process::exit(1);
    }

    match cli.command.as_str() {
        "rustls" => {
            match fs::read_dir(&cli.other_args[0]) {
                Ok(entries) => {
                    minigrep::run_ls(entries);
                },
                Err(e) => println!("Error: {}", e),
            };
        },
        "grep" => {
            let config = Config::build(cli.other_args.into_iter()).unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {err}");
                process::exit(1);
            });
        
            if let Err(e) = minigrep::run_grep(config) {
                eprintln!("Application error: {e}");
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Not a proper cli command");
            process::exit(1);
        }
    }

}

