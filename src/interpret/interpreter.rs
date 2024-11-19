use crate::interpret::error::InterpreterError;
use crate::parse::expr::Expr;
use crate::parse::stmt::Stmt;
use crate::scan::token::Token;
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

  pub fn type_name(&self) -> &'static str {
    match self {
      Value::Number(_) => "Number",
      Value::Nil => "nil",
      Value::Boolean(_) => "Boolean",
      Value::String(_) => "String",
    }
  }
}


impl Interpreter {
  pub fn new() -> Interpreter {
    Interpreter {}
  }

  pub fn interpret_stmts(&mut self, stmts: &[Stmt]) -> Result<(), InterpreterError> {
    for stmt in stmts {
      match stmt {
        Stmt::Expr(expr) => { self.interpret_expr(expr)?; },
        Stmt::Print(expr) => {
          let value = self.interpret_expr(expr)?;
          println!("{}", value.to_string());
        }
        Stmt::Var(_, _, _) => panic!("not implemented")
      }
    }
    Ok(())
  }

  pub fn interpret_expr(&self, expr: &Expr) -> Result<Value, InterpreterError> {
    match expr {
      Expr::LiteralNumber { value } => Ok(Value::Number(*value)),
      Expr::LiteralNil => Ok(Value::Nil),
      Expr::LiteralBool { value } => Ok(Value::Boolean(*value)),
      Expr::Unary { operator, right } => self.unary(operator, right)?,
      Expr::LiteralString { value } => Ok(Value::String(value.to_string())),
      Expr::Group { expression } => self.interpret_expr(expression),
      Expr::Binary { left, operator, right } => self.binary(left, operator, right)?,
      Expr::Variable { .. } => unimplemented!(),
      Expr::Assign { .. } => unimplemented!()
    }
  }

  fn unary(&self, operator: &Token, right: &Expr) -> Result<Result<Value, InterpreterError>, InterpreterError> {
    let value = self.interpret_expr(right)?;
    Ok(match (value, operator.kind()) {
      (Value::Number(value), TokenKind::Minus) => Ok(Value::Number(-value)),
      (val, TokenKind::Bang) => Ok(Value::Boolean(!self.is_truthy(&val))),
      (value, TokenKind::Minus) => Err(InterpreterError::NotANumber(operator.line(), value.type_name().to_string())),
      _ => Err(InterpreterError::InvalidExpression)
    })
  }

