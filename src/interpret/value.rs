use crate::parse::stmt::Stmt;
use crate::interpret::lox_fn::{Callable, LoxFn};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Number(f64),
  Nil,
  Boolean(bool),
  String(String),
  Callable(Callable),
}


impl Value {
  pub fn fun(name: String, params: Vec<String>, body: Vec<Stmt>, context_id: usize) -> Value {
    Value::Callable(Callable::Lox(LoxFn::new(name, params, body, context_id)))
  }

  pub fn to_string(&self) -> String {
    match self {
      Value::Number(value) => format!("{value}"),
      Value::Nil => "nil".to_string(),
      Value::Boolean(value) => format!("{value}"),
      Value::String(value) => value.to_string(),
      Value::Callable(fun) => fun.to_string(),
    }
  }

  pub fn type_name(&self) -> &'static str {
    match self {
      Value::Number(_) => "Number",
      Value::Nil => "nil",
      Value::Boolean(_) => "Boolean",
      Value::String(_) => "String",
      Value::Callable(_) => "function",
    }
  }
}
