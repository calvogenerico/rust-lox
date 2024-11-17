mod scan;
mod parse;
mod interpret;

use std::fs::File;
use std::process::ExitCode;
use clap::{Error, Parser, Subcommand};

use scan::scanner::Scanner;
use parse::parser::LoxParser;
use crate::interpret::interpreter::Interpreter;
use crate::parse::print_ast::PrintAst;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git")]
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}


#[derive(Debug, Subcommand)]
enum Commands {
  #[command(arg_required_else_help = true)]
  Tokenize {
    file_path: String,
  },

  #[command(arg_required_else_help = true)]
  Parse {
    file_path: String,
  },
  #[command(arg_required_else_help = true)]
  Evaluate {
    file_path: String,
  },
}

fn main() -> Result<ExitCode, Error> {
  let args = Cli::parse();

  let code = match args.command {
    Commands::Tokenize { file_path: path } => {
      let mut input = File::open(&path)?;
      let scanner = Scanner::new(&mut input);
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
    Commands::Parse { file_path: path } => {
      let mut input = File::open(&path)?;
      let scanner = Scanner::new(&mut input);
      let (tokens, errors) = scanner.scan_tokens();

      let parser = LoxParser::new(tokens);
      let ast = parser.parse();

      if !errors.is_empty() || ast.is_err() {
        for error in &errors {
          eprintln!("{error}")
        }
        eprintln!("{}", ast.unwrap_err().to_string());
        return Ok(ExitCode::from(65))
      }

      let repr = PrintAst::new().print(&ast.unwrap());
      println!("{}", &repr);

      ExitCode::from(0)
    },
    Commands::Evaluate { file_path } => {
      let mut input = File::open(&file_path)?;
      let scanner = Scanner::new(&mut input);
      let (tokens, _errors) = scanner.scan_tokens();

      let parser = LoxParser::new(tokens);
      let ast = parser.parse().unwrap();

      let interpreter = Interpreter::new();
      let res = interpreter.interpret(&ast);
      if res.is_err() {
        eprintln!("{}", res.unwrap_err().to_string());
        return Ok(ExitCode::from(70));
      }
      let value = res.unwrap();

      println!("{}", value.to_string());

      ExitCode::from(0)
    }
  };
  Ok(code)
}
