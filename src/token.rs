use crate::token_kind::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  kind: TokenKind,
  line: usize,
}

impl Token {
  pub fn new(kind: TokenKind, line: usize) -> Token {
    Token {
      kind,
      line
    }
  }

  pub fn to_string(&self) -> String {
    self.kind.to_string()
  }
}