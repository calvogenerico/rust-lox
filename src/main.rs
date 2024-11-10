mod scanner;
mod token_kind;
mod token;

use std::fs::File;
use std::process::ExitCode;
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

fn main() -> Result<ExitCode, Error> {
    let args = Cli::parse();

    let code = match args.command {
        Commands::Tokenize { file_path: Some(path) } => {
            let mut input = File::open(&path)?;
            let mut scanner = Scanner::new(&mut input);
            let (tokens, errors) = scanner.scan_tokens();

            for error in &errors {
                eprintln!("{error}")
            }

            for token in tokens {
                println!("{}", token.to_string());
            }

            if errors.len() == 0 {
                ExitCode::from(0)
            } else {
                ExitCode::from(65)
            }

        },
        _ => ExitCode::from(1)
    };
    Ok(code)
}
