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

    // Literals
    Numeric(f64),
    String(String),
    Identifier(String),

    Eof,
}

impl LoxToken {
    pub fn to_string(&self) -> String {
        match self {
            LoxToken::LeftParen => "LEFT_PAREN ( null".to_string(),
            LoxToken::RightParen => "RIGHT_PAREN ) null".to_string(),
            LoxToken::LeftBrace => "LEFT_BRACE { null".to_string(),
            LoxToken::RightBrace => "RIGHT_BRACE } null".to_string(),
            LoxToken::Comma => "COMMA , null".to_string(),
            LoxToken::Dot => "DOT . null".to_string(),
            LoxToken::Minus => "MINUS - null".to_string(),
            LoxToken::Plus => "PLUS + null".to_string(),
            LoxToken::Semicolon => "SEMICOLON ; null".to_string(),
            LoxToken::Slash => "SLASH / null".to_string(),
            LoxToken::Star => "STAR * null".to_string(),
            LoxToken::Bang => "BANG ! null".to_string(),
            LoxToken::BangEqual => "BANG_EQUAL ! null".to_string(),
            LoxToken::Equal => "EQUAL = null".to_string(),
            LoxToken::EqualEqual => "EQUAL_EQUAL == null".to_string(),
            LoxToken::Greater => "GREATER > null".to_string(),
            LoxToken::GreaterEqual => "GREATER_EQUAL >= null".to_string(),
            LoxToken::Less => "LESS > null".to_string(),
            LoxToken::LessEqual => "LESS_EQUAL >= null".to_string(),
            LoxToken::Numeric(value) => format!("IDENTIFIER {value} {value}"),
            LoxToken::String(value) => format!("IDENTIFIER \"{value}\" {value}"),
            LoxToken::Identifier(value) => format!("IDENTIFIER {value} null"),
            LoxToken::Eof => "EOF null".to_string(),
        }
    }
}