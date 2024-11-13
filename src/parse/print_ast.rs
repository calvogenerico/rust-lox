use crate::parse::enum_expr::Expr;

pub struct PrintAst {}

impl PrintAst {
  pub fn new() -> PrintAst {
    PrintAst {}
  }

  pub fn print(&self, root: &Expr) -> String {
    match root {
      Expr::LiteralNumber { value } => format!("{:?}", value),
      Expr::LiteralString { value } => format!("{value}"),
      Expr::LiteralBool { value } => format!("{value}"),
      Expr::LiteralNil => "nil".to_string(),
      Expr::Unary { operator, right } => format!("({}{})", operator.kind().symbol(), self.print(right)),
      Expr::Binary { left, operator, right } =>
        format!("({} {} {})", operator.kind().symbol(), self.print(left), self.print(right)),
      Expr::Group { expression } => format!("(group {})", self.print(expression))
    }
  }
}
