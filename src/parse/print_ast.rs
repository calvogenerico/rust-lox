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
      let line = self.print_stmt(stmt);
      lines.push(line);
    }
    lines.join(" ")
  }

  fn print_stmt(&self, stmt: &Stmt) -> String {
    match stmt {
      Stmt::Expr(expr) => self.print_expr(expr),
      Stmt::Print(expr) => format!("(print {})", self.print_expr(expr)),
      Stmt::Var(name, value, _) => format!("(def_var `{}` {})", name, self.print_expr(value)),
      Stmt::ScopeBlock(stmts) => format!("(block_scope {})", self.print_stmts(stmts)),
      Stmt::If {
        condition,
        then,
        els,
      } => format!(
        "(if {} {} {})",
        self.print_expr(condition),
        self.print_stmt(then),
        els
          .as_ref()
          .map(|stmt| self.print_stmt(&stmt))
          .unwrap_or("".to_string()),
      ),
      Stmt::While { condition, body } => format!(
        "(while {} {})",
        self.print_expr(condition),
        self.print_stmt(body)
      ),
      Stmt::Function { name, params, body } =>
        format!("(fun_def `{}` ({}) ({}))", name, params.iter().map(|s| format!("`{}`", s) ).collect::<Vec<_>>().join(" "), self.print_stmts(body)),
      Stmt::Return(expr) => format!("(return {})", self.print_expr(expr))
    }
  }

  pub fn print_expr(&self, root: &Expr) -> String {
    match root {
      Expr::LiteralNumber { value } => format!("{:?}", value),
      Expr::LiteralString { value } => format!("{value}"),
      Expr::LiteralBool { value } => format!("{value}"),
      Expr::LiteralNil => "nil".to_string(),
      Expr::Unary { operator, right } => {
        format!("({} {})", operator.kind().symbol(), self.print_expr(right))
      }
      Expr::Binary {
        left,
        operator,
        right,
      } => format!(
        "({} {} {})",
        operator.kind().symbol(),
        self.print_expr(left),
        self.print_expr(right)
      ),
      Expr::Group { expression } => format!("(group {})", self.print_expr(expression)),
      Expr::Variable { name, .. } => format!("`{}`", name),
      Expr::Assign { name, value, .. } => {
        format!("(assign_var `{}` {})", name, self.print_expr(value))
      }
      Expr::Logical {
        left,
        operator,
        right,
      } => format!(
        "({} {} {})",
        operator.symbol(),
        self.print_expr(left),
        self.print_expr(right)
      ),
      Expr::Call { callee, args, .. } => {
        let args: Vec<String> = args.iter().map(|a| self.print_expr(a)).collect();
        format!("(call {} ({}))", self.print_expr(callee), args.join(" "))
      }
    }
  }
}
