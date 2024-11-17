use std::io::{Read};
use utf8_read::{Char, Reader};
use crate::scan::token::Token;
use crate::scan::token_kind::TokenKind;

pub struct Scanner<'r, R: Read> {
  input: Reader<&'r mut R>,
  tokens: Vec<Token>,
  peeked: Option<char>,
  current_line: usize,
  errors: Vec<String>,
}

fn reserved_words(input: &str) -> Option<TokenKind> {
  match input {
    "and" => Some(TokenKind::And),
    "class" => Some(TokenKind::Class),
    "else" => Some(TokenKind::Else),
    "false" => Some(TokenKind::False),
    "fun" => Some(TokenKind::Fun),
    "for" => Some(TokenKind::For),
    "if" => Some(TokenKind::If),
    "nil" => Some(TokenKind::Nil),
    "or" => Some(TokenKind::Or),
    "print" => Some(TokenKind::Print),
    "return" => Some(TokenKind::Return),
    "super" => Some(TokenKind::Super),
    "this" => Some(TokenKind::This),
    "true" => Some(TokenKind::True),
    "var" => Some(TokenKind::Var),
    "while" => Some(TokenKind::While),
    _ => None
  }
}

impl<'r, R: Read> Scanner<'r, R> {
  pub fn new(read: &'r mut R) -> Scanner<'r, R> {
    Scanner {
      input: Reader::new(read),
      tokens: vec![],
      peeked: None,
      current_line: 1,
      errors: vec![],
    }
  }

  pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<String>> {
    while !self.eof() {
      let next_char = self.take_char();
      if next_char.is_some() {
        self.scan_char(next_char.unwrap())
      };
    }

    self.push_token_current_line(TokenKind::Eof);
    
    if self.errors.len() > 0 {
      Err(self.errors)
    } else {
      Ok(self.tokens)
    }
  }

  fn eof(&self) -> bool {
    self.input.eof()
  }

  fn scan_char(&mut self, a_char: char) {
    match a_char {
      '(' => self.push_token_current_line(TokenKind::LeftParen),
      ')' => self.push_token_current_line(TokenKind::RightParen),
      '{' => self.push_token_current_line(TokenKind::LeftBrace),
      '}' => self.push_token_current_line(TokenKind::RightBrace),
      ',' => self.push_token_current_line(TokenKind::Comma),
      '.' => self.push_token_current_line(TokenKind::Dot),
      '-' => self.push_token_current_line(TokenKind::Minus),
      '+' => self.push_token_current_line(TokenKind::Plus),
      ';' => self.push_token_current_line(TokenKind::Semicolon),
      '/' => self.scan_slash_or_comment(),
      '*' => self.push_token_current_line(TokenKind::Star),
      '!' => self.scan_maybe_two_chars(TokenKind::Bang, TokenKind::BangEqual),
      '=' => self.scan_maybe_two_chars(TokenKind::Equal, TokenKind::EqualEqual),
      '>' => self.scan_maybe_two_chars(TokenKind::Greater, TokenKind::GreaterEqual),
      '<' => self.scan_maybe_two_chars(TokenKind::Less, TokenKind::LessEqual),
      '"' => self.scan_string(),
      ' ' => {}
      '\n' => {}
      '\r' => {}
      '\t' => {}
      a_char => {
        if a_char.is_digit(10) {
          self.scan_number(a_char);
        } else if Self::char_is_alphanumeric(&a_char) {
          self.scan_identifier(a_char);
        } else {
          self.scan_unexpected_character(a_char);
        }
      }
    }
  }

  fn push_token_current_line(&mut self, kind: TokenKind) {
    self.push_token_at(kind, self.current_line)
  }

  fn push_token_at(&mut self, kind: TokenKind, line_number: usize) {
    self.tokens.push(Token::new(kind, line_number))
  }

  fn scan_slash_or_comment(&mut self) {
    let next = self.peek_char();

    if next.is_some_and(|n| n == '/') {
      self.take_chars_until('\n');
    } else {
      self.push_token_current_line(TokenKind::Slash);
    }
  }

  fn scan_unexpected_character(&mut self, a_char: char) {
    let error = format!("[line {}] Error: Unexpected character: {}", self.current_line, a_char);
    self.errors.push(error);
  }

  fn scan_identifier(&mut self, a_char: char) {
    let mut buf = String::from(a_char);
    self.take_following_alphanumeric(&mut buf);
    let token = reserved_words(&buf).unwrap_or(TokenKind::Identifier(buf));
    self.push_token_current_line(token);
  }

  fn scan_string(&mut self) {
    let start = self.current_line;
    if let Some(content) = self.take_chars_until('"') {
      self.push_token_at(TokenKind::String(content), start);
    } else {
      self.errors.push(format!("[line {start}] Error: Unterminated string."));
    }
  }

  fn scan_number(&mut self, a_char: char) {
    let mut numerical_str = String::from(a_char);
    self.take_following_digits(&mut numerical_str);

    if self.peek_char().is_some_and(|p| p == '.') {
      numerical_str.push(self.take_char().unwrap());
      self.take_following_digits(&mut numerical_str);
    }

    self.push_token_current_line(TokenKind::Number(numerical_str));
  }

  fn take_following_digits(&mut self, buf: &mut String) {
    loop {
      let peeked = self.peek_char();
      let maybe_digit = peeked.filter(|a| a.is_digit(10));
      if let Some(digit) = maybe_digit {
        self.take_char();
        buf.push(digit)
      } else {
        break;
      }
    }
  }

  fn take_following_alphanumeric(&mut self, buf: &mut String) {
    loop {
      let peeked = self.peek_char();
      let maybe_digit = peeked.filter(|a| Self::char_is_alphanumeric(a));
      if let Some(digit) = maybe_digit {
        self.take_char();
        buf.push(digit)
      } else {
        break;
      }
    }
  }

  fn char_is_alphanumeric(a: &char) -> bool {
    a.is_alphanumeric() || *a == '_'
  }

  fn take_chars_until(&mut self, limit: char) -> Option<String> {
    let mut buf = String::new();
    loop {
      let taken = self.take_char()?;
      if taken == limit {
        break;
      } else {
        buf.push(taken)
      }
    }
    Some(buf)
  }

  fn take_char(&mut self) -> Option<char> {
    let next_char = self.peeked.take().or_else(|| {
      match self.input.next_char() {
        Ok(Char::Char(res)) => Some(res),
        _ => None
      }
    });

    if next_char.is_some_and(|c| c == '\n') {
      self.current_line += 1
    }

    next_char
  }

  fn peek_char(&mut self) -> Option<char> {
    if self.peeked.is_some() {
      return self.peeked.clone();
    }

    let next_char = self.take_char();
    self.peeked.replace(next_char?);
    self.peeked.clone()
  }

  fn scan_maybe_two_chars(&mut self, token1: TokenKind, token2: TokenKind) {
    if self.peek_char().is_some_and(|c| c == '=') {
      self.take_char();
      self.push_token_current_line(token2);
    } else {
      self.push_token_current_line(token1);
    }
  }
}

#[cfg(test)]
mod tests {
  use std::io::{Cursor};
  use super::*;

