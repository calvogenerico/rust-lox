use std::fmt::{Display};
use std::io::{Read};
use utf8_read::{Reader};

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
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
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
            LoxToken::Eof => "EOF null",
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
            '{' => self.tokens.push(LoxToken::LeftBrace),
            '}' => self.tokens.push(LoxToken::RightBrace),
            ',' => self.tokens.push(LoxToken::Comma),
            '.' => self.tokens.push(LoxToken::Dot),
            '-' => self.tokens.push(LoxToken::Minus),
            '+' => self.tokens.push(LoxToken::Plus),
            ';' => self.tokens.push(LoxToken::Semicolon),
            '/' => self.tokens.push(LoxToken::Slash),
            '*' => self.tokens.push(LoxToken::Star),
            _ => println!("another")
        }
    }
}