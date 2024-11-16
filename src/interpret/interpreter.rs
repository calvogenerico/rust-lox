use crate::interpret::error::InterpreterError;
use crate::parse::expr::Expr;
use crate::scan::token_kind::TokenKind;

pub struct Interpreter {}

#[derive(Debug, PartialEq)]
pub enum Value {
  Number(f64),
  Nil,
  Boolean(bool),
  String(String),
}

impl Value {
  pub fn to_string(&self) -> String {
    match self {
      Value::Number(value) => format!("{value}"),
      Value::Nil => "nil".to_string(),
      Value::Boolean(value) => format!("{value}"),
      Value::String(value) => value.to_string()
    }
  }
}


impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {}
  }

  pub fn interpret(&self, expr: &Expr) -> Result<Value, InterpreterError> {
    match expr {
      Expr::LiteralNumber { value } => Ok(Value::Number(*value)),
      Expr::LiteralNil => Ok(Value::Nil),
      Expr::LiteralBool { value } => Ok(Value::Boolean(*value)),
      Expr::Unary { operator, right } => {
        let value = self.interpret(right)?;
        match (value, operator.kind()) {
          (Value::Number(value), TokenKind::Minus) => Ok(Value::Number(-value)),
          (val, TokenKind::Bang) => Ok(Value::Boolean(!self.is_truthy(&val))),
          _ => panic!("not implemented")
        }
      }
      Expr::LiteralString { value } => Ok(Value::String(value.to_string())),
      Expr::Group { expression } => self.interpret(expression),
      Expr::Binary { left, operator, right } => {
        let left = self.interpret(left)?;
        let right = self.interpret(right)?;

        match (operator.kind(), &left, &right) {
          (TokenKind::Plus, Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
          (TokenKind::Minus, Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
          (TokenKind::Plus, Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{s1}{s2}"))),
          _ => panic!()
        }
      }
    }
  }

  fn is_truthy(&self, value: &Value) -> bool {
    !self.is_falsey(value)
  }

  fn is_falsey(&self, value: &Value) -> bool {
    match value {
      Value::Nil => true,
      Value::Boolean(false) => true,
      _ => false
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
    let interpreted = Interpreter::new().interpret(&expr);
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

  #[test]
  fn parse_minus_one() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::Number("1".to_string()), 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(-1.0))
  }

  #[test]
  fn parse_minus_string() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::String("foo".to_string()), 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::String("foo".to_string()))
  }

  #[test]
  fn eval_not_true_returns_false() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::True, 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_not_false_returns_true() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::False, 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_not_a_positive_number_returns_false() {
    // any number is truty
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::Number("1.0".to_string()), 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_not_zero_returns_false() {
    // any number is truty
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::Number("0".to_string()), 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_not_nil_returns_true() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::Nil, 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_a_group_returns_inner_expr() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::LeftParen, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::RightParen, 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(1.0))
  }

  #[test]
  fn eval_an_addition_returns_the_result() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::Number("2".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(3.0))
  }

  #[test]
  fn eval_a_subtraction_returns_the_result() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::Number("5".to_string()), 1),
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(4.0))
  }

  #[test]
  fn eval_a_subtraction_can_return_negative_number() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::Number("5".to_string()), 1),
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::Number("12".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(-7.0))
  }

  #[test]
  fn eval_a_plus_between_strings_concatenate_strings() {
    let interpreted = interpret_tokens(vec![
      Token::new(TokenKind::String("foo".to_string()), 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::String("bar".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::String("foobar".to_string()))
  }
}