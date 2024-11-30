use crate::parse::expr::Expr;
use crate::parse::parse_error::ParseError;
use crate::parse::stmt::Stmt;
use crate::scan::token::Token;
use crate::scan::token_kind::TokenKind;
use std::cell::RefCell;

pub struct LoxParser {
  tokens: Vec<Token>,
  current_pos: RefCell<usize>,
}

impl LoxParser {
  pub fn new(tokens: Vec<Token>) -> LoxParser {
    LoxParser {
      tokens,
      current_pos: RefCell::new(0),
    }
  }

  fn inc(&self) {
    *self.current_pos.borrow_mut() += 1;
  }

  fn pos(&self) -> usize {
    *self.current_pos.borrow()
  }

  pub fn parse(mut self) -> Result<Vec<Stmt>, ParseError> {
    let mut stmts = vec![];
    while !self.is_at_end() {
      stmts.push(self.declaration()?)
    }
    Ok(stmts)
  }

  fn declaration(&mut self) -> Result<Stmt, ParseError> {
    if self.advance_if_match(&[TokenKind::Var]).is_some() {
      self.var_declaration()
    } else {
      self.statement()
    }
  }

  fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
    let token = self.next_token()?;
    let line = token.line();
    if let TokenKind::Identifier(name) = token.kind() {
      let name = name.clone();
      let stmt = if self.peek_kind().is_some_and(|k| *k == TokenKind::Equal) {
        self.consume(TokenKind::Equal)?;
        let expr = self.expression()?;
        Stmt::Var(name, expr, line)
      } else {
        Stmt::Var(name, Expr::LiteralNil, line)
      };
      self.consume(TokenKind::Semicolon)?;
      Ok(stmt)
    } else {
      Err(ParseError::MalformedExpression(
        line,
        format!("Expected identifier, got {}", token.symbol()),
      ))
    }
  }

  fn statement(&mut self) -> Result<Stmt, ParseError> {
    let stmt = match self
      .advance_if_match(&[
        TokenKind::Print,
        TokenKind::If,
        TokenKind::LeftBrace,
        TokenKind::While,
        TokenKind::For,
      ])
      .map(|t| t.kind())
    {
      Some(TokenKind::Print) => self.print_stmt()?,
      Some(TokenKind::If) => self.if_stmt()?,
      Some(TokenKind::LeftBrace) => self.scope_block()?,
      Some(TokenKind::While) => self.while_stmt()?,
      Some(TokenKind::For) => self.for_stmt()?,
      _ => self.expression_stmt()?,
    };

    Ok(stmt)
  }

  fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
    let stmt = Stmt::Print(self.expression()?);

    if !self.is_at_end() {
      self.consume(TokenKind::Semicolon)?;
    }

    Ok(stmt)
  }

  fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
    self.consume(TokenKind::LeftParen)?;
    let condition = self.expression()?;
    self.consume(TokenKind::RightParen)?;
    let then = self.statement().map(|s| Box::new(s))?;

    let els = self
      .advance_if_match(&[TokenKind::Else])
      .map(|_| ())
      .map(|_| self.statement().map(|s| Box::new(s)))
      .transpose()?;

    Ok(Stmt::If {
      condition,
      then,
      els,
    })
  }

  fn scope_block(&mut self) -> Result<Stmt, ParseError> {
    let mut stmts = vec![];

    while self
      .peek_kind()
      .is_some_and(|k| *k != TokenKind::RightBrace)
    {
      stmts.push(self.declaration()?)
    }

    self.consume(TokenKind::RightBrace)?;

    Ok(Stmt::ScopeBlock(stmts))
  }

  fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
    self.consume(TokenKind::LeftParen)?;
    let condition = self.expression()?;
    self.consume(TokenKind::RightParen)?;

    let body = self.statement().map(Box::new)?;

    Ok(Stmt::While { condition, body })
  }

  fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
    self.consume(TokenKind::LeftParen)?;

    // Var declaration -- for (HERE;;) {}
    let declaration = match self.advance_if_match(&[TokenKind::Semicolon]) {
      Some(_) => None,
      None => {
        if let Some(_) = self.advance_if_match(&[TokenKind::Var]) {
          Some(self.var_declaration()?)
        } else {
          Some(self.expression_stmt()?)
        }
      }
    };

    // Ending condition -- for (;HERE;) {}
    let condition = self
      .peek_kind()
      .filter(|k| **k != TokenKind::Semicolon)
      .map(|_| ())
      .map(|_| self.expression())
      .transpose()?;
    self.consume(TokenKind::Semicolon)?;

    // Increment -- for (;;HERE) {}
    let increment = self
      .peek_kind()
      .filter(|k| **k != TokenKind::RightParen)
      .map(|_| ())
      .map(|_| self.expression())
      .transpose()?;
    self.consume(TokenKind::RightParen)?;

    // Body -- for (;;) HERE
    let for_body = self.statement()?;

    // Assemble all together
    let while_body = match increment {
      Some(inc) => Stmt::ScopeBlock(vec![for_body, Stmt::Expr(inc)]),
      None => for_body,
    };

    let while_stmt = Stmt::While {
      condition: condition.unwrap_or(Expr::LiteralBool { value: true }),
      body: Box::new(while_body),
    };

    let mut stmts = match declaration {
      Some(stmt) => vec![stmt],
      None => vec![],
    };

    stmts.push(while_stmt);
    Ok(Stmt::ScopeBlock(stmts))
  }

  fn expression_stmt(&mut self) -> Result<Stmt, ParseError> {
    let stmt = Stmt::Expr(self.expression()?);

    if !self.is_at_end() {
      self.consume(TokenKind::Semicolon)?;
    }
    Ok(stmt)
  }

  fn expression(&mut self) -> Result<Expr, ParseError> {
    self.assignment()
  }

  fn assignment(&mut self) -> Result<Expr, ParseError> {
    let left = self.or()?;

    if let Some(TokenKind::Equal) = self.peek_kind() {
      let equals = self.next_token()?;
      let equals_line = equals.line();

      // This line eagerly consumes to the right;
      let right = self.assignment()?;

      if let Expr::Variable { name, line } = left {
        return Ok(Expr::Assign {
          name,
          value: Box::new(right),
          line,
        });
      } else {
        return Err(ParseError::MalformedExpression(
          equals_line,
          "Invalid assignment target.".to_string(),
        ));
      }
    }

    Ok(left)
  }

  fn or(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.and()?;

    while let Some(operator) = self.advance_if_match(&[TokenKind::Or]) {
      let operator = operator.clone();
      let right = self.and()?;
      left = Expr::Logical {
        left: Box::new(left),
        operator,
        right: Box::new(right),
      };
    }

    Ok(left)
  }

  fn and(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.equality()?;

    while let Some(operator) = self.advance_if_match(&[TokenKind::And]) {
      let operator = operator.clone();
      let right = self.equality()?;
      left = Expr::Logical {
        left: Box::new(left),
        operator,
        right: Box::new(right),
      };
    }

    Ok(left)
  }

  fn equality(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.comparison()?;

    while let Some(operator) = self.advance_if_match(&[TokenKind::EqualEqual, TokenKind::BangEqual])
    {
      let operator = operator.clone();
      let right = self.comparison()?;
      left = Expr::Binary {
        left: Box::new(left),
        operator,
        right: Box::new(right),
      };
    }

    Ok(left)
  }

  fn comparison(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.term()?;

    while let Some(operator) = self.advance_if_match(&[
      TokenKind::Less,
      TokenKind::LessEqual,
      TokenKind::Greater,
      TokenKind::GreaterEqual,
    ]) {
      let operator = operator.clone();
      let right = self.term()?;
      left = Expr::Binary {
        left: Box::new(left),
        operator,
        right: Box::new(right),
      };
    }

    Ok(left)
  }

  fn term(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.factor()?;

    while let Some(operator) = self.advance_if_match(&[TokenKind::Plus, TokenKind::Minus]) {
      let operator = operator.clone();
      let right = self.factor()?;
      left = Expr::Binary {
        left: Box::new(left),
        operator,
        right: Box::new(right),
      };
    }

    Ok(left)
  }

  fn factor(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.unary()?;

    while let Some(operator) = self.advance_if_match(&[TokenKind::Star, TokenKind::Slash]) {
      let operator = operator.clone();
      let right = self.unary()?;
      left = Expr::Binary {
        left: Box::new(left),
        operator,
        right: Box::new(right),
      };
    }

    Ok(left)
  }

  fn unary(&mut self) -> Result<Expr, ParseError> {
    if let Some(operator) = self.advance_if_match(&[TokenKind::Minus, TokenKind::Bang]) {
      let operator = operator.clone();
      let expr = self.unary()?;
      return Ok(Expr::Unary {
        operator,
        right: Box::new(expr),
      });
    }

    self.call()
  }

  fn call(&mut self) -> Result<Expr, ParseError> {
    let mut expr = self.primary()?;

    while let Some(TokenKind::LeftParen) = self.peek_kind() {
      let paren_line = self.consume(TokenKind::LeftParen)?.line();

      // let args = vec![];
      let args = if let Some(_) = self.advance_if_match(&[TokenKind::RightParen]) {
        vec![]
      } else {
        let mut args = vec![];

        loop {
          if let Some(_) = self.advance_if_match(&[TokenKind::RightParen]) {
            break;
          }

          args.push(self.expression()?);
          self.advance_if_match(&[TokenKind::Comma]);
        }

        args
      };

      expr = Expr::Call { line: paren_line, callee: Box::new(expr), args }
    }
    Ok(expr)
  }

  fn primary(&mut self) -> Result<Expr, ParseError> {
    let token = self.next_token()?.clone();

    match token.kind() {
      TokenKind::Number(repr) => Ok(Expr::LiteralNumber {
        value: repr.parse().unwrap(),
      }),
      TokenKind::True => Ok(Expr::LiteralBool { value: true }),
      TokenKind::False => Ok(Expr::LiteralBool { value: false }),
      TokenKind::String(repr) => Ok(Expr::LiteralString {
        value: repr.to_string(),
      }),
      TokenKind::Nil => Ok(Expr::LiteralNil),
      TokenKind::Identifier(name) => Ok(Expr::Variable {
        name: name.clone(),
        line: token.line(),
      }),
      TokenKind::LeftParen => {
        let res = self.expression()?;

        self.consume(TokenKind::RightParen).map_err(|_| {
          ParseError::MalformedExpression(token.line(), "Missing closing parenthesis".to_string())
        })?;

        Ok(Expr::Group {
          expression: Box::new(res),
        })
      }
      TokenKind::Eof => Err(ParseError::MalformedExpression(
        token.line(),
        "Unexpected end of file".to_string(),
      )),
      _ => Err(ParseError::MalformedExpression(
        token.line(),
        format!("Expected expression got `{}`", token.symbol()),
      )),
    }
  }

  fn advance_if_match(&mut self, options: &[TokenKind]) -> Option<&Token> {
    if let Some(token) = self.peek() {
      if options.iter().any(|opt| opt == token.kind()) {
        let res = Some(token);
        self.inc();
        return res;
      }
    }
    None
  }

  fn next_token(&mut self) -> Result<&Token, ParseError> {
    let res = self.tokens.get(self.pos());
    self.inc();
    res.ok_or(ParseError::UnexpectedEndOfFile)
  }

  fn peek(&self) -> Option<&Token> {
    self.tokens.get(self.pos())
  }

  fn peek_kind(&self) -> Option<&TokenKind> {
    self.peek().map(|t| t.kind())
  }

  fn is_at_end(&self) -> bool {
    self.peek().is_some_and(|t| *t.kind() == TokenKind::Eof)
  }

  fn consume(&mut self, kind: TokenKind) -> Result<&Token, ParseError> {
    let next = self.next_token()?;

    if *next.kind() == kind {
      return Ok(next);
    }

    Err(ParseError::MalformedExpression(
      next.line(),
      format!("Expected {}, got {}", kind.symbol(), next.kind().symbol()),
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::parse::print_ast::PrintAst;
  use crate::scan::scanner::Scanner;
  use std::io::Cursor;

  fn parser(tokens: Vec<Token>) -> LoxParser {
    LoxParser::new(tokens)
  }

  fn parse_and_print_expr(mut tokens: Vec<Token>) -> String {
    tokens.push(Token::new(TokenKind::Semicolon, 1));
    tokens.push(Token::new(TokenKind::Eof, 1));
    let parser = parser(tokens);
    let res = parser.parse().unwrap().pop().unwrap();
    let visitor = PrintAst {};
    match res {
      Stmt::Expr(expr) => visitor.print_expr(&expr),
      _ => panic!("should not be this"),
    }
  }

  #[test]
  fn parse_a_number_returns_a_number_expr() {
    let number_token = Token::new(TokenKind::Number("1.2".to_string()), 1);
    let representation = parse_and_print_expr(vec![number_token]);
    assert_eq!(representation, "1.2");
  }

  #[test]
  fn parse_true_returns_a_boolean_expr() {
    let bool_token = Token::new(TokenKind::True, 1);
    let representation = parse_and_print_expr(vec![bool_token]);

    assert_eq!(representation, "true");
  }

  #[test]
  fn parse_equality_returns_corret_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let equal_equal_token = Token::new(TokenKind::EqualEqual, 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);

    let representation = parse_and_print_expr(vec![one_token, equal_equal_token, two_token]);

    assert_eq!(representation, "(== 1.0 2.0)");
  }

  #[test]
  fn parse_unequality_returns_correct_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let bang_equal_token = Token::new(TokenKind::BangEqual, 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);

    let representation = parse_and_print_expr(vec![one_token, bang_equal_token, two_token]);

    assert_eq!(representation, "(!= 1.0 2.0)");
  }

  #[test]
  fn parse_comparisons_returns_correct_tree() {
    let tokens = vec![
      Token::new(TokenKind::Less, 1),
      Token::new(TokenKind::LessEqual, 1),
      Token::new(TokenKind::Greater, 1),
      Token::new(TokenKind::GreaterEqual, 1),
    ];

    for token in tokens {
      let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
      let two_token = Token::new(TokenKind::Number("2".to_string()), 1);

      let representation = parse_and_print_expr(vec![one_token, token.clone(), two_token]);

      assert_eq!(
        representation,
        format!("({} 1.0 2.0)", &token.kind().symbol())
      );
    }
  }

  #[test]
  fn parse_less_than_between_equal_equal_returns_correct_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let three_token = Token::new(TokenKind::Number("3".to_string()), 1);
    let less_than_token = Token::new(TokenKind::LessEqual, 1);
    let equal_equal_token = Token::new(TokenKind::EqualEqual, 1);

    let representation = parse_and_print_expr(vec![
      one_token,
      less_than_token,
      two_token,
      equal_equal_token,
      three_token,
    ]);

    assert_eq!(representation, "(== (<= 1.0 2.0) 3.0)");
  }

  #[test]
  fn parse_plus_returns_right_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let plus_token = Token::new(TokenKind::Plus, 1);

    let representation = parse_and_print_expr(vec![one_token, plus_token, two_token]);

    assert_eq!(representation, "(+ 1.0 2.0)")
  }

  #[test]
  fn parse_plus_minus_and_comparissons_returns_right_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let tree_token = Token::new(TokenKind::Number("3".to_string()), 1);
    let four_token = Token::new(TokenKind::Number("4".to_string()), 1);
    let plus_token = Token::new(TokenKind::Plus, 1);
    let minus_token = Token::new(TokenKind::Minus, 1);
    let equal_equal_token = Token::new(TokenKind::EqualEqual, 1);

    let representation = parse_and_print_expr(vec![
      one_token,
      plus_token,
      two_token,
      equal_equal_token,
      tree_token,
      minus_token,
      four_token,
    ]);

    assert_eq!(representation, "(== (+ 1.0 2.0) (- 3.0 4.0))")
  }

  #[test]
  fn product_test() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let star_token = Token::new(TokenKind::Star, 1);

    let representation = parse_and_print_expr(vec![one_token, star_token, two_token]);

    assert_eq!(representation, "(* 1.0 2.0)")
  }

  #[test]
  fn unary_test() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let minus_token = Token::new(TokenKind::Minus, 1);

    let true_token = Token::new(TokenKind::True, 1);
    let bang_token = Token::new(TokenKind::Bang, 1);

    let equal_equal_token = Token::new(TokenKind::EqualEqual, 1);

    let representation = parse_and_print_expr(vec![
      minus_token,
      one_token,
      equal_equal_token,
      bang_token,
      true_token,
    ]);

    assert_eq!(representation, "(== (- 1.0) (! true))")
  }

  #[test]
  fn all_primary_types() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let nil_token = Token::new(TokenKind::Nil, 1);
    let true_token = Token::new(TokenKind::True, 1);
    let false_token = Token::new(TokenKind::False, 1);
    let string_token = Token::new(TokenKind::String("some string".to_string()), 1);

    assert_eq!(parse_and_print_expr(vec![one_token]), "1.0");
    assert_eq!(parse_and_print_expr(vec![nil_token]), "nil");
    assert_eq!(parse_and_print_expr(vec![true_token]), "true");
    assert_eq!(parse_and_print_expr(vec![false_token]), "false");
    assert_eq!(parse_and_print_expr(vec![string_token]), "some string");
  }

  #[test]
  fn simple_grouping() {
    let left_paren = Token::new(TokenKind::LeftParen, 1);
    let right_paren = Token::new(TokenKind::RightParen, 1);
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);

    let representation = parse_and_print_expr(vec![left_paren, one_token, right_paren]);

    assert_eq!(representation, "(group 1.0)")
  }

  #[test]
  fn can_combine_groups_with_operations() {
    let left_paren = Token::new(TokenKind::LeftParen, 1);
    let right_paren = Token::new(TokenKind::RightParen, 1);
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let plus_token = Token::new(TokenKind::Plus, 1);

    let representation = parse_and_print_expr(vec![
      left_paren,
      one_token,
      plus_token,
      two_token,
      right_paren,
    ]);

    assert_eq!(representation, "(group (+ 1.0 2.0))")
  }

  #[test]
  fn plus_with_unary_test() {
    let nil_token = Token::new(TokenKind::Nil, 1);
    let plus_token = Token::new(TokenKind::Plus, 1);
    let bang_token = Token::new(TokenKind::Bang, 1);
    let false_token = Token::new(TokenKind::False, 1);

    let representation = parse_and_print_expr(vec![nil_token, plus_token, bang_token, false_token]);

    assert_eq!(representation, "(+ nil (! false))")
  }

  #[test]
  fn double_bang_test() {
    let bang_token = Token::new(TokenKind::Bang, 1);
    let false_token = Token::new(TokenKind::False, 1);

    let representation = parse_and_print_expr(vec![bang_token.clone(), bang_token, false_token]);

    assert_eq!(representation, "(! (! false))")
  }

  #[test]
  fn multiple_divs_and_mult() {
    // 84 * 69 / 56
    let n1 = Token::new(TokenKind::Number("84".to_string()), 1);
    let n2 = Token::new(TokenKind::Number("69".to_string()), 1);
    let n3 = Token::new(TokenKind::Number("56".to_string()), 1);
    let star = Token::new(TokenKind::Star, 1);
    let slash = Token::new(TokenKind::Slash, 1);

    let representation = parse_and_print_expr(vec![n1, star, n2, slash, n3]);

    assert_eq!(representation, "(/ (* 84.0 69.0) 56.0)")
  }

  #[test]
  fn multiple_plus_and_minus() {
    // 84 * 69 / 56
    let n1 = Token::new(TokenKind::Number("84".to_string()), 1);
    let n2 = Token::new(TokenKind::Number("69".to_string()), 1);
    let n3 = Token::new(TokenKind::Number("56".to_string()), 1);
    let plus = Token::new(TokenKind::Plus, 1);
    let minus = Token::new(TokenKind::Minus, 1);

    let representation = parse_and_print_expr(vec![n1, plus, n2, minus, n3]);

    assert_eq!(representation, "(- (+ 84.0 69.0) 56.0)")
  }

  #[test]
  fn multiple_equalities() {
    // 84 * 69 / 56
    let n1 = Token::new(TokenKind::Number("84".to_string()), 1);
    let n2 = Token::new(TokenKind::Number("69".to_string()), 1);
    let n3 = Token::new(TokenKind::Number("56".to_string()), 1);
    let equal_equal = Token::new(TokenKind::EqualEqual, 1);
    let bang_equal = Token::new(TokenKind::BangEqual, 1);

    let representation = parse_and_print_expr(vec![n1, equal_equal, n2, bang_equal, n3]);

    assert_eq!(representation, "(!= (== 84.0 69.0) 56.0)")
  }

  #[test]
  fn multiple_comparisons() {
    // 84 * 69 / 56
    let n1 = Token::new(TokenKind::Number("84".to_string()), 1);
    let n2 = Token::new(TokenKind::Number("69".to_string()), 1);
    let n3 = Token::new(TokenKind::Number("56".to_string()), 1);
    let less = Token::new(TokenKind::Less, 1);
    let greater = Token::new(TokenKind::Greater, 1);

    let representation = parse_and_print_expr(vec![n1, less, n2, greater, n3]);

    assert_eq!(representation, "(> (< 84.0 69.0) 56.0)")
  }

  #[test]
  fn a_non_closed_parenthesis_returns_error() {
    let tokens = vec![
      Token::new(TokenKind::LeftParen, 1),
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::Number("2".to_string()), 1),
    ];

    let parser = parser(tokens);
    let res = parser.parse();

    assert!(res.is_err());
    assert_eq!(
      res.unwrap_err(),
      ParseError::MalformedExpression(1, "Missing closing parenthesis".to_string())
    );
  }

  #[test]
  fn a_missing_term_after_a_plus_returns_error() {
    let tokens = vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::Eof, 1),
    ];

    let parser = parser(tokens);
    let res = parser.parse();

    assert!(res.is_err());
    assert_eq!(
      res.unwrap_err(),
      ParseError::MalformedExpression(1, "Unexpected end of file".to_string())
    );
  }

  fn parse_from_code(code: &str) -> String {
    let mut cursor = Cursor::new(code);
    let scanner = Scanner::new(&mut cursor);
    let tokens = scanner.scan_tokens().0;
    let parser = LoxParser::new(tokens);
    let stmts = parser.parse().unwrap();
    PrintAst::new().print_stmts(&stmts)
  }

  #[test]
  fn parse_print_stmt() {
    let ast = parse_from_code("print 1;");

    assert_eq!(ast, "(print 1.0)");
  }

  #[test]
  fn parse_print_with_more_complex_expression_stmt() {
    let ast = parse_from_code("print (1 + 2) > 0;");

    assert_eq!(ast, "(print (> (group (+ 1.0 2.0)) 0.0))");
  }

  #[test]
  fn parse_multiple_stmts() {
    let ast = parse_from_code("1 + 1; 2 + 2;");

    assert_eq!(ast, "(+ 1.0 1.0) (+ 2.0 2.0)");
  }

  #[test]
  fn can_parse_variables() {
    let ast = parse_from_code("var foo = 1;");
    assert_eq!(ast, "(def_var `foo` 1.0)");
  }

  #[test]
  fn can_parse_variables_not_initialized() {
    let ast = parse_from_code("var foo;");
    assert_eq!(ast, "(def_var `foo` nil)");
  }

  #[test]
  fn can_parse_re_assign_some_var() {
    let ast = parse_from_code("var foo = 1;foo = 2;");
    assert_eq!(ast, "(def_var `foo` 1.0) (assign_var `foo` 2.0)");
  }

  #[test]
  fn can_parse_scopes() {
    let ast = parse_from_code("{ 1 + 2; }");
    assert_eq!(ast, "(block_scope (+ 1.0 2.0))");
  }

  #[test]
  fn can_parse_scopes_between_stmts() {
    let ast = parse_from_code("nil; { 1 + 2; } 3;");
    assert_eq!(ast, "nil (block_scope (+ 1.0 2.0)) 3.0");
  }

  #[test]
  fn can_parse_multiple_stmts_inside_scope() {
    let ast = parse_from_code("{ 1 + 2; 2 + 3; nil;}");
    assert_eq!(ast, "(block_scope (+ 1.0 2.0) (+ 2.0 3.0) nil)");
  }

  #[test]
  fn last_comma_can_be_missing() {
    let ast = parse_from_code("1+2; 2+3");
    assert_eq!(ast, "(+ 1.0 2.0) (+ 2.0 3.0)");
  }

  #[test]
  fn can_parse_multiple_assignments_in_one_line() {
    let ast = parse_from_code("var a; var b; a = b = 3;");
    assert_eq!(
      ast,
      "(def_var `a` nil) (def_var `b` nil) (assign_var `a` (assign_var `b` 3.0))"
    );
  }

  #[test]
  fn can_parse_if_stmts() {
    let ast = parse_from_code("if (1 > 2) { 1; } else { 2; } ");
    assert_eq!(ast, "(if (> 1.0 2.0) (block_scope 1.0) (block_scope 2.0))");
  }

  #[test]
  fn can_parse_while_stmts() {
    let ast = parse_from_code("while (a < 10) { 1; }");
    assert_eq!(ast, "(while (< `a` 10.0) (block_scope 1.0))");
  }

  #[test]
  fn can_parse_while_stmts_with_single_line() {
    let ast = parse_from_code("while (a < 10) 1;");
    assert_eq!(ast, "(while (< `a` 10.0) 1.0)");
  }

  #[test]
  fn can_parse_and_expr() {
    let ast = parse_from_code("10 and 1;");
    assert_eq!(ast, "(and 10.0 1.0)");
  }

  #[test]
  fn can_parse_or_expr() {
    let ast = parse_from_code("10 or 1;");
    assert_eq!(ast, "(or 10.0 1.0)");
  }

  #[test]
  fn can_parse_a_for_expr() {
    let ast = parse_from_code("for (var i = 0; i < 3; i = i + 1) print i;");
    assert_eq!(ast, "(block_scope (def_var `i` 0.0) (while (< `i` 3.0) (block_scope (print `i`) (assign_var `i` (+ `i` 1.0)))))");
  }

  #[test]
  fn can_parse_a_for_with_no_initial_assignment() {
    let ast = parse_from_code("for (; i < 3; i = i + 1) print i;");
    assert_eq!(
      ast,
      "(block_scope (while (< `i` 3.0) (block_scope (print `i`) (assign_var `i` (+ `i` 1.0)))))"
    );
  }

  #[test]
  fn can_parse_a_for_with_no_condition() {
    let ast = parse_from_code("for (var i = 0;; i = i + 1) print i;");
    assert_eq!(
      ast,
      "(block_scope (def_var `i` 0.0) (while true (block_scope (print `i`) (assign_var `i` (+ `i` 1.0)))))"
    );
  }

  #[test]
  fn can_parse_a_for_with_no_increment() {
    let ast = parse_from_code("for (i = 0; i < 3;) print i;");
    assert_eq!(
      ast,
      "(block_scope (assign_var `i` 0.0) (while (< `i` 3.0) (print `i`)))"
    );
  }

  #[test]
  fn can_parse_a_function_call_with_no_args() {
    let ast = parse_from_code("somefunc();");
    assert_eq!(ast, "(call `somefunc` ())");
  }

  #[test]
  fn can_parse_a_function_call_with_args() {
    let ast = parse_from_code("somefunc(1, 2, 3);");
    assert_eq!(ast, "(call `somefunc` (1.0 2.0 3.0))");
  }

  #[test]
  fn can_parse_a_function_call_complex_args_args() {
    let ast = parse_from_code("somefunc(1, 3 + 2, arg());");
    assert_eq!(ast, "(call `somefunc` (1.0 (+ 3.0 2.0) (call `arg` ())))");
  }

  #[test]
  fn coso_01() {
    let ast = parse_from_code("var a; if (false) { a = 10; } print a");
    assert_eq!(ast, "(def_var `a` nil) (if false (block_scope (assign_var `a` 10.0)) ) (print `a`)");
  }
}
