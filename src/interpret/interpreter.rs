use crate::interpret::error::InterpreterError;
use crate::parse::expr::Expr;

pub struct Interpreter {}

#[derive(Debug, PartialEq)]
pub enum Value {
  Number(f64),
  Nil,
  Boolean(bool),
}

impl Value {
  pub fn to_string(&self) -> String {
    match self {
      Value::Number(value) => format!("{value}"), 
      Value::Nil => "nil".to_string(),
      Value::Boolean(value) => format!("{value}")
    }
  }
}


impl Interpreter {

  pub fn new() -> Interpreter {
    Interpreter {}
  }

  pub fn interpret(&self, expr: Expr) -> Result<Value, InterpreterError> {
    match expr {
      Expr::LiteralNumber { value } => Ok(Value::Number(value)),
      Expr::LiteralNil => Ok(Value::Nil),
      Expr::LiteralBool { value } => Ok(Value::Boolean(value)),
      _ => panic!("not implemented")
    }
  }

}

#[cfg(test)]
mod tests {
  use crate::parse::parser::LoxParser;
  use crate::scan::token::Token;
  use crate::scan::token_kind::TokenKind;
  use super::*;
  fn interpret_tokens(tokens: Vec<Token>) -> Result<Value, InterpreterError> {
    let tokens = tokens;
    let expr = LoxParser::new(tokens).parse().unwrap();
    let interpreted = Interpreter::new().interpret(expr);
    interpreted
  }

  #[test]
  fn parse_number_1() {
    let interpreted = interpret_tokens(vec![Token::new(TokenKind::Number("1.0".to_string()), 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(1f64))
  }

  #[test]
  fn parse_number_2() {
    let interpreted = interpret_tokens(vec![Token::new(TokenKind::Number("2.0".to_string()), 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(2f64))
  }

  #[test]
  fn parse_nil() {
    let interpreted = interpret_tokens(vec![Token::new(TokenKind::Nil, 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Nil)
  }

  #[test]
  fn parse_true() {
    let interpreted = interpret_tokens(vec![Token::new(TokenKind::True, 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn parse_false() {
    let interpreted = interpret_tokens(vec![Token::new(TokenKind::False, 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }
}