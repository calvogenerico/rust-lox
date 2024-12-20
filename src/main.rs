mod interpret;
mod parse;
mod scan;

use clap::{Parser, Subcommand};
use std::fs::File;
use std::process::ExitCode;

use crate::interpret::error::RuntimeError;
use crate::interpret::interpreter::Interpreter;
use crate::parse::expr::Expr;
use crate::parse::parse_error::ParseError;
use crate::parse::print_ast::PrintAst;
use crate::parse::stmt::Stmt;
use crate::scan::token::Token;
use parse::parser::LoxParser;
use scan::scanner::Scanner;

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
  Tokenize { file_path: String },

  #[command(arg_required_else_help = true)]
  Parse { file_path: String },
  #[command(arg_required_else_help = true)]
  Evaluate { file_path: String },
  #[command(arg_required_else_help = true)]
  Run { file_path: String },
}

struct ReportError {
  exit_code: u8,
  errors: Vec<String>,
}

impl From<Vec<String>> for ReportError {
  fn from(value: Vec<String>) -> Self {
    ReportError {
      errors: value,
      exit_code: 65,
    }
  }
}

impl From<std::io::Error> for ReportError {
  fn from(_value: std::io::Error) -> Self {
    ReportError {
      errors: vec!["Cannot read source file".to_string()],
      exit_code: 1,
    }
  }
}

impl From<ParseError> for ReportError {
  fn from(value: ParseError) -> Self {
    ReportError {
      exit_code: 65,
      errors: vec![value.to_string()],
    }
  }
}

impl From<RuntimeError> for ReportError {
  fn from(value: RuntimeError) -> Self {
    ReportError {
      exit_code: 70,
      errors: vec![value.to_string()],
    }
  }
}

fn scan(input: &mut File) -> Result<Vec<Token>, ReportError> {
  let scanner = Scanner::new(input);
  let (tokens, errors) = scanner.scan_tokens();
  if errors.len() > 0 {
    Err(errors)?
  } else {
    Ok(tokens)
  }
}

fn exec_main(cli: Cli) -> Result<String, ReportError> {
  match cli.command {
    Commands::Tokenize { file_path } => {
      let mut input = File::open(&file_path)?;
      let (tokens, errors) = Scanner::new(&mut input).scan_tokens();
      let strings = tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>();

      if errors.len() > 0 {
        for error in errors {
          eprintln!("{error}")
        }
        for line in strings {
          println!("{line}")
        }
        return Err(ReportError {
          errors: vec![],
          exit_code: 65,
        });
      }

      let strings = tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>();
      Ok(strings.join("\n"))
    }
    Commands::Parse { file_path } => {
      let mut input = File::open(&file_path)?;
      let tokens = scan(&mut input)?;
      let ast = parse(tokens)?;
      let printer = PrintAst::new();

      Ok(printer.print_stmts(&ast))
    }
    Commands::Evaluate { file_path } => {
      let mut input = File::open(&file_path)?;
      let tokens = scan(&mut input)?;
      let vec = parse(tokens)?;
      let ast = vec.first().unwrap();
      let expr = match ast {
        Stmt::Expr(expr) => expr,
        Stmt::Print(expr) => expr,
        _ => panic!("Evaluate can only evaluate a single expression"),
      };

      Ok(interpret_expr(expr)?)
    }
    Commands::Run { file_path } => {
      let mut input = File::open(&file_path)?;
      let tokens = scan(&mut input)?;
      let stmts = parse(tokens)?;
      Ok(interpret(stmts)?)
    }
  }
}

fn interpret_expr(expr: &Expr) -> Result<String, RuntimeError> {
  let stdout = std::io::stdout().lock();
  let mut interpreter = Interpreter::new(stdout);
  interpreter.interpret_expr(&expr).map(|v| v.to_string())
}

fn interpret(stmts: Vec<Stmt>) -> Result<String, RuntimeError> {
  let stdout = std::io::stdout().lock();
  let mut interpreter = Interpreter::new(stdout);
  interpreter.interpret_stmts(&stmts)?;
  Ok(String::new())
}

fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
  let parser = LoxParser::new(tokens);
  parser.parse()
}

fn main() -> ExitCode {
  let args = Cli::parse();

  match exec_main(args) {
    Ok(msg) => {
      println!("{}", msg);
      ExitCode::from(0)
    }
    Err(report) => {
      for msg in report.errors {
        eprintln!("{}", msg)
      }
      ExitCode::from(report.exit_code)
    }
  }
}
