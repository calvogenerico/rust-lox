#[derive(Debug, Clone, PartialEq)]
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
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

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
            LoxToken::Greater => "GREATER > null",
            LoxToken::GreaterEqual => "GREATER_EQUAL >= null",
            LoxToken::Less => "LESS > null",
            LoxToken::LessEqual => "LESS_EQUAL >= null",
            LoxToken::Eof => "EOF null",
        }
    }
}