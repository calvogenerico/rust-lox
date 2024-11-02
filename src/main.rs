use std::env;
use std::fs;
use std::io::{self, Write};
use clap::{Parser, Subcommand};


#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Debug, Subcommand)]
enum Commands {
    /// Clones repos
    #[command(arg_required_else_help = true)]
    Tokenize {
        file_path: Option<String>,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Tokenize { file_path: Some(path) } => {
            let file_contents = fs::read_to_string(&path).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", &path).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                panic!("Scanner not implemented");
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        },
        _ => panic!("error")
    }
}
