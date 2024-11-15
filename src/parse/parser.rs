use crate::parse::expr::Expr;
use crate::parse::parse_error::ParseError;
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

  pub fn parse(mut self) -> Result<Expr, ParseError> {
    self.expression()
  }

  fn expression(&mut self) -> Result<Expr, ParseError> {
    self.equality()
  }

  fn equality(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.comparison()?;

    while let Some(operator) = self.advance_if_match(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
      let right = self.comparison()?;
      left = Expr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) };
    }

    Ok(left)
  }

  fn comparison(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.term()?;

    while let Some(operator) = self.advance_if_match(
      &[TokenKind::Less, TokenKind::LessEqual, TokenKind::Greater, TokenKind::GreaterEqual]
    ) {
      let right = self.term()?;
      left = Expr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) };
    }

    Ok(left)
  }

  fn term(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.factor()?;

    while let Some(operator) = self.advance_if_match(
      &[TokenKind::Plus, TokenKind::Minus]
    ) {
      let right = self.factor()?;
      left = Expr::Binary { left: Box::new(left), operator, right: Box::new(right) };
    }

    Ok(left)
  }

  fn factor(&mut self) -> Result<Expr, ParseError> {
    let mut left = self.unary()?;

    while let Some(operator) = self.advance_if_match(
      &[TokenKind::Star, TokenKind::Slash]
    ) {
      let right = self.unary()?;
      left = Expr::Binary { left: Box::new(left), operator, right: Box::new(right) };
    }

    Ok(left)
  }

  fn unary(&mut self) -> Result<Expr, ParseError> {
    if let Some(operator) = self.advance_if_match(
      &[TokenKind::Minus, TokenKind::Bang]
    ) {
      let expr = self.unary()?;
      return Ok(Expr::Unary { operator, right: Box::new(expr) });
    }

    self.primary()
  }

  fn primary(&mut self) -> Result<Expr, ParseError> {
    let maybe_token = self.next_token();
    // let maybe_kind = maybe_token.map(|t| t.kind());
    if maybe_token.is_none() {
      return Err(ParseError::MissingEOF);
    }

    let token = maybe_token.unwrap().clone();

    match token.kind() {
      TokenKind::Number(repr) => Ok(Expr::LiteralNumber { value: repr.parse().unwrap() }),
      TokenKind::True => Ok(Expr::LiteralBool { value: true }),
      TokenKind::False => Ok(Expr::LiteralBool { value: false }),
      TokenKind::String(repr) => Ok(Expr::LiteralString { value: repr.to_string() }),
      TokenKind::Nil => Ok(Expr::LiteralNil),
      TokenKind::LeftParen => {
        let res = self.expression()?;
        let closed = self.next_token().is_some_and(|atoken| {
          *atoken.kind() == TokenKind::RightParen
        });

        if !closed {
          return Err(ParseError::MalformedExpression(token.line(), "Missing closing parenthesis".to_string()));
        }

        Ok(Expr::Group { expression: Box::new(res) })
      }
      TokenKind::Eof => Err(ParseError::MalformedExpression(token.line(), "Unexpected end of file".to_string())),
      _ => Err(ParseError::MalformedExpression(token.line(), format!("Expected expression got `{}`", token.symbol())))
    }
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

  fn parse_and_print(tokens: Vec<Token>) -> String {
    let parser = parser(tokens);
    let res = parser.parse().unwrap();
    let visitor = PrintAst {};
    visitor.print(&res)
  }

  #[test]
  fn parse_a_number_returns_a_number_expr() {
    let number_token = Token::new(TokenKind::Number("1.2".to_string()), 1);
    let representation = parse_and_print(vec![number_token]);
    assert_eq!(representation, "1.2");
  }

  #[test]
  fn parse_true_returns_a_boolean_expr() {
    let bool_token = Token::new(TokenKind::True, 1);
    let representation = parse_and_print(vec![bool_token]);

    assert_eq!(representation, "true");
  }

  #[test]
  fn parse_equality_returns_corret_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let equal_equal_token = Token::new(TokenKind::EqualEqual, 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);

    let representation = parse_and_print(vec![one_token, equal_equal_token, two_token]);

    assert_eq!(representation, "(== 1.0 2.0)");
  }

  #[test]
  fn parse_unequality_returns_correct_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let bang_equal_token = Token::new(TokenKind::BangEqual, 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);

    let representation = parse_and_print(vec![one_token, bang_equal_token, two_token]);

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


      let representation = parse_and_print(vec![one_token, token.clone(), two_token]);

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

    let representation = parse_and_print(vec![
      one_token,
      less_than_token,
      two_token,
      equal_equal_token,
      three_token
    ]);

    assert_eq!(representation, "(== (<= 1.0 2.0) 3.0)");
  }

  #[test]
  fn parse_plus_returns_right_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let plus_token = Token::new(TokenKind::Plus, 1);

    let representation = parse_and_print(vec![
      one_token,
      plus_token,
      two_token
    ]);

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

    let representation = parse_and_print(vec![
      one_token,
      plus_token,
      two_token,
      equal_equal_token,
      tree_token,
      minus_token,
      four_token
    ]);

    assert_eq!(representation, "(== (+ 1.0 2.0) (- 3.0 4.0))")
  }

  #[test]
  fn product_test() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let star_token = Token::new(TokenKind::Star, 1);

    let representation = parse_and_print(vec![
      one_token,
      star_token,
      two_token
    ]);

    assert_eq!(representation, "(* 1.0 2.0)")
  }

  #[test]
  fn unary_test() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let minus_token = Token::new(TokenKind::Minus, 1);

    let true_token = Token::new(TokenKind::True, 1);
    let bang_token = Token::new(TokenKind::Bang, 1);

    let equal_equal_token = Token::new(TokenKind::EqualEqual, 1);

    let representation = parse_and_print(vec![
      minus_token,
      one_token,
      equal_equal_token,
      bang_token,
      true_token
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

    assert_eq!(parse_and_print(vec![one_token]), "1.0");
    assert_eq!(parse_and_print(vec![nil_token]), "nil");
    assert_eq!(parse_and_print(vec![true_token]), "true");
    assert_eq!(parse_and_print(vec![false_token]), "false");
    assert_eq!(parse_and_print(vec![string_token]), "some string");
  }

  #[test]
  fn simple_grouping() {
    let left_paren = Token::new(TokenKind::LeftParen, 1);
    let right_paren = Token::new(TokenKind::RightParen, 1);
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);

    let representation = parse_and_print(vec![
      left_paren,
      one_token,
      right_paren
    ]);

    assert_eq!(representation, "(group 1.0)")
  }

  #[test]
  fn can_combine_groups_with_operations() {
    let left_paren = Token::new(TokenKind::LeftParen, 1);
    let right_paren = Token::new(TokenKind::RightParen, 1);
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);
    let plus_token = Token::new(TokenKind::Plus, 1);

    let representation = parse_and_print(vec![
      left_paren,
      one_token,
      plus_token,
      two_token,
      right_paren
    ]);

    assert_eq!(representation, "(group (+ 1.0 2.0))")
  }

  #[test]
  fn plus_with_unary_test() {
    let nil_token = Token::new(TokenKind::Nil, 1);
    let plus_token = Token::new(TokenKind::Plus, 1);
    let bang_token = Token::new(TokenKind::Bang, 1);
    let false_token = Token::new(TokenKind::False, 1);

    let representation = parse_and_print(vec![
      nil_token,
      plus_token,
      bang_token,
      false_token
    ]);

    assert_eq!(representation, "(+ nil (! false))")
  }

  #[test]
  fn double_bang_test() {
    let bang_token = Token::new(TokenKind::Bang, 1);
    let false_token = Token::new(TokenKind::False, 1);

    let representation = parse_and_print(vec![
      bang_token.clone(),
      bang_token,
      false_token
    ]);

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


    let representation = parse_and_print(vec![
      n1,
      star,
      n2,
      slash,
      n3
    ]);

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


    let representation = parse_and_print(vec![
      n1,
      plus,
      n2,
      minus,
      n3
    ]);

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


    let representation = parse_and_print(vec![
      n1,
      equal_equal,
      n2,
      bang_equal,
      n3
    ]);

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


    let representation = parse_and_print(vec![
      n1,
      less,
      n2,
      greater,
      n3
    ]);

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
    assert_eq!(res.unwrap_err(), ParseError::MalformedExpression(1, "Missing closing parenthesis".to_string()));
  }

  #[test]
  fn a_missing_term_after_a_plus_returns_error() {
    let tokens = vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::Eof, 1)
    ];

    let parser = parser(tokens);
    let res = parser.parse();

    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), ParseError::MalformedExpression(1, "Unexpected end of file".to_string()));
  }
}

