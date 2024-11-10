use crate::parse::enum_expr::EnumExpr;
use crate::parse::expr::{BooleanExpr, LiteralExpr};

pub trait Visitor<T> {
  fn visit_literal(&self, expr: &LiteralExpr) -> T;
  fn visit_boolean(&self, expr: &BooleanExpr) -> T;
}

pub struct PrintAst {}

impl PrintAst {
  pub fn print(&self, root: &EnumExpr) -> String {
    match root {
      EnumExpr::LiteralNumber { value } => format!("{value}"),
      EnumExpr::LiteralBool { value } => format!("{value}"),
      EnumExpr::Binary { left, operator, right } =>
        format!("({} {} {})", operator.kind().symbol(), self.print(left), self.print(right))
    }
  }
}

impl Visitor<String> for PrintAst {
  fn visit_literal(&self, expr: &LiteralExpr) -> String {
    format!("{}", expr.value)
  }

  fn visit_boolean(&self, expr: &BooleanExpr) -> String {
    format!("{}", expr.value)
  }
}