  fn binary(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<Result<Value, InterpreterError>, InterpreterError> {
    let left = self.interpret_expr(left)?;
    let right = self.interpret_expr(right)?;

    Ok(match (operator.kind(), &left, &right) {
      (TokenKind::EqualEqual, val1, val2) => Ok(Value::Boolean(self.are_equal(val1, val2))),
      (TokenKind::BangEqual, val1, val2) => Ok(Value::Boolean(!self.are_equal(val1, val2))),
      (TokenKind::Plus, Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
      (TokenKind::Minus, Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
      (TokenKind::Star, Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
      (TokenKind::Slash, Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
      (TokenKind::Less, Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 < n2)),
      (TokenKind::LessEqual, Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 <= n2)),
      (TokenKind::Greater, Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 > n2)),
      (TokenKind::GreaterEqual, Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1 >= n2)),
      (TokenKind::Plus, Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{s1}{s2}"))),
      (
        TokenKind::Greater
        | TokenKind::GreaterEqual
        | TokenKind::Less
        | TokenKind::LessEqual
        | TokenKind::Plus
        | TokenKind::Star
        | TokenKind::Slash,
        val1,
        val2) =>
        Err(InterpreterError::WrongBinaryOperationType(
          operator.line(),
          operator.kind().symbol(),
          val1.type_name().to_string(),
          val2.type_name().to_string()
        )),

      _ => Err(InterpreterError::InvalidExpression)
    })
  }

  fn are_equal(&self, val1: &Value, val2: &Value) -> bool {
    match (val1, val2) {
      (Value::Number(n1), Value::Number(n2)) => n1 == n2,
      (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
      (Value::String(s1), Value::String(s2)) => s1 == s2,
      (val1, val2) => val1 == val2
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
  use crate::parse::stmt::Stmt;
  use crate::scan::token::Token;
  use crate::scan::token_kind::TokenKind;
  use super::*;
  fn interpret_expression(tokens: Vec<Token>) -> Result<Value, InterpreterError> {
    let tokens = tokens;
    let stmts = LoxParser::new(tokens).parse().unwrap();
    let interpreter = Interpreter::new();
    match stmts.first().unwrap() {
      Stmt::Expr(expr) => { interpreter.interpret_expr(expr) }
      _ => panic!("should be an expr")
    }
  }

  #[test]
  fn eval_number_1() {
    let interpreted = interpret_expression(vec![Token::new(TokenKind::Number("1.0".to_string()), 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(1f64))
  }

  #[test]
  fn eval_number_2() {
    let interpreted = interpret_expression(vec![Token::new(TokenKind::Number("2.0".to_string()), 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(2f64))
  }

  #[test]
  fn eval_nil() {
    let interpreted = interpret_expression(vec![Token::new(TokenKind::Nil, 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Nil)
  }

  #[test]
  fn eval_true() {
    let interpreted = interpret_expression(vec![Token::new(TokenKind::True, 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_false() {
    let interpreted = interpret_expression(vec![Token::new(TokenKind::False, 1)]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_minus_one() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::Number("1".to_string()), 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(-1.0))
  }

  #[test]
  fn eval_minus_string() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::String("foo".to_string()), 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::String("foo".to_string()))
  }

  #[test]
  fn eval_not_true_returns_false() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::True, 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_not_false_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::False, 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_not_a_positive_number_returns_false() {
    // any number is truthy
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::Number("1.0".to_string()), 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_not_zero_returns_false() {
    // any number is truthy
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::Number("0".to_string()), 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_not_nil_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::Nil, 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_a_group_returns_inner_expr() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::LeftParen, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::RightParen, 1)
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(1.0))
  }

  #[test]
  fn eval_an_addition_returns_the_result() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::Number("2".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(3.0))
  }

  #[test]
  fn eval_a_subtraction_returns_the_result() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("5".to_string()), 1),
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(4.0))
  }

  #[test]
  fn eval_a_subtraction_can_return_negative_number() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("5".to_string()), 1),
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::Number("12".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(-7.0))
  }

  #[test]
  fn eval_a_plus_between_strings_concatenate_strings() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::String("foo".to_string()), 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::String("bar".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::String("foobar".to_string()))
  }

  #[test]
  fn eval_a_star_between_numbers_multiplies() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("7".to_string()), 1),
      Token::new(TokenKind::Star, 1),
      Token::new(TokenKind::Number("3".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(21.0))
  }

  #[test]
  fn eval_a_slash_between_numbers_divides() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Slash, 1),
      Token::new(TokenKind::Number("2".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(0.5))
  }

  #[test]
  fn eval_1_lower_than_2_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Less, 1),
      Token::new(TokenKind::Number("2".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_2_lower_than_1_returns_false() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("2".to_string()), 1),
      Token::new(TokenKind::Less, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_lower_than_1_returns_false() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Less, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_lower_equal_than_2_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::LessEqual, 1),
      Token::new(TokenKind::Number("2".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_1_lower_equal_than_1_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::LessEqual, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_2_lower_equal_than_1_returns_false() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("2".to_string()), 1),
      Token::new(TokenKind::LessEqual, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_greater_than_2_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Greater, 1),
      Token::new(TokenKind::Number("2".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_greater_than_1_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Greater, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_2_greater_than_1_returns_false() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("2".to_string()), 1),
      Token::new(TokenKind::Greater, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_1_greater_equal_than_2_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::GreaterEqual, 1),
      Token::new(TokenKind::Number("2".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_greater_equal_than_1_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::GreaterEqual, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_2_greater_equal_than_1_returns_false() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("2".to_string()), 1),
      Token::new(TokenKind::GreaterEqual, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_1_equal_1_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::EqualEqual, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_1_equal_string_1_returns_false() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::EqualEqual, 1),
      Token::new(TokenKind::String("1".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_holu_not_equal_holu_returns_false() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::String("holu".to_string()), 1),
      Token::new(TokenKind::BangEqual, 1),
      Token::new(TokenKind::String("holu".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_not_equal_2_returns_true() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::BangEqual, 1),
      Token::new(TokenKind::Number("2".to_string()), 1),
    ]);
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_minus_string_returns_error() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::String("1".to_string()), 1),
    ]);

    let err = interpreted.unwrap_err();

    assert_eq!(err, InterpreterError::NotANumber(1, "String".to_string()))
  }

  #[test]
  fn eval_minus_nil_returns_error() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::Nil, 1),
    ]);

    let err = interpreted.unwrap_err();

    assert_eq!(err, InterpreterError::NotANumber(1, "nil".to_string()))
  }

  #[test]
  fn eval_minus_true_returns_error() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::True, 1),
    ]);

    let err = interpreted.unwrap_err();

    assert_eq!(err, InterpreterError::NotANumber(1, "Boolean".to_string()))
  }

  #[test]
  fn eval_aditions_fails_if_one_is_a_bool() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::True, 1),
    ]);

    let err = interpreted.unwrap_err();
    assert_eq!(err, InterpreterError::WrongBinaryOperationType(
      1,
      "+".to_string(),
      "Number".to_string(),
      "Boolean".to_string()
    ));

    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::True, 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
    ]);

    let err = interpreted.unwrap_err();

    assert_eq!(err, InterpreterError::WrongBinaryOperationType(
      1,
      "+".to_string(),
      "Boolean".to_string(),
      "Number".to_string()
    ));
  }

  #[test]
  fn eval_addition_between_number_and_string_fails() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::String("2".to_string()), 1),
    ]);

    let err = interpreted.unwrap_err();
    assert_eq!(err, InterpreterError::WrongBinaryOperationType(
      1,
      "+".to_string(),
      "Number".to_string(),
      "String".to_string()
    ));
  }

  #[test]
  fn eval_comparisson_between_not_numbers_error() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::LessEqual, 1),
      Token::new(TokenKind::String("2".to_string()), 1),
    ]);

    let err = interpreted.unwrap_err();
    assert_eq!(err, InterpreterError::WrongBinaryOperationType(
      1,
      "<=".to_string(),
      "Number".to_string(),
      "String".to_string()
    ));
  }

  #[test]
  fn eval_multiplication_between_number_and_string_error() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Star, 1),
      Token::new(TokenKind::String("2".to_string()), 1),
    ]);

    let err = interpreted.unwrap_err();
    assert_eq!(err, InterpreterError::WrongBinaryOperationType(
      1,
      "*".to_string(),
      "Number".to_string(),
      "String".to_string()
    ));
  }

  #[test]
  fn eval_slash_between_number_and_string_error() {
    let interpreted = interpret_expression(vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Slash, 1),
      Token::new(TokenKind::String("2".to_string()), 1),
    ]);

    let err = interpreted.unwrap_err();
    assert_eq!(err, InterpreterError::WrongBinaryOperationType(
      1,
      "/".to_string(),
      "Number".to_string(),
      "String".to_string()
    ));
  }
}