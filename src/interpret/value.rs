use crate::parse::stmt::Stmt;
use std::io::Write;
use crate::interpret::error::RuntimeError;
use crate::interpret::interpreter::Interpreter;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Number(f64),
  Nil,
  Boolean(bool),
  String(String),
  Fn(LoxFn),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoxFn {
  name: String,
  params: Vec<String>,
  body: Vec<Stmt>,
  context_id: usize,
}

impl LoxFn {
  pub fn call <W: Write>(
    &self,
    interpreter: &mut Interpreter<W>,
    mut args: Vec<Value>,
    line: usize
  ) -> Result<Value, RuntimeError> {
    if args.len() != self.params.len() {
      return Err(RuntimeError::WrongNumberOfArguments(line, self.name.clone(), self.params.len(), args.len() ))
    }

    let old_branch = interpreter.current_scope();
    let branch_id = interpreter.branch(self.context_id);
    interpreter.change_scope(branch_id);

    args.drain(..).enumerate().for_each(|(index, value)| {
      interpreter.define_var(&self.params[index], value)
    });

    interpreter.interpret_stmts(&self.body)?;
    interpreter.release_scope(branch_id);
    interpreter.change_scope(old_branch);

    Ok(Value::Nil)
  }
}

impl Value {
  pub fn fun(name: String, params: Vec<String>, body: Vec<Stmt>, context_id: usize) -> Value {
    Value::Fn(LoxFn {
      name,
      params,
      body,
      context_id,
    })
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
