use std::io::{Read};
use utf8_read::{Char, Reader};

pub struct Scanner<'r, R: Read> {
    input: Reader<&'r mut R>,
    tokens: Vec<LoxToken>,
    peeked: Option<char>
}


#[derive(Clone)]
pub enum LoxToken {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or Two tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,

    Eof,
}

impl LoxToken {
    pub fn to_str(&self) -> &str {
        match self {
            LoxToken::LeftParen => "LEFT_PAREN ( null",
            LoxToken::RightParen => "RIGHT_PAREN ) null",
            LoxToken::LeftBrace => "LEFT_BRACE { null",
            LoxToken::RightBrace => "RIGHT_BRACE } null",
            LoxToken::Comma => "COMMA , null",
            LoxToken::Dot => "DOT . null",
            LoxToken::Minus => "MINUS - null",
            LoxToken::Plus => "PLUS + null",
            LoxToken::Semicolon => "SEMICOLON ; null",
            LoxToken::Slash => "SLASH / null",
            LoxToken::Star => "STAR * null",
            LoxToken::Bang => "BANG ! null",
            LoxToken::BangEqual => "BANG_EQUAL ! null",
            LoxToken::Equal => "EQUAL = null",
            LoxToken::EqualEqual => "EQUAL_EQUAL == null",
            LoxToken::Eof => "EOF null",
        }
    }
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
            self.tokens.push(token2)
        } else {
            self.tokens.push(token1)
        }
    }
}