#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  // Single-character tokens.
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,

  // One or Two tokens
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,

  // Literals
  Number(String),
  String(String),
  Identifier(String),

  // Keywords
  And,
  Class,

  Eof,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
}

impl TokenKind {
  pub fn symbol(&self) -> String {
    match self {
      TokenKind::LeftParen => "(".to_string(),
      TokenKind::RightParen => ")".to_string(),
      TokenKind::LeftBrace => "{".to_string(),
      TokenKind::RightBrace => "}".to_string(),
      TokenKind::Comma => ",".to_string(),
      TokenKind::Dot => ".".to_string(),
      TokenKind::Minus => "-".to_string(),
      TokenKind::Plus => "+".to_string(),
      TokenKind::Semicolon => ";".to_string(),
      TokenKind::Slash => "/".to_string(),
      TokenKind::Star => "*".to_string(),
      TokenKind::Bang => "!".to_string(),
      TokenKind::BangEqual => "!=".to_string(),
      TokenKind::Equal => "=".to_string(),
      TokenKind::EqualEqual => "==".to_string(),
      TokenKind::Greater => ">".to_string(),
      TokenKind::GreaterEqual => ">=".to_string(),
      TokenKind::Less => "<".to_string(),
      TokenKind::LessEqual => "<=".to_string(),
      TokenKind::Number(value) => value.to_string(),
      TokenKind::String(value) => value.to_string(),
      TokenKind::Identifier(value) => value.to_string(),
      TokenKind::And => "and".to_string(),
      TokenKind::Class => "class".to_string(),
      TokenKind::Else => "else".to_string(),
      TokenKind::False => "false".to_string(),
      TokenKind::Fun => "fun".to_string(),
      TokenKind::For => "for".to_string(),
      TokenKind::If => "if".to_string(),
      TokenKind::Nil => "nil".to_string(),
      TokenKind::Or => "or".to_string(),
      TokenKind::Print => "print".to_string(),
      TokenKind::Return => "return".to_string(),
      TokenKind::Super => "super".to_string(),
      TokenKind::This => "this".to_string(),
      TokenKind::True => "true".to_string(),
      TokenKind::Var => "var".to_string(),
      TokenKind::While => "while".to_string(),
      TokenKind::Eof => "".to_string(), // The double space is on purpose. The representation is empty.
    }
  }

  pub fn full_format(&self) -> String {
    match self {
      TokenKind::LeftParen => "LEFT_PAREN ( null".to_string(),
      TokenKind::RightParen => "RIGHT_PAREN ) null".to_string(),
      TokenKind::LeftBrace => "LEFT_BRACE { null".to_string(),
      TokenKind::RightBrace => "RIGHT_BRACE } null".to_string(),
      TokenKind::Comma => "COMMA , null".to_string(),
      TokenKind::Dot => "DOT . null".to_string(),
      TokenKind::Minus => "MINUS - null".to_string(),
      TokenKind::Plus => "PLUS + null".to_string(),
      TokenKind::Semicolon => "SEMICOLON ; null".to_string(),
      TokenKind::Slash => "SLASH / null".to_string(),
      TokenKind::Star => "STAR * null".to_string(),
      TokenKind::Bang => "BANG ! null".to_string(),
      TokenKind::BangEqual => "BANG_EQUAL != null".to_string(),
      TokenKind::Equal => "EQUAL = null".to_string(),
      TokenKind::EqualEqual => "EQUAL_EQUAL == null".to_string(),
      TokenKind::Greater => "GREATER > null".to_string(),
      TokenKind::GreaterEqual => "GREATER_EQUAL >= null".to_string(),
      TokenKind::Less => "LESS < null".to_string(),
      TokenKind::LessEqual => "LESS_EQUAL <= null".to_string(),
      TokenKind::Number(value) => format!("NUMBER {value} {:?}", value.parse::<f64>().unwrap()),
      TokenKind::String(value) => format!("STRING \"{value}\" {value}"),
      TokenKind::Identifier(value) => format!("IDENTIFIER {value} null"),
      TokenKind::And => "AND and null".to_string(),
      TokenKind::Class => "CLASS class null".to_string(),
      TokenKind::Else => "ELSE else null".to_string(),
      TokenKind::False => "FALSE false null".to_string(),
      TokenKind::Fun => "FUN fun null".to_string(),
      TokenKind::For => "FOR for null".to_string(),
      TokenKind::If => "IF if null".to_string(),
      TokenKind::Nil => "NIL nil null".to_string(),
      TokenKind::Or => "OR or null".to_string(),
      TokenKind::Print => "PRINT print null".to_string(),
      TokenKind::Return => "RETURN return null".to_string(),
      TokenKind::Super => "SUPER super null".to_string(),
      TokenKind::This => "THIS this null".to_string(),
      TokenKind::True => "TRUE true null".to_string(),
      TokenKind::Var => "VAR var null".to_string(),
      TokenKind::While => "WHILE while null".to_string(),
      TokenKind::Eof => "EOF  null".to_string(), // The double space is on purpose. The representation is empty.
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn eof_to_string() {
    assert_eq!(&TokenKind::Eof.full_format(), "EOF  null")
  }

  #[test]
  fn bang_equal_to_string() {
    assert_eq!(&TokenKind::BangEqual.full_format(), "BANG_EQUAL != null")
  }

  #[test]
  fn less_to_string() {
    assert_eq!(&TokenKind::Less.full_format(), "LESS < null")
  }

  #[test]
  fn less_equal_to_string() {
    assert_eq!(&TokenKind::LessEqual.full_format(), "LESS_EQUAL <= null")
  }

  #[test]
  fn identifier_to_string() {
    assert_eq!(
      &TokenKind::Identifier("foo".to_string()).full_format(),
      "IDENTIFIER foo null"
    )
  }

  #[test]
  fn string_to_string() {
    assert_eq!(
      &TokenKind::String("foo".to_string()).full_format(),
      "STRING \"foo\" foo"
    )
  }

  #[test]
  fn number_to_string() {
    assert_eq!(
      &TokenKind::Number("47.0".to_string()).full_format(),
      "NUMBER 47.0 47.0"
    )
  }

  #[test]
  fn number_to_string_integers() {
    assert_eq!(
      &TokenKind::Number("47".to_string()).full_format(),
      "NUMBER 47 47.0"
    )
  }

  #[test]
  fn number_to_2_decimals() {
    assert_eq!(
      &TokenKind::Number("47.11".to_string()).full_format(),
      "NUMBER 47.11 47.11"
    )
  }

  #[test]
  fn number_to_string_ending_with_dot() {
    assert_eq!(
      &TokenKind::Number("47.".to_string()).full_format(),
      "NUMBER 47. 47.0"
    )
  }

  #[test]
  fn super_to_string() {
    assert_eq!(&TokenKind::Super.full_format(), "SUPER super null")
  }

  #[test]
  fn or_to_string() {
    assert_eq!(&TokenKind::Or.full_format(), "OR or null")
  }

  #[test]
  fn var_to_string() {
    assert_eq!(&TokenKind::Var.full_format(), "VAR var null")
  }
}
