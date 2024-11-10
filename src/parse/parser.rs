use crate::parse::enum_expr::EnumExpr;
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

  pub fn parse(mut self) -> EnumExpr {
    self.expression().unwrap()
  }

  fn expression(&mut self) -> Result<EnumExpr, String> {
    self.equality()
  }

  fn equality(&mut self) -> Result<EnumExpr, String> {
    let left = self.comparison()?;

    if let Some(operator) = self.advance_if_match(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
      let right = self.comparison()?;
      return Ok(EnumExpr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) });
    }

    Ok(left)
  }

  fn comparison(&mut self) -> Result<EnumExpr, String> {
    let left = self.literal()?;

    if let Some(operator) = self.advance_if_match(
      &[TokenKind::Less, TokenKind::LessEqual, TokenKind::Greater, TokenKind::GreaterEqual]
    ) {
      let right = self.literal()?;
      return Ok(EnumExpr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) });
    }

    Ok(left)
  }

  fn literal(&mut self) -> Result<EnumExpr, String> {
    let maybe = self.next_token();
    let token = maybe.unwrap();

    let token = match token.kind() {
      TokenKind::Number(repr) => EnumExpr::LiteralNumber { value: repr.parse().unwrap() },
      TokenKind::True => EnumExpr::LiteralBool { value: true },
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
  fn parse_less_than_returns_correct_tree() {
    let one_token = Token::new(TokenKind::Number("1".to_string()), 1);
    let less_than_token = Token::new(TokenKind::Less, 1);
    let two_token = Token::new(TokenKind::Number("2".to_string()), 1);

    let parser = parser(vec![one_token, less_than_token, two_token]);
    let res = parser.parse();

    let visitor = PrintAst {};
    let representation = visitor.print(&res);

    assert_eq!(representation, "(< 1 2)");
  }

  #[test]
  fn parse_less_than_between_equal_equal_reeturns_correct_tree() {
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
}