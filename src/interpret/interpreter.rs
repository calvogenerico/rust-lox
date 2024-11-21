use crate::interpret::environment::Environment;
use crate::interpret::error::RuntimeError;
use crate::parse::expr::Expr;
use crate::parse::stmt::Stmt;
use crate::scan::token::Token;
use crate::scan::token_kind::TokenKind;

pub struct Interpreter {
  env: Environment
}

#[derive(Debug, PartialEq, Clone)]
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
    Interpreter {
      env: Environment::new()
    }
  }

  pub fn interpret_stmts(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
    for stmt in stmts {
      match stmt {
        Stmt::Expr(expr) => { self.interpret_expr(expr)?; },
        Stmt::Print(expr) => {
          let value = self.interpret_expr(expr)?;
          println!("{}", value.to_string());
        }
        Stmt::Var(name, expr, _) => {
          let value = self.interpret_expr(expr)?;
          self.env.define(name, value)
        }
      }
    }
    Ok(())
  }

  pub fn interpret_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
    match expr {
      Expr::LiteralNumber { value } => Ok(Value::Number(*value)),
      Expr::LiteralNil => Ok(Value::Nil),
      Expr::LiteralBool { value } => Ok(Value::Boolean(*value)),
      Expr::Unary { operator, right } => self.unary(operator, right),
      Expr::LiteralString { value } => Ok(Value::String(value.to_string())),
      Expr::Group { expression } => self.interpret_expr(expression),
      Expr::Binary { left, operator, right } => self.binary(left, operator, right),
      Expr::Variable { name, line } => self.env.get(name, *line).map(|v| v.clone()),
      Expr::Assign { value, name, line } => {
        let value = self.interpret_expr(value)?;
        self.env.assign(name, value.clone(), *line)?;
        Ok(value)
      }
    }
  }

  fn unary(&mut self, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
    let value = self.interpret_expr(right)?;
    Ok(match (value, operator.kind()) {
      (Value::Number(value), TokenKind::Minus) => Value::Number(-value),
      (val, TokenKind::Bang) => Value::Boolean(!self.is_truthy(&val)),
      (value, TokenKind::Minus) => return Err(RuntimeError::NotANumber(operator.line(), value.type_name().to_string())),
      _ => return Err(RuntimeError::InvalidExpression)
    })
  }

  fn binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
    let left = self.interpret_expr(left)?;
    let right = self.interpret_expr(right)?;

    Ok(match (operator.kind(), &left, &right) {
      (TokenKind::EqualEqual, val1, val2) => Value::Boolean(self.are_equal(val1, val2)),
      (TokenKind::BangEqual, val1, val2) => Value::Boolean(!self.are_equal(val1, val2)),
      (TokenKind::Plus, Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
      (TokenKind::Minus, Value::Number(n1), Value::Number(n2)) => Value::Number(n1 - n2),
      (TokenKind::Star, Value::Number(n1), Value::Number(n2)) => Value::Number(n1 * n2),
      (TokenKind::Slash, Value::Number(n1), Value::Number(n2)) => Value::Number(n1 / n2),
      (TokenKind::Less, Value::Number(n1), Value::Number(n2)) => Value::Boolean(n1 < n2),
      (TokenKind::LessEqual, Value::Number(n1), Value::Number(n2)) => Value::Boolean(n1 <= n2),
      (TokenKind::Greater, Value::Number(n1), Value::Number(n2)) => Value::Boolean(n1 > n2),
      (TokenKind::GreaterEqual, Value::Number(n1), Value::Number(n2)) => Value::Boolean(n1 >= n2),
      (TokenKind::Plus, Value::String(s1), Value::String(s2)) => Value::String(format!("{s1}{s2}")),
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
        return Err(RuntimeError::WrongBinaryOperationType(
          operator.line(),
          operator.kind().symbol(),
          val1.type_name().to_string(),
          val2.type_name().to_string()
        )),

      _ => return Err(RuntimeError::InvalidExpression)
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
  use std::io::Cursor;
  use crate::parse::parser::LoxParser;
  use crate::parse::stmt::Stmt;
  use crate::scan::scanner::Scanner;
  use super::*;
  fn interpret_expression(src: &str) -> Result<Value, RuntimeError> {
    let mut cursor = Cursor::new(src);
    let scanner = Scanner::new(&mut cursor);
    let tokens = scanner.scan_tokens().unwrap();
    let stmts = LoxParser::new(tokens).parse().unwrap();
    let mut interpreter = Interpreter::new();
    match stmts.first().unwrap() {
      Stmt::Expr(expr) => { interpreter.interpret_expr(expr) }
      _ => panic!("should be an expr")
    }
  }

  #[test]
  fn eval_number_1() {
    let interpreted = interpret_expression("1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(1f64))
  }

  #[test]
  fn eval_number_2() {
    let interpreted = interpret_expression("2;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(2f64))
  }

  #[test]
  fn eval_nil() {
    let interpreted = interpret_expression("nil;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Nil)
  }

  #[test]
  fn eval_true() {
    let interpreted = interpret_expression("true;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_false() {
    let interpreted = interpret_expression("false;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_minus_one() {
    let interpreted = interpret_expression("-1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(-1.0))
  }

  #[test]
  fn eval_minus_string() {
    let interpreted = interpret_expression("\"foo\";");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::String("foo".to_string()))
  }

  #[test]
  fn eval_not_true_returns_false() {
    let interpreted = interpret_expression("!true;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_not_false_returns_true() {
    let interpreted = interpret_expression("!false;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_not_a_positive_number_returns_false() {
    // any number is truthy
    let interpreted = interpret_expression("!1.0;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }
  //
  #[test]
  fn eval_not_zero_returns_false() {
    // any number is truthy
    let interpreted = interpret_expression("!0;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_not_nil_returns_true() {
    let interpreted = interpret_expression("!nil;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_a_group_returns_inner_expr() {
    let interpreted = interpret_expression("(1);");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(1.0))
  }

  #[test]
  fn eval_an_addition_returns_the_result() {
    let interpreted = interpret_expression("1 + 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(3.0))
  }

  #[test]
  fn eval_a_subtraction_returns_the_result() {
    let interpreted = interpret_expression("5 - 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(4.0))
  }

  #[test]
  fn eval_a_subtraction_can_return_negative_number() {
    let interpreted = interpret_expression("5 - 12;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(-7.0))
  }

  #[test]
  fn eval_a_plus_between_strings_concatenate_strings() {
    let interpreted = interpret_expression("\"foo\" + \"bar\";");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::String("foobar".to_string()))
  }

  #[test]
  fn eval_a_star_between_numbers_multiplies() {
    let interpreted = interpret_expression("7 * 3;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(21.0))
  }

  #[test]
  fn eval_a_slash_between_numbers_divides() {
    let interpreted = interpret_expression("1 / 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Number(0.5))
  }

  #[test]
  fn eval_1_lower_than_2_returns_true() {
    let interpreted = interpret_expression("1 < 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_2_lower_than_1_returns_false() {
    let interpreted = interpret_expression("2 < 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_lower_than_1_returns_false() {
    let interpreted = interpret_expression("1 < 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_lower_equal_than_2_returns_true() {
    let interpreted = interpret_expression("1 <= 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_1_lower_equal_than_1_returns_true() {
    let interpreted = interpret_expression("1 <= 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_2_lower_equal_than_1_returns_false() {
    let interpreted = interpret_expression("2 <= 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_greater_than_2_returns_true() {
    let interpreted = interpret_expression("1 >= 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_greater_than_1_returns_true() {
    let interpreted = interpret_expression("1 > 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_2_greater_than_1_returns_false() {
    let interpreted = interpret_expression("2 > 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_1_greater_equal_than_2_returns_true() {
    let interpreted = interpret_expression("1 >= 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_greater_equal_than_1_returns_true() {
    let interpreted = interpret_expression("1 >= 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_2_greater_equal_than_1_returns_false() {
    let interpreted = interpret_expression("2 >= 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_1_equal_1_returns_true() {
    let interpreted = interpret_expression("1 == 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_1_equal_string_1_returns_false() {
    let interpreted = interpret_expression("1 == \"1\";");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }
  //
  #[test]
  fn eval_holu_not_equal_holu_returns_false() {
    let interpreted = interpret_expression("\"holu\" != \"holu\";");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(false))
  }

  #[test]
  fn eval_1_not_equal_2_returns_true() {
    let interpreted = interpret_expression("1 != 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, Value::Boolean(true))
  }

  #[test]
  fn eval_minus_string_returns_error() {
    let interpreted = interpret_expression("-\"foo\";");

    let err = interpreted.unwrap_err();

    assert_eq!(err, RuntimeError::NotANumber(1, "String".to_string()))
  }

  #[test]
  fn eval_minus_nil_returns_error() {
    let interpreted = interpret_expression("-nil;");

    let err = interpreted.unwrap_err();

    assert_eq!(err, RuntimeError::NotANumber(1, "nil".to_string()))
  }

  #[test]
  fn eval_minus_true_returns_error() {
    let interpreted = interpret_expression("-true;");

    let err = interpreted.unwrap_err();

    assert_eq!(err, RuntimeError::NotANumber(1, "Boolean".to_string()))
  }

  #[test]
  fn eval_aditions_fails_if_one_is_a_bool() {
    let interpreted = interpret_expression("1 + true;");

    let err = interpreted.unwrap_err();
    assert_eq!(err, RuntimeError::WrongBinaryOperationType(
      1,
      "+".to_string(),
      "Number".to_string(),
      "Boolean".to_string()
    ));

    let interpreted = interpret_expression("true + 1;");

    let err = interpreted.unwrap_err();

    assert_eq!(err, RuntimeError::WrongBinaryOperationType(
      1,
      "+".to_string(),
      "Boolean".to_string(),
      "Number".to_string()
    ));
  }

  #[test]
  fn eval_addition_between_number_and_string_fails() {
    let interpreted = interpret_expression("1 + \"2\";");

    let err = interpreted.unwrap_err();
    assert_eq!(err, RuntimeError::WrongBinaryOperationType(
      1,
      "+".to_string(),
      "Number".to_string(),
      "String".to_string()
    ));
  }

  #[test]
  fn eval_comparisson_between_not_numbers_error() {
    let interpreted = interpret_expression("1 <= \"2\";");

    let err = interpreted.unwrap_err();
    assert_eq!(err, RuntimeError::WrongBinaryOperationType(
      1,
      "<=".to_string(),
      "Number".to_string(),
      "String".to_string()
    ));
  }

  #[test]
  fn eval_multiplication_between_number_and_string_error() {
    let interpreted = interpret_expression("1 * \"2\";");
  
    let err = interpreted.unwrap_err();
    assert_eq!(err, RuntimeError::WrongBinaryOperationType(
      1,
      "*".to_string(),
      "Number".to_string(),
      "String".to_string()
    ));
  }
  
  #[test]
  fn eval_slash_between_number_and_string_error() {
    let interpreted = interpret_expression("1 / \"2\";");
  
    let err = interpreted.unwrap_err();
    assert_eq!(err, RuntimeError::WrongBinaryOperationType(
      1,
      "/".to_string(),
      "Number".to_string(),
      "String".to_string()
    ));
  }
}