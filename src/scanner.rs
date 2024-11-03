use std::fmt::{Display, Formatter};
use std::io::{BufReader, Read};
use utf8_read::{Char, Reader};

pub struct Scanner {
    tokens: Vec<LoxToken>,
}


#[derive(Clone)]
pub enum LoxToken {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Eof,
}

impl LoxToken {
    pub fn to_str(&self) -> &str {
        match self {
            LoxToken::LeftParen => "LEFT_PAREN ( null",
            LoxToken::RightParen => "RIGHT_PAREN ) null",
            LoxToken::LeftBrace => "LEFT_BRACE { null",
            LoxToken::RightBrace => "RIGHT_BRACE } null",
            LoxToken::Eof => "EOF null"
        }
    }
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            tokens: vec![]
        }
    }

    pub fn scan_tokens<R: Read>(&mut self, reader: R) -> Vec<LoxToken> {
        let mut utf8 = Reader::new(reader);
        let iter = utf8.into_iter();

        for char in iter {
            self.scan_char(char.unwrap())
        }

        self.tokens.push(LoxToken::Eof);

        self.tokens.clone()
    }

    fn scan_char(&mut self, a_char: char) {
        match a_char {
            '(' => self.tokens.push(LoxToken::LeftParen),
            ')' => self.tokens.push(LoxToken::RightParen),
            '{' => self.tokens.push(LoxToken::RightBrace),
            '}' => self.tokens.push(LoxToken::RightBrace),
            _ => println!("another")
        }
    }
}