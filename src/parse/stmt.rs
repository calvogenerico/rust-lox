use crate::parse::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
  Expr(Expr),
  Print(Expr),
  Var(String, Expr, usize),
  ScopeBlock(Vec<Stmt>),
  If{ condition: Expr, then: Box<Stmt>, els: Option<Box<Stmt>> }
}
