use crate::parse::enum_expr::Expr;
use crate::scan::token::Token;
use crate::scan::token_kind::TokenKind;


pub struct Parser {
  tokens: Vec<Token>,
  current_pos: usize,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser {
      tokens,
      current_pos: 0,
    }
  }

  pub fn parse(mut self) -> Expr {
    self.expression().unwrap()
  }

  fn expression(&mut self) -> Result<Expr, String> {
    self.equality()
  }

  fn equality(&mut self) -> Result<Expr, String> {
    let left = self.comparison()?;

    if let Some(operator) = self.advance_if_match(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
      let right = self.comparison()?;
      return Ok(Expr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) });
    }

    Ok(left)
  }

  fn comparison(&mut self) -> Result<Expr, String> {
    let left = self.term()?;

    if let Some(operator) = self.advance_if_match(
      &[TokenKind::Less, TokenKind::LessEqual, TokenKind::Greater, TokenKind::GreaterEqual]
    ) {
      let right = self.primary()?;
      return Ok(Expr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) });
    }

    Ok(left)
  }

  fn term(&mut self) -> Result<Expr, String> {
    let left = self.factor()?;

    if let Some(operator) = self.advance_if_match(
      &[TokenKind::Plus, TokenKind::Minus]
    ) {
      let right = self.primary()?;
      return Ok(Expr::Binary { left: Box::new(left), operator, right: Box::new(right) });
    }

    Ok(left)
  }

  fn factor(&mut self) -> Result<Expr, String> {
    let left = self.unary()?;

    if let Some(operator) = self.advance_if_match(
      &[TokenKind::Star, TokenKind::Slash]
    ) {
      let right = self.primary()?;
      return Ok(Expr::Binary { left: Box::new(left), operator, right: Box::new(right) });
    }

    Ok(left)
  }

  fn unary(&mut self) -> Result<Expr, String> {
    if let Some(operator) = self.advance_if_match(
      &[TokenKind::Minus, TokenKind::Bang]
    ) {
      let expr = self.primary()?;
      return Ok(Expr::Unary { operator, right: Box::new(expr) });
    }

    self.primary()
  }

  fn primary(&mut self) -> Result<Expr, String> {
    let maybe = self.next_token();
    let token = maybe.unwrap();

    let token = match token.kind() {
      TokenKind::Number(repr) => Expr::LiteralNumber { value: repr.parse().unwrap() },
      TokenKind::True => Expr::LiteralBool { value: true },
      TokenKind::False => Expr::LiteralBool { value: false },
      TokenKind::String(repr) => Expr::LiteralString { value: repr.to_string() },
      TokenKind::Nil => Expr::LiteralNil,
      _ => panic!("not implemented")
    };

    Ok(token)
  }


  fn advance_if_match(&mut self, options: &[TokenKind]) -> Option<Token> {
    if let Some(token) = self.peek() {
      if options.iter().any(|opt| opt == token.kind()) {
        let res = Some(token.clone());
        self.next_token();
        return res;
      }
    }
    None
  }

  fn next_token(&mut self) -> Option<&Token> {
    let res = self.tokens.get(self.current_pos);
    self.current_pos += 1;
    res
  }

  fn peek(&mut self) -> Option<&Token> {
    self.tokens.get(self.current_pos)
  }
}


#[cfg(test)]
mod tests {
  use crate::parse::visitor::PrintAst;
  use super::*;


  fn parser(tokens: Vec<Token>) -> Parser {
    Parser::new(tokens)
  }

  #[test]
  fn parse_a_number_returns_a_number_expr() {
    let number_token = Token::new(TokenKind::Number("1.2".to_string()), 1);
    let parser = parser(vec![number_token]);
    let res = parser.parse();

    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "1.2");
  }

  #[test]
  fn parse_true_returns_a_boolean_expr() {
    let bool_token = Token::new(TokenKind::True, 1);
    let parser = parser(vec![bool_token]);
    let res = parser.parse();

    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "true");
  }

  #[test]
  fn parse_equality_returns_corret_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let equal_equal_token = Token::new(TokenKind::EqualEqual, 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);

    let parser = parser(vec![one_token, equal_equal_token, two_token]);
    let res = parser.parse();

    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(== 1 2)");
  }

  #[test]
  fn parse_unequality_returns_correct_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let bang_equal_token = Token::new(TokenKind::BangEqual, 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);

    let parser = parser(vec![one_token, bang_equal_token, two_token]);
    let res = parser.parse();

    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(!= 1 2)");
  }

  #[test]
  fn parse_comparisons_returns_correct_tree() {
    let tokens = vec![
      Token::new(TokenKind::Less, 1),
      Token::new(TokenKind::LessEqual, 1),
      Token::new(TokenKind::Greater, 1),
      Token::new(TokenKind::GreaterEqual, 1)
    ];

    for token in tokens {
      let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
      let two_token = Token::new(TokenKind::Number("2".to_string()), 1);

      let parser = parser(vec![one_token, token.clone(), two_token]);
      let res = parser.parse();

      let visitor = PrintAst {};
      let representation = visitor.print(&res);

      assert_eq!(representation, format!("({} 1 2)", &token.kind().symbol()));
    }

  }

  #[test]
  fn parse_less_than_between_equal_equal_returns_correct_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let three_token = Token::new(TokenKind::Number("3".to_string()), 1);
    let less_than_token = Token::new(TokenKind::LessEqual, 1);
    let equal_equal_token = Token::new(TokenKind::EqualEqual, 1);

    let parser = parser(vec![
      one_token,
      less_than_token,
      two_token,
      equal_equal_token,
      three_token
    ]);
    let res = parser.parse();

    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(== (<= 1 2) 3)");
  }

  #[test]
  fn parse_plus_returns_right_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let plus_token = Token::new(TokenKind::Plus, 1);

    let parser = parser(vec![
      one_token,
      plus_token,
      two_token
    ]);
    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(+ 1 2)")
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

    let parser = parser(vec![
      one_token,
      plus_token,
      two_token,
      equal_equal_token,
      tree_token,
      minus_token,
      four_token
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(== (+ 1 2) (- 3 4))")
  }

  #[test]
  fn product_test() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let star_token = Token::new(TokenKind::Star, 1);

    let parser = parser(vec![
      one_token,
      star_token,
      two_token
    ]);
    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(* 1 2)")
  }

  #[test]
  fn unary_test() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let minus_token = Token::new(TokenKind::Minus, 1);

    let true_token = Token::new(TokenKind::True, 1);
    let bang_token = Token::new(TokenKind::Bang, 1);

    let equal_equal_token = Token::new(TokenKind::EqualEqual, 1);

    let parser = parser(vec![
      minus_token,
      one_token,
      equal_equal_token,
      bang_token,
      true_token
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(== (-1) (!true))")
  }

  #[test]
  fn all_primary_types() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let nil_token = Token::new(TokenKind::Nil, 1);
    let true_token = Token::new(TokenKind::True, 1);
    let false_token = Token::new(TokenKind::False, 1);
    let string_token = Token::new(TokenKind::String("some string".to_string()), 1);

    let print = PrintAst::new();

    assert_eq!(print.print(&Parser::new(vec![one_token]).parse()), "1");
    assert_eq!(print.print(&Parser::new(vec![nil_token]).parse()), "nil");
    assert_eq!(print.print(&Parser::new(vec![true_token]).parse()), "true");
    assert_eq!(print.print(&Parser::new(vec![false_token]).parse()), "false");
    assert_eq!(print.print(&Parser::new(vec![string_token]).parse()), "\"some string\"");
  }
}