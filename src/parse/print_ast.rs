use crate::parse::expr::Expr;
use crate::parse::stmt::Stmt;

pub struct PrintAst {}

impl PrintAst {
  pub fn new() -> PrintAst {
    PrintAst {}
  }

  pub fn print_stmts(&self, stmts: &[Stmt]) -> String {
    let mut lines = vec![];
    for stmt in stmts {
      let line = match stmt {
        Stmt::Expr(expr) => self.print_expr(expr),
        Stmt::Print(expr) => format!("(print {})", self.print_expr(expr)),
        Stmt::Var(name, value, _) => format!("(def_var `{}` {})", name, self.print_expr(value))
      };
      lines.push(line);
    }
    lines.join("\n")
  }

  pub fn print_expr(&self, root: &Expr) -> String {
    match root {
      Expr::LiteralNumber { value } => format!("{:?}", value),
      Expr::LiteralString { value } => format!("{value}"),
      Expr::LiteralBool { value } => format!("{value}"),
      Expr::LiteralNil => "nil".to_string(),
      Expr::Unary { operator, right } => format!("({} {})", operator.kind().symbol(), self.print_expr(right)),
      Expr::Binary { left, operator, right } =>
        format!("({} {} {})", operator.kind().symbol(), self.print_expr(left), self.print_expr(right)),
      Expr::Group { expression } => format!("(group {})", self.print_expr(expression)),
      Expr::Variable { name, .. } => format!("`{}`", name),
      Expr::Assign { name, value , .. } => format!("(assign_var `{}` {})", name, self.print_expr(value))
    }
  }
}
