use std::fmt::{Debug, Formatter};
use crate::interpret::error::RuntimeError;
use crate::interpret::interpreter::Interpreter;
use crate::interpret::value::Value;
use crate::parse::stmt::Stmt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub enum Callable {
  Lox(LoxFn),
  Native(NativeFn),
}

impl Callable {
  pub fn call<W: Write>(
    &self,
    interpreter: &mut Interpreter<W>,
    args: Vec<Value>,
    line: usize,
  ) -> Result<Value, RuntimeError> {
    match self {
      Callable::Lox(fun) => fun.call(interpreter, args, line),
      Callable::Native(fun) => fun.call(interpreter, args, line)
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Callable::Lox(fun) => fun.to_string(),
      Callable::Native(fun) => fun.to_string()
    }
  }
}

type NativeLambda = fn(Vec<Value>) -> Result<Value, RuntimeError>;
#[derive(Clone)]
pub struct NativeFn {
  name: String,
  implementation: NativeLambda,
}

impl NativeFn {
  pub fn new(name: String, implementation: NativeLambda) -> NativeFn {
    NativeFn {
      name,
      implementation
    }
  }

  pub fn call<W: Write>(
    &self,
    _interpreter: &mut Interpreter<W>,
    args: Vec<Value>,
    _line: usize,
  ) -> Result<Value, RuntimeError> {
    (self.implementation)(args)
  }

  pub fn to_string(&self) -> String {
    format!("<nativefn {}>", &self.name)
  }
}

impl Debug for NativeFn {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str("<fn ")?;
    f.write_str(&self.name)?;
    f.write_str(">")?;
    Ok(())
  }
}

impl PartialEq for NativeFn {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoxFn {
  pub name: String,
  params: Vec<String>,
  body: Vec<Stmt>,
  context_id: usize,
}

impl LoxFn {
  pub fn new(name: String, params: Vec<String>, body: Vec<Stmt>, context_id: usize) -> LoxFn {
    LoxFn {
      name,
      params,
      body,
      context_id,
    }
  }

  pub fn call<W: Write>(
    &self,
    interpreter: &mut Interpreter<W>,
    mut args: Vec<Value>,
    line: usize,
  ) -> Result<Value, RuntimeError> {
    if args.len() != self.params.len() {
      return Err(RuntimeError::WrongNumberOfArguments(
        line,
        self.name.clone(),
        self.params.len(),
        args.len(),
      ));
    }

    interpreter.with_branching(self.context_id, move |inter| {
      args
        .drain(..)
        .enumerate()
        .for_each(|(index, value)| inter.define_var(&self.params[index], value));

      let call_res = inter.interpret_stmts(&self.body);
      if let Err(RuntimeError::Return(value)) = call_res {
        return Ok(value)
      } else {
        return call_res
      }
    })
  }

  pub fn to_string(&self) -> String {
    format!("<fn {}>", self.name)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn native_function_can_be_created_and_called() {
    let callable = Callable::Native(NativeFn {
      name: "foo".to_string(),
      implementation: |vec| {
        let res = format!("{:?}", vec);
        Ok(Value::String(res))
      },
    });
    let mut fake_stdout: Vec<u8> = vec![];

    let mut inter = Interpreter::new(&mut fake_stdout);

    let coso = callable.call(&mut inter, vec![Value::Number(1.0)], 10).unwrap();
    assert_eq!(coso, Value::String("[Number(1.0)]".to_string()));
  }
}
