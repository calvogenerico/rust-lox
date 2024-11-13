use crate::scan::token::Token;

pub enum Expr {
  LiteralNumber {value: f64},
  LiteralBool {value: bool},
  LiteralString { value: String },
  Binary { left: Box<Expr>, operator: Token, right: Box<Expr> },
  Unary { operator: Token, right: Box<Expr> },
  Group { expression: Box<Expr> },
  LiteralNil,
}