use crate::scan::token::Token;

pub enum Expr {
  LiteralNumber {value: f64},
  LiteralBool {value: bool},
  Binary { left: Box<Expr>, operator: Token, right: Box<Expr> }
}