#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
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

impl TokenKind {
    pub fn to_string(&self) -> String {
        match self {
            TokenKind::LeftParen => "LEFT_PAREN ( null".to_string(),
            TokenKind::RightParen => "RIGHT_PAREN ) null".to_string(),
            TokenKind::LeftBrace => "LEFT_BRACE { null".to_string(),
            TokenKind::RightBrace => "RIGHT_BRACE } null".to_string(),
            TokenKind::Comma => "COMMA , null".to_string(),
            TokenKind::Dot => "DOT . null".to_string(),
            TokenKind::Minus => "MINUS - null".to_string(),
            TokenKind::Plus => "PLUS + null".to_string(),
            TokenKind::Semicolon => "SEMICOLON ; null".to_string(),
            TokenKind::Slash => "SLASH / null".to_string(),
            TokenKind::Star => "STAR * null".to_string(),
            TokenKind::Bang => "BANG ! null".to_string(),
            TokenKind::BangEqual => "BANG_EQUAL != null".to_string(),
            TokenKind::Equal => "EQUAL = null".to_string(),
            TokenKind::EqualEqual => "EQUAL_EQUAL == null".to_string(),
            TokenKind::Greater => "GREATER > null".to_string(),
            TokenKind::GreaterEqual => "GREATER_EQUAL >= null".to_string(),
            TokenKind::Less => "LESS < null".to_string(),
            TokenKind::LessEqual => "LESS_EQUAL <= null".to_string(),
            TokenKind::Number(value) => format!("NUMBER {value} {:?}", value.parse::<f64>().unwrap()),
            TokenKind::String(value) => format!("STRING \"{value}\" {value}"),
            TokenKind::Identifier(value) => format!("IDENTIFIER {value} null"),
            TokenKind::And => "AND and null".to_string(),
            TokenKind::Class => "CLASS class null".to_string(),
            TokenKind::Else => "ELSE else null".to_string(),
            TokenKind::False => "FALSE false null".to_string(),
            TokenKind::Fun => "FUN fun null".to_string(),
            TokenKind::For => "FOR for null".to_string(),
            TokenKind::If => "IF if null".to_string(),
            TokenKind::Nil => "NIL nil null".to_string(),
            TokenKind::Or => "OR or null".to_string(),
            TokenKind::Print => "PRINT print null".to_string(),
            TokenKind::Return => "RETURN return null".to_string(),
            TokenKind::Super => "SUPER super null".to_string(),
            TokenKind::This => "THIS this null".to_string(),
            TokenKind::True => "TRUE true null".to_string(),
            TokenKind::Var => "VAR var null".to_string(),
            TokenKind::While => "WHILE while null".to_string(),
            TokenKind::Eof => "EOF  null".to_string(), // The double space is on purpose. The representation is empty.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eof_to_string() {
        assert_eq!(&TokenKind::Eof.to_string(), "EOF  null")
    }

    #[test]
    fn bang_equal_to_string() {
        assert_eq!(&TokenKind::BangEqual.to_string(), "BANG_EQUAL != null")
    }

    #[test]
    fn less_to_string() {
        assert_eq!(&TokenKind::Less.to_string(), "LESS < null")
    }

    #[test]
    fn less_equal_to_string() {
        assert_eq!(&TokenKind::LessEqual.to_string(), "LESS_EQUAL <= null")
    }

    #[test]
    fn identifier_to_string() {
        assert_eq!(&TokenKind::Identifier("foo".to_string()).to_string(), "IDENTIFIER foo null")
    }

    #[test]
    fn string_to_string() {
        assert_eq!(&TokenKind::String("foo".to_string()).to_string(), "STRING \"foo\" foo")
    }

    #[test]
    fn number_to_string() {
        assert_eq!(&TokenKind::Number("47.0".to_string()).to_string(), "NUMBER 47.0 47.0")
    }

    #[test]
    fn number_to_string_integers() {
        assert_eq!(&TokenKind::Number("47".to_string()).to_string(), "NUMBER 47 47.0")
    }

    #[test]
    fn number_to_2_decimals() {
        assert_eq!(&TokenKind::Number("47.11".to_string()).to_string(), "NUMBER 47.11 47.11")
    }

    #[test]
    fn number_to_string_ending_with_dot() {
        assert_eq!(&TokenKind::Number("47.".to_string()).to_string(), "NUMBER 47. 47.0")
    }

    #[test]
    fn super_to_string() {
        assert_eq!(&TokenKind::Super.to_string(), "SUPER super null")
    }

    #[test]
    fn or_to_string() {
        assert_eq!(&TokenKind::Or.to_string(), "OR or null")
    }

    #[test]
    fn var_to_string() {
        assert_eq!(&TokenKind::Var.to_string(), "VAR var null")
    }
}