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

    // Keywords
    And,
    Class,

    Eof,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
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
            LoxToken::And => "AND and null".to_string(),
            LoxToken::Class => "CLASS class null".to_string(),
            LoxToken::Else => "ELSE Else null".to_string(),
            LoxToken::False => "FALSE False null".to_string(),
            LoxToken::Fun => "FUN Fun null".to_string(),
            LoxToken::For => "FOR For null".to_string(),
            LoxToken::If => "IF If null".to_string(),
            LoxToken::Nil => "NIL Nil null".to_string(),
            LoxToken::Or => "OR Or null".to_string(),
            LoxToken::Print => "PRINT Print null".to_string(),
            LoxToken::Return => "RETURN Return null".to_string(),
            LoxToken::Super => "SUPER Super null".to_string(),
            LoxToken::This => "THIS This null".to_string(),
            LoxToken::True => "TRUE True null".to_string(),
            LoxToken::Var => "VAR Var null".to_string(),
            LoxToken::While => "WHILE While null".to_string(),
            LoxToken::Eof => "EOF  null".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eof_to_string() {
        assert_eq!(&LoxToken::Eof.to_string(), "EOF  null")
    }
}