use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RuntimeError {
  #[error("[line {0}]: Expected a number, got a {1}")]
  NotANumber(usize, String),
  #[error("[line {0}]: Operation {1} expected 2 numbers. Received {2} and {3}")]
  WrongBinaryOperationType(usize, String, String, String),
  #[error("Expression cannot be executed. Maybe there is an issue with the parser.")]
  InvalidExpression,
  #[error("[line {0}]: Undefined variable: {1}")]
  UndefinedVariable(usize, String),
}
