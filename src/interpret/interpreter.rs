use crate::interpret::branching_scope::BranchingScope;
use crate::interpret::error::RuntimeError;
use crate::interpret::lox_fn::{Callable, NativeFn};
use crate::interpret::value::Value;
use crate::parse::expr::Expr;
use crate::parse::stmt::Stmt;
use crate::scan::token::Token;
use crate::scan::token_kind::TokenKind;
use std::io::Write;
use std::slice;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Interpreter<W: Write> {
  env: BranchingScope,
  // global_id: usize,
  current_id: usize,
  stdout: W,
}

impl<W: Write> Interpreter<W> {
  pub fn new(writer: W) -> Self {
    let mut env = BranchingScope::empty();
    let global_id = env.branch(0);

    env.define(
      global_id,
      "clock",
      Value::Callable(Callable::Native(NativeFn::new(
        "clock".to_string(),
        |_a| {
          let start = SystemTime::now();
          let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
          Ok(Value::Number(since_the_epoch.as_secs() as f64))
        }
      ))),
    );

    Interpreter {
      env,
      // global_id,
      current_id: global_id,
      stdout: writer,
    }
  }

  pub fn interpret_stmts(&mut self, stmts: &[Stmt]) -> Result<Value, RuntimeError> {
    for stmt in stmts {
      self.interpret_stmt(stmt)?;
    }
    Ok(Value::Nil)
  }

