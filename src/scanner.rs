use std::io::{Read};
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

        while !utf8.eof() {
            if let Ok(Char::Char(a_char)) = utf8.next_char() {
                self.scan_char(a_char, &mut utf8)
            }
        }

        self.tokens.push(LoxToken::Eof);

        self.tokens.clone()
    }

    fn scan_char<R: Read>(&mut self, a_char: char, mut rem: &mut Reader<R>) {
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
            '!' => {
                let followed_by_equals = matches!(
                    rem.peekable().peek(),
                    Some(Ok('='))
                );

                if followed_by_equals {
                    rem.next();
                    self.tokens.push(LoxToken::BangEqual);
                } else {
                    self.tokens.push(LoxToken::Bang)
                }
            },
            another => println!("another: {}", another)
        }
    }
}