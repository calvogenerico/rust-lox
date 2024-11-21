use std::collections::HashMap;
use crate::interpret::error::RuntimeError;
use crate::interpret::interpreter::Value;

pub struct Environment {
  values: HashMap<String, Value>
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      values: HashMap::new()
    }
  }

  pub fn define(&mut self, key: &str, value: Value) {
    self.values.insert(key.to_string(), value);
  }

  pub fn get(&self, key: &String, line: usize) -> Result<&Value, RuntimeError>  {
    self.values.get(key).ok_or(RuntimeError::UndefinedVariable(line, key.clone()))
  }

  pub fn assign(&mut self, key: &String, value: Value, line: usize) -> Result<(), RuntimeError> {
    if !self.values.contains_key(key) {
      return Err(RuntimeError::UndefinedVariable(line, key.clone()))
    }
    self.values.insert(key.clone(), value.clone());
    Ok(())
  }
}