#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<TokenValue>,
    pub line: u32,
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    String(String),
    Number(f64),
}

impl Token {
    pub fn create(token_type: TokenType, line: u32) -> Self {
        Token {
            token_type,
            value: None,
            line,
        }
    }

    pub fn create_with_string(token_type: TokenType, line: u32, value: String) -> Self {
        Token {
            token_type,
            value: Some(TokenValue::String(value)),
            line,
        }
    }

    pub fn create_with_number(token_type: TokenType, line: u32, value: f64) -> Self {
        Token {
            token_type,
            value: Some(TokenValue::Number(value)),
            line,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
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

    Eof,
}
