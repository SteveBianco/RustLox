use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    iter: Peekable<Chars<'a>>,
    has_error: bool,
}

pub enum Token {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl<'a> Scanner<'a> {
    pub fn scan(input: &'a String) -> Vec<Token> {
        let _scanner = Scanner::new(&input);
        vec![]
    }

    fn new(input: &'a String) -> Self {
        let iter = input.chars().peekable();
        Scanner {
            iter,
            has_error: false,
        }
    }

    fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, location: &str, message: &str) {
        println!("[line {line}] Error {location} : {message}");
        self.has_error = true;
    }
}
