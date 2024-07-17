use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    input: &'a str,
    current_pos: usize,
    next_pos: usize,
    current_char: Option<char>,
    has_error: bool,
}

impl<'a> Scanner<'a> {
    pub fn scan(input: &'a String) -> Vec<Token> {
        let scanner = Scanner::new(&input);
        vec![]
    }

    fn new(input: &'a String) -> Self {
        Scanner {
            input: input,
            current_pos: 0,
            next_pos: 0,
            current_char: None,
            has_error: false,
        }
    }

    // Consumes the next character in the input.
    // Updates current_position, next_position and current_char.
    fn read_char(&mut self) {
        // Update current_char
        self.current_char = self.peek_char();

        // Consume the character by updating current_pos.
        self.current_pos = self.next_pos;

        // Update next_pos. Char type has 4 bytes. The utf-8 representation can be 1-4 bytes.
        self.next_pos = self.current_pos + self.current_char.map_or(0, |c| c.len_utf8());
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.next_pos >= self.input.len() {
            None
        } else {
            self.input[self.next_pos..].chars().next()
        }
    }

    // fn error(&mut self, line: u32, message: &str) {
    //     self.report(line, "", message);
    // }

    // fn report(&mut self, line: u32, location: &str, message: &str) {
    //     println!("[line {line}] Error {location} : {message}");
    //     self.haserror = true;
    // }
}

pub enum Token {
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
    Identifier(String),
    String(String),
    Number(String),

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
