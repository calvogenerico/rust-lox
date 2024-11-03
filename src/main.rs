mod scanner;

use std::env;
use std::fs::File;
use std::io::{self, Write};
use clap::{Error, Parser, Subcommand};
use crate::scanner::Scanner;

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

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args.command {
        Commands::Tokenize { file_path: Some(path) } => {
            let mut input = File::open(&path)?;
            let mut scanner = Scanner::new();
            let scanned = scanner.scan_tokens(&mut input);

            for token in scanned {
                println!("{}", token.to_str());
            }
        },
        _ => panic!("error")
    }
    Ok(())
}
