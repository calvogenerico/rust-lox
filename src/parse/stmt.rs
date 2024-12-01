use crate::parse::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
  Expr(Expr),
  Print(Expr),
  Var(String, Expr, usize),
  ScopeBlock(Vec<Stmt>),
  If {
    condition: Expr,
    then: Box<Stmt>,
    els: Option<Box<Stmt>>,
  },
  While {
    condition: Expr,
    body: Box<Stmt>,
  },
  Function {
    name: String,
    params: Vec<String>,
    body: Vec<Stmt>
  },
}
