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
  #[error("Cannot write to stdout")]
  CannotWriteToStdout,
  #[error("[line {0}]: Tried to divide by zero")]
  ZeroDivision(usize),
  #[error("[line {0}]: Expected function, got {1}")]
  NotAFunction(usize, String),
  #[error("[line {0}]: {1} expeted {2} arguments, but {3} received")]
  WrongNumberOfArguments(usize, String, usize, usize),
}
