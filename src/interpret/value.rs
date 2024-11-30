
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Number(f64),
  Nil,
  Boolean(bool),
  String(String),
  // Fn(LoxFn)
}

// pub struct LoxFn {
//   context: Environment
// }

impl Value {
  pub fn to_string(&self) -> String {
    match self {
      Value::Number(value) => format!("{value}"),
      Value::Nil => "nil".to_string(),
      Value::Boolean(value) => format!("{value}"),
      Value::String(value) => value.to_string(),
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
