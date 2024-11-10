use crate::scan::token::Token;

pub enum EnumExpr {
  LiteralNumber {value: f64},
  LiteralBool {value: bool},
  Binary { left: Box<EnumExpr>, operator: Token, right: Box<EnumExpr> }
}