use std::io::Read;
use utf8_read::{Char, Reader};
use crate::lox_token::LoxToken;

pub struct Scanner<'r, R: Read> {
    input: Reader<&'r mut R>,
    tokens: Vec<LoxToken>,
    peeked: Option<char>
}


impl <'r, R: Read> Scanner<'r, R> {
    pub fn new(read: &'r mut R) -> Scanner<'r, R> {
        Scanner {
            input: Reader::new(read),
            tokens: vec![],
            peeked: None
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
            _ => {}
        }
    }

    fn take_char(&mut self) -> Option<char> {
        if self.peeked.is_some() {
            return self.peeked.take()
        }

        match self.input.next_char() {
            Ok(Char::Char(res)) => Some(res),
            _ => None
        }
    }

    fn peek_char(&mut self) -> Option<char> {
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
mod tests{
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
        assert_eq!(tokens, vec![ LoxToken::LeftParen, LoxToken::Eof ]);
    }

    #[test]
    fn test_right_paren() {
        let tokens = scan_program(")");
        assert_eq!(tokens, vec![ LoxToken::RightParen, LoxToken::Eof ]);
    }

    #[test]
    fn test_left_brace() {
        let tokens = scan_program("{");
        assert_eq!(tokens, vec![ LoxToken::LeftBrace, LoxToken::Eof ]);
    }

    #[test]
    fn test_right_brace() {
        let tokens = scan_program("}");
        assert_eq!(tokens, vec![ LoxToken::RightBrace, LoxToken::Eof ]);
    }

    #[test]
    fn test_comma() {
        let tokens = scan_program(",");
        assert_eq!(tokens, vec![ LoxToken::Comma, LoxToken::Eof ]);
    }

    #[test]
    fn test_dot() {
        let tokens = scan_program(".");
        assert_eq!(tokens, vec![ LoxToken::Dot, LoxToken::Eof ]);
    }

    #[test]
    fn test_minus() {
        let tokens = scan_program("-");
        assert_eq!(tokens, vec![ LoxToken::Minus, LoxToken::Eof ]);
    }

    #[test]
    fn test_plus() {
        let tokens = scan_program("+");
        assert_eq!(tokens, vec![ LoxToken::Plus, LoxToken::Eof ]);
    }

    #[test]
    fn test_semicolon() {
        let tokens = scan_program(";");
        assert_eq!(tokens, vec![ LoxToken::Semicolon, LoxToken::Eof ]);
    }

    #[test]
    fn test_slash() {
        let tokens = scan_program("/");
        assert_eq!(tokens, vec![ LoxToken::Slash, LoxToken::Eof ]);
    }

    #[test]
    fn test_star() {
        let tokens = scan_program("*");
        assert_eq!(tokens, vec![ LoxToken::Star, LoxToken::Eof ]);
    }

    #[test]
    fn test_bang() {
        let tokens = scan_program("!");
        assert_eq!(tokens, vec![ LoxToken::Bang, LoxToken::Eof ]);
    }

    #[test]
    fn test_bang_equal() {
        let tokens = scan_program("!=");
        assert_eq!(tokens, vec![ LoxToken::BangEqual, LoxToken::Eof ]);
    }

    #[test]
    fn test_less_than() {
        let tokens = scan_program("<");
        assert_eq!(tokens, vec![ LoxToken::Less, LoxToken::Eof ]);
    }

    #[test]
    fn test_less_equal() {
        let tokens = scan_program("<=");
        assert_eq!(tokens, vec![ LoxToken::LessEqual, LoxToken::Eof ]);
    }

    #[test]
    fn triple_equals_creates_one_double_equal_and_then_a_simple_equal() {
        let tokens = scan_program("===");
        assert_eq!(tokens, vec![ LoxToken::EqualEqual, LoxToken::Equal, LoxToken::Eof ]);
    }

    #[test]
    fn bang_bang_equal_gets_bang_bang_equal() {
        let tokens = scan_program("!!=");
        assert_eq!(tokens, vec![ LoxToken::Bang, LoxToken::BangEqual, LoxToken::Eof ]);
    }
}