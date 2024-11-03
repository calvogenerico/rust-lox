use std::io::Read;
use utf8_read::{Char, Reader};
use crate::lox_token::LoxToken;

pub struct Scanner<'r, R: Read> {
    input: Reader<&'r mut R>,
    tokens: Vec<LoxToken>,
    peeked: Option<char>,
}

fn reserved_words(input: &str) -> Option<LoxToken> {
    match input {
        "and" => Some(LoxToken::And),
        "class" => Some(LoxToken::Class),
        "else" => Some(LoxToken::Else),
        "false" => Some(LoxToken::False),
        "fun" => Some(LoxToken::Fun),
        "for" => Some(LoxToken::For),
        "if" => Some(LoxToken::If),
        "nil" => Some(LoxToken::Nil),
        "or" => Some(LoxToken::Or),
        "print" => Some(LoxToken::Print),
        "return" => Some(LoxToken::Return),
        "super" => Some(LoxToken::Super),
        "this" => Some(LoxToken::This),
        "true" => Some(LoxToken::True),
        "var" => Some(LoxToken::Var),
        "while" => Some(LoxToken::While),
        _ => None
    }
}

impl<'r, R: Read> Scanner<'r, R> {
    pub fn new(read: &'r mut R) -> Scanner<'r, R> {
        Scanner {
            input: Reader::new(read),
            tokens: vec![],
            peeked: None,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<LoxToken> {
        while !self.eof() {
            let next_char = self.take_char();
            if next_char.is_some() {
                self.scan_char(next_char.unwrap())
            };
        }

        self.tokens.push(LoxToken::Eof);

        self.tokens.clone()
    }

    fn eof(&self) -> bool {
        self.input.eof()
    }

    fn scan_char(&mut self, a_char: char) {
        match a_char {
            '(' => self.tokens.push(LoxToken::LeftParen),
            ')' => self.tokens.push(LoxToken::RightParen),
            '{' => self.tokens.push(LoxToken::LeftBrace),
            '}' => self.tokens.push(LoxToken::RightBrace),
            ',' => self.tokens.push(LoxToken::Comma),
            '.' => self.tokens.push(LoxToken::Dot),
            '-' => self.tokens.push(LoxToken::Minus),
            '+' => self.tokens.push(LoxToken::Plus),
            ';' => self.tokens.push(LoxToken::Semicolon),
            '/' => self.tokens.push(LoxToken::Slash),
            '*' => self.tokens.push(LoxToken::Star),
            '!' => self.scan_maybe_two_chars(LoxToken::Bang, LoxToken::BangEqual),
            '=' => self.scan_maybe_two_chars(LoxToken::Equal, LoxToken::EqualEqual),
            '>' => self.scan_maybe_two_chars(LoxToken::Greater, LoxToken::GreaterEqual),
            '<' => self.scan_maybe_two_chars(LoxToken::Less, LoxToken::LessEqual),
            '"' => self.scan_string(),
            ' ' => {}
            '\n' => {}
            '\r' => {}
            a_char => {
                if a_char.is_digit(10) {
                    self.scan_number(a_char);
                } else if a_char.is_alphabetic() {
                    self.scan_identifier(a_char);
                } else {
                    panic!("not implemented")
                }
            }
        }
    }

    fn scan_identifier(&mut self, a_char: char) {
        let mut buf = String::from(a_char);
        self.take_following_alphanumeric(&mut buf);
        let token = reserved_words(&buf).unwrap_or(LoxToken::Identifier(buf));
        self.tokens.push(token);
    }

    fn scan_string(&mut self) {
        let buf = self.take_chars_until('"');
        self.tokens.push(LoxToken::String(buf.unwrap()));
    }

    fn scan_number(&mut self, a_char: char) {
        let mut buf = String::from(a_char);
        self.take_following_digits(&mut buf);

        if self.peek_char().is_some_and(|p| p == '.') {
            buf.push(self.take_char().unwrap());
            self.take_following_digits(&mut buf);
        }

        self.tokens.push(LoxToken::Numeric(buf.parse().unwrap()));
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
            let maybe_digit = peeked.filter(|a| a.is_alphanumeric());
            if let Some(digit) = maybe_digit {
                self.take_char();
                buf.push(digit)
            } else {
                break;
            }
        }
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
        if self.peeked.is_some() {
            return self.peeked.take();
        }

        match self.input.next_char() {
            Ok(Char::Char(res)) => Some(res),
            _ => None
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.peeked.is_some() {
            return self.peeked.clone();
        }

        let next_char = self.take_char();
        self.peeked.replace(next_char?);
        self.peeked.clone()
    }

    fn scan_maybe_two_chars(&mut self, token1: LoxToken, token2: LoxToken) {
        if self.peek_char().is_some_and(|c| c == '=') {
            self.take_char();
            self.tokens.push(token2);
        } else {
            self.tokens.push(token1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor};
    use super::*;

    fn scan_program(code: &str) -> Vec<LoxToken> {
        let program = String::from(code);
        let mut cursor = Cursor::new(program);
        let mut scan = Scanner::new(&mut cursor);
        scan.scan_tokens()
    }

    #[test]
    fn test_left_paren() {
        let tokens = scan_program("(");
        assert_eq!(tokens, vec![LoxToken::LeftParen, LoxToken::Eof]);
    }

    #[test]
    fn test_right_paren() {
        let tokens = scan_program(")");
        assert_eq!(tokens, vec![LoxToken::RightParen, LoxToken::Eof]);
    }

    #[test]
    fn test_left_brace() {
        let tokens = scan_program("{");
        assert_eq!(tokens, vec![LoxToken::LeftBrace, LoxToken::Eof]);
    }

    #[test]
    fn test_right_brace() {
        let tokens = scan_program("}");
        assert_eq!(tokens, vec![LoxToken::RightBrace, LoxToken::Eof]);
    }

    #[test]
    fn test_comma() {
        let tokens = scan_program(",");
        assert_eq!(tokens, vec![LoxToken::Comma, LoxToken::Eof]);
    }

    #[test]
    fn test_dot() {
        let tokens = scan_program(".");
        assert_eq!(tokens, vec![LoxToken::Dot, LoxToken::Eof]);
    }

    #[test]
    fn test_minus() {
        let tokens = scan_program("-");
        assert_eq!(tokens, vec![LoxToken::Minus, LoxToken::Eof]);
    }

    #[test]
    fn test_plus() {
        let tokens = scan_program("+");
        assert_eq!(tokens, vec![LoxToken::Plus, LoxToken::Eof]);
    }

    #[test]
    fn test_semicolon() {
        let tokens = scan_program(";");
        assert_eq!(tokens, vec![LoxToken::Semicolon, LoxToken::Eof]);
    }

    #[test]
    fn test_slash() {
        let tokens = scan_program("/");
        assert_eq!(tokens, vec![LoxToken::Slash, LoxToken::Eof]);
    }

    #[test]
    fn test_star() {
        let tokens = scan_program("*");
        assert_eq!(tokens, vec![LoxToken::Star, LoxToken::Eof]);
    }

    #[test]
    fn test_bang() {
        let tokens = scan_program("!");
        assert_eq!(tokens, vec![LoxToken::Bang, LoxToken::Eof]);
    }

    #[test]
    fn test_bang_equal() {
        let tokens = scan_program("!=");
        assert_eq!(tokens, vec![LoxToken::BangEqual, LoxToken::Eof]);
    }

    #[test]
    fn test_less_than() {
        let tokens = scan_program("<");
        assert_eq!(tokens, vec![LoxToken::Less, LoxToken::Eof]);
    }

    #[test]
    fn test_less_equal() {
        let tokens = scan_program("<=");
        assert_eq!(tokens, vec![LoxToken::LessEqual, LoxToken::Eof]);
    }

    #[test]
    fn triple_equals_creates_one_double_equal_and_then_a_simple_equal() {
        let tokens = scan_program("===");
        assert_eq!(tokens, vec![LoxToken::EqualEqual, LoxToken::Equal, LoxToken::Eof]);
    }

    #[test]
    fn bang_bang_equal_gets_bang_bang_equal() {
        let tokens = scan_program("!!=");
        assert_eq!(tokens, vec![LoxToken::Bang, LoxToken::BangEqual, LoxToken::Eof]);
    }

    #[test]
    fn only_number_one_returns_digit_1() {
        let tokens = scan_program("1");
        assert_eq!(tokens, vec![LoxToken::Numeric(1.0), LoxToken::Eof]);
    }

    #[test]
    fn nine_nine_one_returns_digit_99() {
        let tokens = scan_program("99");
        assert_eq!(tokens, vec![LoxToken::Numeric(99.0), LoxToken::Eof]);
    }

    #[test]
    fn nine_nine_dot_1_one_returns_digit_99_dot_1() {
        let tokens = scan_program("99.1");
        assert_eq!(tokens, vec![LoxToken::Numeric(99.1), LoxToken::Eof]);
    }

    #[test]
    fn nine_nine_dot_returns_digit_99_dot_0() {
        let tokens = scan_program("99.");
        assert_eq!(tokens, vec![LoxToken::Numeric(99.0), LoxToken::Eof]);
    }


    #[test]
    fn dot_nine_nine_returns_digit_0_dot_99() {
        let tokens = scan_program(".99");
        assert_eq!(tokens, vec![LoxToken::Dot, LoxToken::Numeric(99.0), LoxToken::Eof]);
    }

    #[test]
    fn white_spaces_are_ignored() {
        let tokens = scan_program("( )");
        assert_eq!(tokens, vec![LoxToken::LeftParen, LoxToken::RightParen, LoxToken::Eof]);
        let tokens = scan_program(" ");
        assert_eq!(tokens, vec![LoxToken::Eof]);
    }

    #[test]
    fn new_lines_do_not_produce_any_token() {
        let tokens = scan_program("(\n)");
        assert_eq!(tokens, vec![LoxToken::LeftParen, LoxToken::RightParen, LoxToken::Eof]);
        let tokens = scan_program("\n");
        assert_eq!(tokens, vec![LoxToken::Eof]);
    }

    #[test]
    fn windows_new_lines_do_not_produce_any_token() {
        let tokens = scan_program("(\r\n)");
        assert_eq!(tokens, vec![LoxToken::LeftParen, LoxToken::RightParen, LoxToken::Eof]);
        let tokens = scan_program("\r\n");
        assert_eq!(tokens, vec![LoxToken::Eof]);
    }

    #[test]
    fn string_test() {
        let tokens = scan_program("\"foo\"");
        assert_eq!(tokens, vec![LoxToken::String("foo".to_string()), LoxToken::Eof]);
    }

    #[test]
    fn string_can_have_any_character_inside() {
        let string_content = "(){}\\+-.,;: \n \r 123 asd ";
        let tokens = scan_program(&format!("\"{}\"", string_content));
        assert_eq!(tokens, vec![LoxToken::String(string_content.to_string()), LoxToken::Eof]);
    }

    #[test]
    fn identifier_test() {
        let tokens = scan_program("holu");
        assert_eq!(tokens, vec![LoxToken::Identifier("holu".to_string()), LoxToken::Eof]);
    }

    #[test]
    fn and_test() {
        let tokens = scan_program("and");
        assert_eq!(tokens, vec![LoxToken::And, LoxToken::Eof]);
    }

    #[test]
    fn class_test() {
        let tokens = scan_program("class");
        assert_eq!(tokens, vec![LoxToken::Class, LoxToken::Eof]);
    }

    #[test]
    fn else_test() {
        let tokens = scan_program("else");
        assert_eq!(tokens, vec![LoxToken::Else, LoxToken::Eof]);
    }

    #[test]
    fn false_test() {
        let tokens = scan_program("false");
        assert_eq!(tokens, vec![LoxToken::False, LoxToken::Eof]);
    }

    #[test]
    fn fun_test() {
        let tokens = scan_program("fun");
        assert_eq!(tokens, vec![LoxToken::Fun, LoxToken::Eof]);
    }

    #[test]
    fn for_test() {
        let tokens = scan_program("for");
        assert_eq!(tokens, vec![LoxToken::For, LoxToken::Eof]);
    }

    #[test]
    fn if_test() {
        let tokens = scan_program("if");
        assert_eq!(tokens, vec![LoxToken::If, LoxToken::Eof]);
    }

    #[test]
    fn nil_test() {
        let tokens = scan_program("nil");
        assert_eq!(tokens, vec![LoxToken::Nil, LoxToken::Eof]);
    }

    #[test]
    fn or_test() {
        let tokens = scan_program("or");
        assert_eq!(tokens, vec![LoxToken::Or, LoxToken::Eof]);
    }

    #[test]
    fn print_test() {
        let tokens = scan_program("print");
        assert_eq!(tokens, vec![LoxToken::Print, LoxToken::Eof]);
    }

    #[test]
    fn return_test() {
        let tokens = scan_program("return");
        assert_eq!(tokens, vec![LoxToken::Return, LoxToken::Eof]);
    }

    #[test]
    fn super_test() {
        let tokens = scan_program("super");
        assert_eq!(tokens, vec![LoxToken::Super, LoxToken::Eof]);
    }

    #[test]
    fn this_test() {
        let tokens = scan_program("this");
        assert_eq!(tokens, vec![LoxToken::This, LoxToken::Eof]);
    }

    #[test]
    fn true_test() {
        let tokens = scan_program("true");
        assert_eq!(tokens, vec![LoxToken::True, LoxToken::Eof]);
    }

    #[test]
    fn var_test() {
        let tokens = scan_program("var");
        assert_eq!(tokens, vec![LoxToken::Var, LoxToken::Eof]);
    }

    #[test]
    fn while_test() {
        let tokens = scan_program("while");
        assert_eq!(tokens, vec![LoxToken::While, LoxToken::Eof]);
    }
}