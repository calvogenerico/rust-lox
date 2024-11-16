use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
  #[error("Malformed expression [line {0}]: {1}")]
  MalformedExpression(usize, String),
  #[error("Missing end of file")]
  MissingEOF
}