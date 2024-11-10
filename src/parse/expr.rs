use crate::parse::visitor::Visitor;

pub trait Expr {
  fn accept<T>(&self, visitor: impl Visitor<T>) -> T;
}

impl Expr for LiteralExpr {
  fn accept<T>(&self, visitor: impl Visitor<T>) -> T {
    visitor.visit_literal(self)
  }
}

pub struct LiteralExpr {
  pub value: f64,
}

pub struct BooleanExpr {
  pub value: bool,
}

impl BooleanExpr {
  pub fn new(value: bool) -> BooleanExpr { BooleanExpr { value } }
}

impl Expr for BooleanExpr {
  fn accept<T>(&self, visitor: impl Visitor<T>) -> T {
    visitor.visit_boolean(self)
  }
}