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
    Number(String),
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
            LoxToken::BangEqual => "BANG_EQUAL != null".to_string(),
            LoxToken::Equal => "EQUAL = null".to_string(),
            LoxToken::EqualEqual => "EQUAL_EQUAL == null".to_string(),
            LoxToken::Greater => "GREATER > null".to_string(),
            LoxToken::GreaterEqual => "GREATER_EQUAL >= null".to_string(),
            LoxToken::Less => "LESS < null".to_string(),
            LoxToken::LessEqual => "LESS_EQUAL <= null".to_string(),
            LoxToken::Number(value) => format!("NUMBER {value} {:?}", value.parse::<f64>().unwrap()),
            LoxToken::String(value) => format!("STRING \"{value}\" {value}"),
            LoxToken::Identifier(value) => format!("IDENTIFIER {value} null"),
            LoxToken::And => "AND and null".to_string(),
            LoxToken::Class => "CLASS class null".to_string(),
            LoxToken::Else => "ELSE else null".to_string(),
            LoxToken::False => "FALSE false null".to_string(),
            LoxToken::Fun => "FUN fun null".to_string(),
            LoxToken::For => "FOR for null".to_string(),
            LoxToken::If => "IF if null".to_string(),
            LoxToken::Nil => "NIL nil null".to_string(),
            LoxToken::Or => "OR or null".to_string(),
            LoxToken::Print => "PRINT print null".to_string(),
            LoxToken::Return => "RETURN return null".to_string(),
            LoxToken::Super => "SUPER super null".to_string(),
            LoxToken::This => "THIS this null".to_string(),
            LoxToken::True => "TRUE true null".to_string(),
            LoxToken::Var => "VAR Var null".to_string(),
            LoxToken::While => "WHILE while null".to_string(),
            LoxToken::Eof => "EOF  null".to_string(), // The double space is on purpose. The representation is empty.
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

    #[test]
    fn bang_equal_to_string() {
        assert_eq!(&LoxToken::BangEqual.to_string(), "BANG_EQUAL != null")
    }

    #[test]
    fn less_to_string() {
        assert_eq!(&LoxToken::Less.to_string(), "LESS < null")
    }

    #[test]
    fn less_equal_to_string() {
        assert_eq!(&LoxToken::LessEqual.to_string(), "LESS_EQUAL <= null")
    }

    #[test]
    fn identifier_to_string() {
        assert_eq!(&LoxToken::Identifier("foo".to_string()).to_string(), "IDENTIFIER foo null")
    }

    #[test]
    fn string_to_string() {
        assert_eq!(&LoxToken::String("foo".to_string()).to_string(), "STRING \"foo\" foo")
    }

    #[test]
    fn number_to_string() {
        assert_eq!(&LoxToken::Number("47.0".to_string()).to_string(), "NUMBER 47.0 47.0")
    }

    #[test]
    fn number_to_string_integers() {
        assert_eq!(&LoxToken::Number("47".to_string()).to_string(), "NUMBER 47 47.0")
    }

    #[test]
    fn number_to_2_decimals() {
        assert_eq!(&LoxToken::Number("47.11".to_string()).to_string(), "NUMBER 47.11 47.11")
    }

    #[test]
    fn number_to_string_ending_with_dot() {
        assert_eq!(&LoxToken::Number("47.".to_string()).to_string(), "NUMBER 47. 47.0")
    }

    #[test]
    fn super_to_string() {
        assert_eq!(&LoxToken::Super.to_string(), "SUPER super null")
    }

    #[test]
    fn or_to_string() {
        assert_eq!(&LoxToken::Or.to_string(), "OR or null")
    }
}