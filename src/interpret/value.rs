use crate::parse::stmt::Stmt;
use crate::interpret::lox_fn::LoxFn;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Number(f64),
  Nil,
  Boolean(bool),
  String(String),
  Fn(LoxFn),
}


impl Value {
  pub fn fun(name: String, params: Vec<String>, body: Vec<Stmt>, context_id: usize) -> Value {
    Value::Fn(LoxFn::new(name, params, body, context_id))
  }

  pub fn to_string(&self) -> String {
    match self {
      Value::Number(value) => format!("{value}"),
      Value::Nil => "nil".to_string(),
      Value::Boolean(value) => format!("{value}"),
      Value::String(value) => value.to_string(),
      Value::Fn(LoxFn { name, .. }) => format!("<fn {}>", name),
    }
  }

  pub fn type_name(&self) -> &'static str {
    match self {
      Value::Number(_) => "Number",
      Value::Nil => "nil",
      Value::Boolean(_) => "Boolean",
      Value::String(_) => "String",
      Value::Fn(_) => "function",
    }
  }
}
