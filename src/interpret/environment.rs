use crate::interpret::error::RuntimeError;
use crate::interpret::value::Value;
use std::collections::HashMap;

pub struct Environment {
  values: HashMap<String, Value>,
  enclosing: Option<Box<Environment>>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      values: HashMap::new(),
      enclosing: None,
    }
  }

  pub fn from(enclosing: Environment) -> Environment {
    Environment {
      values: HashMap::new(),
      enclosing: Some(Box::new(enclosing)),
    }
  }

  pub fn release(self) -> Option<Environment> {
    self.enclosing.map(|e| *e)
  }

  pub fn define(&mut self, key: &str, value: Value) {
    self.values.insert(key.to_string(), value);
  }

  pub fn get(&self, key: &String, line: usize) -> Result<&Value, RuntimeError> {
    self
      .values
      .get(key)
      .or_else(|| {
        self
          .enclosing
          .as_ref()
          .map(|e| e.get(key, line).ok())
          .flatten()
      })
      .ok_or(RuntimeError::UndefinedVariable(line, key.clone()))
  }

  pub fn assign(&mut self, key: &String, value: Value, line: usize) -> Result<(), RuntimeError> {
    if !self.values.contains_key(key) {
      if self.enclosing.is_some() {
        return self.enclosing.as_mut().unwrap().assign(key, value, line);
      }

      return Err(RuntimeError::UndefinedVariable(line, key.clone()));
    }
    self.values.insert(key.clone(), value.clone());
    Ok(())
  }
}