  pub fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
    match stmt {
      Stmt::Expr(expr) => {
        self.interpret_expr(expr)?;
      }
      Stmt::Print(expr) => {
        let value = self.interpret_expr(expr)?;
        writeln!(self.stdout, "{}", &value.to_string())
          .map_err(|_| RuntimeError::CannotWriteToStdout)?;
      }
      Stmt::Var(name, expr, _) => {
        let value = self.interpret_expr(expr)?;
        self.env.define(self.current_id, name, value);
      }
      Stmt::ScopeBlock(stmts) => {
        self.interpret_scope_block_stmt(stmts)?;
      }
      Stmt::If {
        condition,
        then,
        els,
      } => {
        self.interpret_if(condition, then, els.as_ref().map(|b| &**b))?;
      }
      Stmt::While { condition, body } => {
        self.interpret_while(condition, body)?;
      }
      Stmt::Function { name, params, body } => {
        self.interpret_function_definition(name, params, body)?;
      }
      Stmt::Return(expr) => self.interpret_return(expr)?,
    }
    Ok(())
  }

  fn interpret_scope_block_stmt(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
    let new_scope = self.env.branch(self.current_id);
    self.current_id = new_scope;
    self.interpret_stmts(stmts)?;
    self.current_id = self.env.release(self.current_id);
    Ok(())
  }

  // fn branch(&self) -> Interpreter<W> {
  //   Interpreter {
  //     env: self.branch(),
  //     stdout: &self.stdout
  //   }
  // }

  fn interpret_if(
    &mut self,
    condition: &Expr,
    then: &Stmt,
    els: Option<&Stmt>,
  ) -> Result<(), RuntimeError> {
    let value = self.interpret_expr(condition)?;
    if self.is_truthy(&value) {
      self.interpret_stmts(slice::from_ref(then))?;
    } else {
      els
        .map(|stmt| self.interpret_stmts(slice::from_ref(stmt)))
        .transpose()?;
    }
    Ok(())
  }

  fn interpret_while(&mut self, condition: &Expr, body: &Stmt) -> Result<(), RuntimeError> {
    while self.interpret_expr(condition).map(|v| self.is_truthy(&v))? {
      self.interpret_stmt(body)?;
    }
    Ok(())
  }

  fn interpret_function_definition(
    &mut self,
    name: &str,
    params: &[String],
    body: &[Stmt],
  ) -> Result<Value, RuntimeError> {
    let new_branch = self.env.branch(self.current_id);
    let fun = Value::fun(
      name.to_string(),
      params.to_vec(),
      body.to_vec(),
      new_branch,
    );



    self.env.define(self.current_id, name, fun);
    Ok(Value::Nil)
  }

  fn interpret_return(&mut self, expr: &Expr) -> Result<(), RuntimeError> {
    let value = self.interpret_expr(expr)?;
    Err(RuntimeError::Return(value))
  }

  pub fn interpret_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
    match expr {
      Expr::LiteralNumber { value } => Ok(Value::Number(*value)),
      Expr::LiteralNil => Ok(Value::Nil),
      Expr::LiteralBool { value } => Ok(Value::Boolean(*value)),
      Expr::Unary { operator, right } => self.unary(operator, right),
      Expr::LiteralString { value } => Ok(Value::String(value.to_string())),
      Expr::Group { expression } => self.interpret_expr(expression),
      Expr::Binary {
        left,
        operator,
        right,
      } => self.binary(left, operator, right),
      Expr::Variable { name, line } => self
        .env
        .get(self.current_id, name)
        .ok_or(RuntimeError::UndefinedVariable(*line, name.to_string()))
        .map(|v| v.clone()),
      Expr::Assign { value, name, line } => {
        let value = self.interpret_expr(value)?;
        self
          .env
          .assign(self.current_id, name, value.clone())
          .ok_or(RuntimeError::UndefinedVariable(*line, name.to_string()))?;
        Ok(value)
      }
      Expr::Logical {
        left,
        operator,
        right,
      } => self.logical(left, operator, right),
      Expr::Call { callee, args, line } => self.interpret_call(callee, args, *line),
    }
  }

  fn unary(&mut self, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
    let value = self.interpret_expr(right)?;
    Ok(match (value, operator.kind()) {
      (Value::Number(value), TokenKind::Minus) => Value::Number(-value),
      (val, TokenKind::Bang) => Value::Boolean(!self.is_truthy(&val)),
      (value, TokenKind::Minus) => {
        return Err(RuntimeError::NotANumber(
          operator.line(),
          value.type_name().to_string(),
        ))
      }
      _ => return Err(RuntimeError::InvalidExpression),
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
      (TokenKind::Slash, Value::Number(n1), Value::Number(n2)) => {
        if *n2 == 0.0 {
          return Err(RuntimeError::ZeroDivision(operator.line()));
        }
        Value::Number(n1 / n2)
      }
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
        val2,
      ) => {
        return Err(RuntimeError::WrongBinaryOperationType(
          operator.line(),
          operator.kind().symbol(),
          val1.type_name().to_string(),
          val2.type_name().to_string(),
        ))
      }

      _ => return Err(RuntimeError::InvalidExpression),
    })
  }

  fn logical(
    &mut self,
    left: &Expr,
    operator: &Token,
    right: &Expr,
  ) -> Result<Value, RuntimeError> {
    let left = self.interpret_expr(left)?;

    match operator.kind() {
      TokenKind::And => {
        let x = self.is_truthy(&left);
        if x {
          self.interpret_expr(right)
        } else {
          Ok(left)
        }
      }
      TokenKind::Or => {
        if self.is_truthy(&left) {
          Ok(left)
        } else {
          self.interpret_expr(right)
        }
      }
      _ => Err(RuntimeError::InvalidExpression),
    }
  }

  fn interpret_call(
    &mut self,
    callee: &Expr,
    args: &[Expr],
    line: usize,
  ) -> Result<Value, RuntimeError> {
    let callee_value = self.interpret_expr(callee)?;
    let arg_values = args
      .iter()
      .map(|arg| self.interpret_expr(arg))
      .collect::<Result<Vec<Value>, RuntimeError>>()?;
    let lox_fn = if let Value::Callable(lox_fn) = callee_value {
      lox_fn
    } else {
      return Err(RuntimeError::NotAFunction(
        line,
        callee_value.type_name().to_string(),
      ));
    };

    lox_fn.call(self, arg_values, line)
  }

  fn are_equal(&self, val1: &Value, val2: &Value) -> bool {
    match (val1, val2) {
      (Value::Number(n1), Value::Number(n2)) => n1 == n2,
      (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
      (Value::String(s1), Value::String(s2)) => s1 == s2,
      (val1, val2) => val1 == val2,
    }
  }

  fn is_truthy(&self, value: &Value) -> bool {
    !self.is_falsey(value)
  }

  fn is_falsey(&self, value: &Value) -> bool {
    match value {
      Value::Nil => true,
      Value::Boolean(false) => true,
      _ => false,
    }
  }

  pub fn with_branching(
    &mut self,
    base_branch: usize,
    action: impl FnOnce(&mut Interpreter<W>) -> Result<Value, RuntimeError>,
  ) -> Result<Value, RuntimeError> {
    let old = self.current_id;
    let new_branch = self.env.branch(base_branch);
    self.current_id = new_branch;
    let res = action(self);
    self.env.release(new_branch);
    self.current_id = old;
    res
  }

  pub fn define_var(&mut self, name: &str, value: Value) {
    self.env.define(self.current_id, name, value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parse::parser::LoxParser;
  use crate::scan::scanner::Scanner;
  use std::io::Cursor;

  fn interpret_program(src: &str) -> Result<String, RuntimeError> {
    let mut cursor = Cursor::new(src);
    let scanner = Scanner::new(&mut cursor);
    let tokens = scanner.scan_tokens().0;
    let stmts = LoxParser::new(tokens).parse().unwrap();
    let mut fake_stdout: Vec<u8> = vec![];
    let mut interpreter = Interpreter::new(&mut fake_stdout);
    interpreter.interpret_stmts(&stmts)?;
    Ok(String::from_utf8(fake_stdout).unwrap())
  }

  #[test]
  fn eval_number_1() {
    let interpreted = interpret_program("print 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "1\n");
  }

  #[test]
  fn eval_number_2() {
    let interpreted = interpret_program("print 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, "2\n");
  }

  #[test]
  fn eval_nil() {
    let interpreted = interpret_program("print nil;");
    let res = interpreted.unwrap();

    assert_eq!(res, "nil\n")
  }

  #[test]
  fn eval_true() {
    let interpreted = interpret_program("print true;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_false() {
    let interpreted = interpret_program("print false;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_minus_one() {
    let interpreted = interpret_program("print -1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "-1\n")
  }

  #[test]
  fn eval_minus_string() {
    let interpreted = interpret_program("print \"foo\";");
    let res = interpreted.unwrap();

    assert_eq!(res, "foo\n")
  }

  #[test]
  fn eval_not_true_returns_false() {
    let interpreted = interpret_program("print !true;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_not_false_returns_true() {
    let interpreted = interpret_program("print !false;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_not_a_positive_number_returns_false() {
    // any number is truthy
    let interpreted = interpret_program("print !1.0;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }
  //
  #[test]
  fn eval_not_zero_returns_false() {
    // any number is truthy
    let interpreted = interpret_program("print !0;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_not_nil_returns_true() {
    let interpreted = interpret_program("print !nil;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_a_group_returns_inner_expr() {
    let interpreted = interpret_program("print (1);");
    let res = interpreted.unwrap();

    assert_eq!(res, "1\n")
  }

  #[test]
  fn eval_an_addition_returns_the_result() {
    let interpreted = interpret_program("print 1 + 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, "3\n")
  }

  #[test]
  fn eval_a_subtraction_returns_the_result() {
    let interpreted = interpret_program("print 5 - 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "4\n")
  }

  #[test]
  fn eval_a_subtraction_can_return_negative_number() {
    let interpreted = interpret_program("print 5 - 12;");
    let res = interpreted.unwrap();

    assert_eq!(res, "-7\n")
  }

  #[test]
  fn eval_a_plus_between_strings_concatenate_strings() {
    let interpreted = interpret_program("print \"foo\" + \"bar\";");
    let res = interpreted.unwrap();

    assert_eq!(res, "foobar\n")
  }

  #[test]
  fn eval_a_star_between_numbers_multiplies() {
    let interpreted = interpret_program("print 7 * 3;");
    let res = interpreted.unwrap();

    assert_eq!(res, "21\n")
  }

  #[test]
  fn eval_a_slash_between_numbers_divides() {
    let interpreted = interpret_program("print 1 / 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, "0.5\n")
  }

  #[test]
  fn eval_1_lower_than_2_returns_true() {
    let interpreted = interpret_program("print 1 < 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_2_lower_than_1_returns_false() {
    let interpreted = interpret_program("print 2 < 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_1_lower_than_1_returns_false() {
    let interpreted = interpret_program("print 1 < 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_1_lower_equal_than_2_returns_true() {
    let interpreted = interpret_program("print 1 <= 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_1_lower_equal_than_1_returns_true() {
    let interpreted = interpret_program("print 1 <= 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_2_lower_equal_than_1_returns_false() {
    let interpreted = interpret_program("print 2 <= 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_1_greater_than_2_returns_true() {
    let interpreted = interpret_program("print 1 >= 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_1_greater_than_1_returns_true() {
    let interpreted = interpret_program("print 1 > 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_2_greater_than_1_returns_false() {
    let interpreted = interpret_program("print 2 > 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_1_greater_equal_than_2_returns_true() {
    let interpreted = interpret_program("print 1 >= 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_1_greater_equal_than_1_returns_true() {
    let interpreted = interpret_program("print 1 >= 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_2_greater_equal_than_1_returns_false() {
    let interpreted = interpret_program("print 2 >= 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_1_equal_1_returns_true() {
    let interpreted = interpret_program("print 1 == 1;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_1_equal_string_1_returns_false() {
    let interpreted = interpret_program("print 1 == \"1\";");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_holu_not_equal_holu_returns_false() {
    let interpreted = interpret_program("print \"holu\" != \"holu\";");
    let res = interpreted.unwrap();

    assert_eq!(res, "false\n")
  }

  #[test]
  fn eval_1_not_equal_2_returns_true() {
    let interpreted = interpret_program("print 1 != 2;");
    let res = interpreted.unwrap();

    assert_eq!(res, "true\n")
  }

  #[test]
  fn eval_minus_string_returns_error() {
    let interpreted = interpret_program("-\"foo\";");

    let err = interpreted.unwrap_err();

    assert_eq!(err, RuntimeError::NotANumber(1, "String".to_string()))
  }

  #[test]
  fn eval_minus_nil_returns_error() {
    let interpreted = interpret_program("-nil;");

    let err = interpreted.unwrap_err();

    assert_eq!(err, RuntimeError::NotANumber(1, "nil".to_string()))
  }

  #[test]
  fn eval_minus_true_returns_error() {
    let interpreted = interpret_program("-true;");

    let err = interpreted.unwrap_err();

    assert_eq!(err, RuntimeError::NotANumber(1, "Boolean".to_string()))
  }

  #[test]
  fn eval_aditions_fails_if_one_is_a_bool() {
    let interpreted = interpret_program("1 + true;");

    let err = interpreted.unwrap_err();
    assert_eq!(
      err,
      RuntimeError::WrongBinaryOperationType(
        1,
        "+".to_string(),
        "Number".to_string(),
        "Boolean".to_string()
      )
    );

    let interpreted = interpret_program("true + 1;");

    let err = interpreted.unwrap_err();

    assert_eq!(
      err,
      RuntimeError::WrongBinaryOperationType(
        1,
        "+".to_string(),
        "Boolean".to_string(),
        "Number".to_string()
      )
    );
  }

  #[test]
  fn eval_addition_between_number_and_string_fails() {
    let interpreted = interpret_program("1 + \"2\";");

    let err = interpreted.unwrap_err();
    assert_eq!(
      err,
      RuntimeError::WrongBinaryOperationType(
        1,
        "+".to_string(),
        "Number".to_string(),
        "String".to_string()
      )
    );
  }

  #[test]
  fn eval_comparisson_between_not_numbers_error() {
    let interpreted = interpret_program("1 <= \"2\";");

    let err = interpreted.unwrap_err();
    assert_eq!(
      err,
      RuntimeError::WrongBinaryOperationType(
        1,
        "<=".to_string(),
        "Number".to_string(),
        "String".to_string()
      )
    );
  }

  #[test]
  fn eval_multiplication_between_number_and_string_error() {
    let interpreted = interpret_program("1 * \"2\";");

    let err = interpreted.unwrap_err();
    assert_eq!(
      err,
      RuntimeError::WrongBinaryOperationType(
        1,
        "*".to_string(),
        "Number".to_string(),
        "String".to_string()
      )
    );
  }

  #[test]
  fn eval_slash_between_number_and_string_error() {
    let interpreted = interpret_program("1 / \"2\";");

    let err = interpreted.unwrap_err();
    assert_eq!(
      err,
      RuntimeError::WrongBinaryOperationType(
        1,
        "/".to_string(),
        "Number".to_string(),
        "String".to_string()
      )
    );
  }

  #[test]
  fn assign_variable_and_return_variables_returns_the_value_of_the_variable() {
    let interpreted = interpret_program("var a = \"success\"; print a;");

    let res = interpreted.unwrap();
    assert_eq!(res, "success\n");
  }

  #[test]
  fn re_assign_variable_saves_last_value() {
    let interpreted = interpret_program("var a = 1; a = 2; print a;");

    let res = interpreted.unwrap();
    assert_eq!(res, "2\n");
  }

  #[test]
  fn re_define_variable_is_valid() {
    let interpreted = interpret_program("var a = 1; var a = 2; print a;");

    let res = interpreted.unwrap();
    assert_eq!(res, "2\n");
  }

  #[test]
  fn access_undefined_variable_is_an_error() {
    let interpreted = interpret_program("a + 1;");

    let res = interpreted.unwrap_err();
    assert_eq!(res, RuntimeError::UndefinedVariable(1, "a".to_string()));
  }

  #[test]
  fn scopes_can_be_executed_ok() {
    let interpreted = interpret_program("var a; { a = 1;} print a;");

    let res = interpreted.unwrap();
    assert_eq!(res, "1\n");
  }

  #[test]
  fn redefine_variable_inside_scope_do_not_change_outside_scope() {
    let interpreted = interpret_program("var a = 1; { var a = 2;} print a;");

    let res = interpreted.unwrap();
    assert_eq!(res, "1\n");
  }

  #[test]
  fn exec_if_when_condition_is_true() {
    let interpreted = interpret_program("var a; if (true) { a = 10; } print a");

    let res = interpreted.unwrap();
    assert_eq!(res, "10\n");
  }

  #[test]
  fn exec_if_when_condition_is_false_and_no_else() {
    let interpreted = interpret_program("var a; if (false) { a = 10; } print a");

    let res = interpreted.unwrap();
    assert_eq!(res, "nil\n");
  }

  #[test]
  fn exec_if_when_condition_is_false_and_else_clause() {
    let interpreted = interpret_program("var a; if (false) { a = 10; } else { a = -11; } print a");

    let res = interpreted.unwrap();
    assert_eq!(res, "-11\n");
  }

  #[test]
  fn can_exec_a_while_stmt() {
    let interpreted = interpret_program("var a = 0; while (a < 10) a = a + 1; print a;");

    let res = interpreted.unwrap();
    assert_eq!(res, "10\n");
  }

  #[test]
  fn execution_of_and() {
    let res1 = interpret_program("print true and 1").unwrap();
    let res2 = interpret_program("print true and false").unwrap();
    let res3 = interpret_program("print true and nil").unwrap();
    let res4 = interpret_program("print true and \"hello\"").unwrap();
    let res5 = interpret_program("print false and \"hello\"").unwrap();
    let res6 = interpret_program("print nil and \"hello\"").unwrap();

    assert_eq!(res1, "1\n");
    assert_eq!(res2, "false\n");
    assert_eq!(res3, "nil\n");
    assert_eq!(res4, "hello\n");
    assert_eq!(res5, "false\n");
    assert_eq!(res6, "nil\n");
  }

  #[test]
  fn shortcircuit_for_and() {
    let res = interpret_program("print false and 1/0").unwrap();
    assert_eq!(res, "false\n");
  }

  #[test]
  fn execution_of_or() {
    let res1 = interpret_program("print true or false").unwrap();
    let res2 = interpret_program("print false or false").unwrap();
    let res3 = interpret_program("print \"holu\" or false").unwrap();
    let res4 = interpret_program("print nil or 10").unwrap();
    assert_eq!(res1, "true\n");
    assert_eq!(res2, "false\n");
    assert_eq!(res3, "holu\n");
    assert_eq!(res4, "10\n");
  }

  #[test]
  fn short_circuit_for_or() {
    let res = interpret_program("print 1 or 1/0").unwrap();
    assert_eq!(res, "1\n");
  }

  #[test]
  fn define_a_function() {
    let res = interpret_program("fun foo() {}").unwrap();
    assert_eq!(res, "");
  }

  #[test]
  fn define_a_function_and_print_it() {
    let res = interpret_program("fun foo() {}\n print foo;").unwrap();
    assert_eq!(res, "<fn foo>\n");
  }

  #[test]
  fn define_a_function_and_call_it() {
    let res = interpret_program("fun foo() { print \"foo\"; }\n foo();").unwrap();
    assert_eq!(res, "foo\n");
  }

  #[test]
  fn define_a_function_with_args_and_call_it() {
    let res = interpret_program("fun foo(a, b) { print a + b; }\n foo(1, 5);").unwrap();
    assert_eq!(res, "6\n");
  }

  #[test]
  fn define_a_function_that_captures_scope_and_call_it() {
    let res =
      interpret_program("var outside = 10; fun foo(a) { print outside + a; }\n foo(2);").unwrap();
    assert_eq!(res, "12\n");
  }

  #[test]
  fn after_a_function_call_the_scope_is_still_ok() {
    let res =
      interpret_program("var outside = 10; fun foo(a) { outside + a; }\n foo(2); print outside ")
        .unwrap();
    assert_eq!(res, "10\n");
  }

  #[test]
  fn clock_is_defined_globally() {
    let res = interpret_program("print clock()").unwrap();
    let parsed = res.trim().parse::<u64>();
    assert!(parsed.is_ok());
  }

  #[test]
  fn function_that_returns() {
    let res = interpret_program("fun foo() { return 10; } print foo();").unwrap();
    assert_eq!(res, "10\n");
  }

  #[test]
  fn coso() {
    let src = "
      fun dameFun() {
          var a = 0;
          fun funInterna() {
            a = a + 1;
            return a;
          }
          return funInterna;
      }

      var devuelta = dameFun();

      print devuelta();
    ";

    let res = interpret_program(src).unwrap();
    assert_eq!(res, "1\n")
  }
}
