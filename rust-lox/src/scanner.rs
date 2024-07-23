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

    // If the next character in the input matches the specified character,
    // consumte it and return true; othrwise return false.
    fn match_char(&mut self, to_match: char) -> bool {
        if let Some(next_char) = self.peek_char() {
            if next_char == to_match {
                self.read_char();
                return true;
            }
        }

        false
    }

    // Consume white space characters.
    fn skip_white_space(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }

            self.read_char();
        }
    }

    // Intended to process single line comments.
    fn skip_rest_of_line(&mut self) {
        while let Some(next_char) = self.peek_char() {
            self.read_char();
            if next_char == '\n' {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_white_space();

        let token = match self.current_char {
            // Some tokens correspond to a single character
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            Some('{') => Some(Token::LeftBrace),
            Some('}') => Some(Token::RightBrace),
            Some(',') => Some(Token::Comma),
            Some('.') => Some(Token::Dot),
            Some('-') => Some(Token::Minus),
            Some('+') => Some(Token::Plus),
            Some(';') => Some(Token::Semicolon),
            Some('*') => Some(Token::Star),

            // Some characters can correspond to a single token, or be the initial character of a two character sequence
            Some('!') => {
                if self.match_char('=') {
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Bang)
                }
            }
            Some('=') => {
                if self.match_char('=') {
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            }
            Some('>') => {
                if self.match_char('=') {
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            }
            Some('<') => {
                if self.match_char('=') {
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            }

            // Single line comments are proceeded by double slash
            Some('/') => {
                if self.match_char('/') {
                    self.skip_rest_of_line();
                    None
                } else {
                    Some(Token::Slash)
                }
            }

            _ => None,
        };

        token
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
