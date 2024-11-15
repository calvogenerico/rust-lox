use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum InterpreterError {
  // #[error("[line {0}]: Expected a number, got a {1}")]
  // NotANumber(usize, String)
}