  fn scan_program_clean(code: &str) -> Vec<Token> {
    let program = String::from(code);
    let mut cursor = Cursor::new(program);
    let scan = Scanner::new(&mut cursor);
    let tokens = scan.scan_tokens().unwrap();
    
    tokens
  }

  fn scan_program_with_errors(code: &str) -> Vec<String> {
    let program = String::from(code);
    let mut cursor = Cursor::new(program);
    let scan = Scanner::new(&mut cursor);
    scan.scan_tokens().unwrap_err()
  }


  #[test]
  fn test_left_paren() {
    let tokens = scan_program_clean("(");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::LeftParen, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_right_paren() {
    let tokens = scan_program_clean(")");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::RightParen, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_left_brace() {
    let tokens = scan_program_clean("{");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::LeftBrace, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_right_brace() {
    let tokens = scan_program_clean("}");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::RightBrace, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_comma() {
    let tokens = scan_program_clean(",");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Comma, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_dot() {
    let tokens = scan_program_clean(".");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Dot, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_minus() {
    let tokens = scan_program_clean("-");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Minus, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_plus() {
    let tokens = scan_program_clean("+");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Plus, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_semicolon() {
    let tokens = scan_program_clean(";");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Semicolon, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_slash() {
    let tokens = scan_program_clean("/");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Slash, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_star() {
    let tokens = scan_program_clean("*");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Star, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_bang() {
    let tokens = scan_program_clean("!");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_bang_equal() {
    let tokens = scan_program_clean("!=");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::BangEqual, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_less_than() {
    let tokens = scan_program_clean("<");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Less, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn test_less_equal() {
    let tokens = scan_program_clean("<=");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::LessEqual, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn triple_equals_creates_one_double_equal_and_then_a_simple_equal() {
    let tokens = scan_program_clean("===");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::EqualEqual, 1),
      Token::new(TokenKind::Equal, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn bang_bang_equal_gets_bang_bang_equal() {
    let tokens = scan_program_clean("!!=");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Bang, 1),
      Token::new(TokenKind::BangEqual, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn only_number_one_returns_digit_1() {
    let tokens = scan_program_clean("1");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Number("1".to_string()), 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn nine_nine_one_returns_digit_99() {
    let tokens = scan_program_clean("99");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Number("99".to_string()), 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn nine_nine_dot_1_one_returns_digit_99_dot_1() {
    let tokens = scan_program_clean("99.1");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Number("99.1".to_string()), 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn nine_nine_dot_returns_digit_99_dot_0() {
    let tokens = scan_program_clean("99.");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Number("99.".to_string()), 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }


  #[test]
  fn dot_nine_nine_returns_token_dot_and_token_digit_99() {
    let tokens = scan_program_clean(".99");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Dot, 1),
      Token::new(TokenKind::Number("99".to_string()), 1),
      Token::new(TokenKind::Eof, 1),
    ]);
  }

  #[test]
  fn white_spaces_are_ignored() {
    let tokens = scan_program_clean("( )");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::LeftParen, 1),
      Token::new(TokenKind::RightParen, 1),
      Token::new(TokenKind::Eof, 1),
    ]);
    let tokens = scan_program_clean(" ");
    assert_eq!(tokens, vec![Token::new(TokenKind::Eof, 1), ]);
  }

  #[test]
  fn new_lines_do_not_produce_any_token() {
    let tokens = scan_program_clean("(\n)");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::LeftParen, 1),
      Token::new(TokenKind::RightParen, 2),
      Token::new(TokenKind::Eof, 2),
    ]);
    let tokens = scan_program_clean("\n");
    assert_eq!(tokens, vec![Token::new(TokenKind::Eof, 2), ]);
  }

  #[test]
  fn windows_new_lines_do_not_produce_any_token() {
    let tokens = scan_program_clean("(\r\n)");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::LeftParen, 1),
      Token::new(TokenKind::RightParen, 2),
      Token::new(TokenKind::Eof, 2),
    ]);
    let tokens = scan_program_clean("\r\n");
    assert_eq!(tokens, vec![Token::new(TokenKind::Eof, 2)]);
  }


  #[test]
  fn string_test() {
    let tokens = scan_program_clean("\"foo\"");

    assert_eq!(tokens, vec![
      Token::new(TokenKind::String("foo".to_string()), 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn string_can_have_any_character_inside() {
    let string_content = "(){}\\+-.,;: \n \r 123 asd ";
    let tokens = scan_program_clean(&format!("\"{}\"", string_content));
    assert_eq!(tokens, vec![
      Token::new(TokenKind::String(string_content.to_string()), 1),
      Token::new(TokenKind::Eof, 2)
    ]);
  }

  #[test]
  fn identifier_test() {
    let tokens = scan_program_clean("holu");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Identifier("holu".to_string()), 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn and_test() {
    let tokens = scan_program_clean("and");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::And, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn class_test() {
    let tokens = scan_program_clean("class");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Class, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn else_test() {
    let tokens = scan_program_clean("else");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Else, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn false_test() {
    let tokens = scan_program_clean("false");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::False, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn fun_test() {
    let tokens = scan_program_clean("fun");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Fun, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn for_test() {
    let tokens = scan_program_clean("for");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::For, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn if_test() {
    let tokens = scan_program_clean("if");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::If, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn nil_test() {
    let tokens = scan_program_clean("nil");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Nil, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn or_test() {
    let tokens = scan_program_clean("or");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Or, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn print_test() {
    let tokens = scan_program_clean("print");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Print, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn return_test() {
    let tokens = scan_program_clean("return");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Return, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn super_test() {
    let tokens = scan_program_clean("super");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Super, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn this_test() {
    let tokens = scan_program_clean("this");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::This, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn true_test() {
    let tokens = scan_program_clean("true");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::True, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn var_test() {
    let tokens = scan_program_clean("var");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Var, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn while_test() {
    let tokens = scan_program_clean("while");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::While, 1),
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn eof_test() {
    let tokens = scan_program_clean("");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn dollar_sign_produces_an_error() {
    let errors = scan_program_with_errors("$");

    assert_eq!(errors, vec!["[line 1] Error: Unexpected character: $"])
  }


  #[test]
  fn errors_track_line_number() {
    let errors = scan_program_with_errors("\n@");

    assert_eq!(errors, vec!["[line 2] Error: Unexpected character: @"])
  }

  #[test]
  fn only_one_commented_line_produce_nothing() {
    let tokens = scan_program_clean("// this is a comment");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn comments_can_include_unknown_characters() {
    let tokens = scan_program_clean("// $@#");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::Eof, 1)
    ]);
  }

  #[test]
  fn comments_end_at_the_end_if_the_line() {
    let tokens = scan_program_clean("(// $@#\n)");
    assert_eq!(tokens, vec![
      Token::new(TokenKind::LeftParen, 1),
      Token::new(TokenKind::RightParen, 2),
      Token::new(TokenKind::Eof, 2),
    ]);
  }

  #[test]
  fn string_not_terminated_produce_an_error() {
    let errors = scan_program_with_errors("\"bar\" \"unterminated");
    assert_eq!(errors, vec!["[line 1] Error: Unterminated string."]);
  }
}