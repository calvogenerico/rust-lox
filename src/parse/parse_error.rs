use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
  #[error("Malformed expression [line {0}]: {1}")]
  MalformedExpression(usize, String),
  #[error("Unexpected end of file")]
  UnexpectedEndOfFile,
  #[error("[line {0}]: Expected function name after fun.")]
  MissingFunctionName(usize),
}
