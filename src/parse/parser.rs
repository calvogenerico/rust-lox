use crate::parse::enum_expr::Expr;
use crate::scan::token::Token;
use crate::scan::token_kind::TokenKind;


pub struct LoxParser {
  tokens: Vec<Token>,
  current_pos: usize,
}

impl LoxParser {
  pub fn new(tokens: Vec<Token>) -> LoxParser {
    LoxParser {
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
    let mut left = self.comparison()?;

    while let Some(operator) = self.advance_if_match(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
      let right = self.comparison()?;
      left = Expr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) };
    }

    Ok(left)
  }

  fn comparison(&mut self) -> Result<Expr, String> {
    let mut left = self.term()?;

    while let Some(operator) = self.advance_if_match(
      &[TokenKind::Less, TokenKind::LessEqual, TokenKind::Greater, TokenKind::GreaterEqual]
    ) {
      let right = self.term()?;
      left = Expr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) };
    }

    Ok(left)
  }

  fn term(&mut self) -> Result<Expr, String> {
    let mut left = self.factor()?;

    while let Some(operator) = self.advance_if_match(
      &[TokenKind::Plus, TokenKind::Minus]
    ) {
      let right = self.factor()?;
      left = Expr::Binary { left: Box::new(left), operator, right: Box::new(right) };
    }

    Ok(left)
  }

  fn factor(&mut self) -> Result<Expr, String> {
    let mut left = self.unary()?;

    while let Some(operator) = self.advance_if_match(
      &[TokenKind::Star, TokenKind::Slash]
    ) {
      let right = self.unary()?;
      left = Expr::Binary { left: Box::new(left), operator, right: Box::new(right) };
    }

    Ok(left)
  }

  fn unary(&mut self) -> Result<Expr, String> {
    if let Some(operator) = self.advance_if_match(
      &[TokenKind::Minus, TokenKind::Bang]
    ) {
      let expr = self.unary()?;
      return Ok(Expr::Unary { operator, right: Box::new(expr) });
    }

    self.primary()
  }

  fn primary(&mut self) -> Result<Expr, String> {
    let maybe = self.next_token();
    let token = maybe.unwrap();

    let expr = match token.kind() {
      TokenKind::Number(repr) => Expr::LiteralNumber { value: repr.parse().unwrap() },
      TokenKind::True => Expr::LiteralBool { value: true },
      TokenKind::False => Expr::LiteralBool { value: false },
      TokenKind::String(repr) => Expr::LiteralString { value: repr.to_string() },
      TokenKind::Nil => Expr::LiteralNil,
      TokenKind::LeftParen => {
        let res = self.expression()?;
        self.next_token().unwrap();
        Expr::Group { expression: Box::new(res) }
      }
      _ => panic!("not implemented")
    };

    Ok(expr)
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
  use crate::parse::print_ast::PrintAst;
  use super::*;


  fn parser(tokens: Vec<Token>) -> LoxParser {
    LoxParser::new(tokens)
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

    assert_eq!(representation, "(== 1.0 2.0)");
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

    assert_eq!(representation, "(!= 1.0 2.0)");
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

      assert_eq!(representation, format!("({} 1.0 2.0)", &token.kind().symbol()));
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

    assert_eq!(representation, "(== (<= 1.0 2.0) 3.0)");
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

    assert_eq!(representation, "(== (+ 1.0 2.0) (- 3.0 4.0))")
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

    assert_eq!(representation, "(* 1.0 2.0)")
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

    assert_eq!(representation, "(== (- 1.0) (! true))")
  }

  #[test]
  fn all_primary_types() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let nil_token = Token::new(TokenKind::Nil, 1);
    let true_token = Token::new(TokenKind::True, 1);
    let false_token = Token::new(TokenKind::False, 1);
    let string_token = Token::new(TokenKind::String("some string".to_string()), 1);

    let print = PrintAst::new();

    assert_eq!(print.print(&LoxParser::new(vec![one_token]).parse()), "1.0");
    assert_eq!(print.print(&LoxParser::new(vec![nil_token]).parse()), "nil");
    assert_eq!(print.print(&LoxParser::new(vec![true_token]).parse()), "true");
    assert_eq!(print.print(&LoxParser::new(vec![false_token]).parse()), "false");
    assert_eq!(print.print(&LoxParser::new(vec![string_token]).parse()), "some string");
  }

  #[test]
  fn simple_grouping() {
    let left_paren = Token::new(TokenKind::LeftParen, 1);
    let right_paren = Token::new(TokenKind::RightParen, 1);
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);

    let parser = parser(vec![
      left_paren,
      one_token,
      right_paren
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(group 1.0)")
  }

  #[test]
  fn can_combine_groups_with_operations() {
    let left_paren = Token::new(TokenKind::LeftParen, 1);
    let right_paren = Token::new(TokenKind::RightParen, 1);
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let plus_token = Token::new(TokenKind::Plus, 1);

    let parser = parser(vec![
      left_paren,
      one_token,
      plus_token,
      two_token,
      right_paren
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(group (+ 1.0 2.0))")
  }

  #[test]
  fn plus_with_unary_test() {
    let nil_token = Token::new(TokenKind::Nil, 1);
    let plus_token = Token::new(TokenKind::Plus, 1);
    let bang_token = Token::new(TokenKind::Bang, 1);
    let false_token = Token::new(TokenKind::False, 1);

    let parser = parser(vec![
      nil_token,
      plus_token,
      bang_token,
      false_token
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(+ nil (! false))")
  }

  #[test]
  fn double_bang_test() {
    let bang_token = Token::new(TokenKind::Bang, 1);
    let false_token = Token::new(TokenKind::False, 1);

    let parser = parser(vec![
      bang_token.clone(),
      bang_token,
      false_token
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

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


    let parser = parser(vec![
      n1,
      star,
      n2,
      slash,
      n3
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

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


    let parser = parser(vec![
      n1,
      plus,
      n2,
      minus,
      n3
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

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


    let parser = parser(vec![
      n1,
      equal_equal,
      n2,
      bang_equal,
      n3
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

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


    let parser = parser(vec![
      n1,
      less,
      n2,
      greater,
      n3
    ]);

    let res = parser.parse();
    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(> (< 84.0 69.0) 56.0)")
  }
}

