use std::io::Write;
use crate::interpret::error::RuntimeError;
use crate::interpret::interpreter::Interpreter;
use crate::interpret::value::Value;
use crate::parse::stmt::Stmt;

#[derive(Debug, PartialEq, Clone)]
pub struct LoxFn {
  pub name: String,
  params: Vec<String>,
  body: Vec<Stmt>,
  context_id: usize,
}

impl LoxFn {
  pub fn new(
    name: String,
    params: Vec<String>,
    body: Vec<Stmt>,
    context_id: usize,
  ) -> LoxFn {
    LoxFn {
      name,
      params,
      body,
      context_id,
    }
  }

  pub fn call <W: Write>(
    &self,
    interpreter: &mut Interpreter<W>,
    mut args: Vec<Value>,
    line: usize
  ) -> Result<Value, RuntimeError> {
    if args.len() != self.params.len() {
      return Err(RuntimeError::WrongNumberOfArguments(line, self.name.clone(), self.params.len(), args.len() ))
    }

    interpreter.with_branching(self.context_id, move |inter| {
      args.drain(..).enumerate().for_each(|(index, value)| {
        inter.define_var(&self.params[index], value)
      });

      inter.interpret_stmts(&self.body)
    })?;

    Ok(Value::Nil)
  }
}