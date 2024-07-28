use std::collections::HashMap;

pub struct Scanner<'a> {
    input: &'a str,
    current_pos: usize,
    last_read_char: Option<char>,
    keyword_to_token: HashMap<&'static str, Token>,
}

impl<'a> Scanner<'a> {
    pub fn scan(input: &'a String) -> Vec<Token> {
        let mut scanner = Scanner::new(&input);
        let mut tokens = Vec::<Token>::new();

        while let Some(token) = scanner.next_token() {
            tokens.push(token);
        }
        tokens.push(Token::Eof);

        tokens
    }

    fn new(input: &'a String) -> Self {
        Scanner {
            input: input,
            current_pos: 0,
            last_read_char: None,
            keyword_to_token: Self::create_keyword_map(),
        }
    }

    // Consumes the next character in the input.
    // Updates current_position, next_position and current_char.
    fn read_char(&mut self) -> Option<char> {
        let next_char = self.peek_char();

        // Update current_pos. Char type has 4 bytes. The utf-8 representation can be 1-4 bytes.
        self.current_pos = self.current_pos + next_char.map_or(0, |c| c.len_utf8());
        self.last_read_char = next_char;
        next_char
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            None
        } else {
            self.input[self.current_pos..].chars().next()
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
        while let Some(c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }

            // Consume the whitespace character
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

        let token = match self.read_char() {
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

            Some('"') => self.read_string_literal(),

            // Single line comments are proceeded by double slash
            Some('/') => {
                if self.match_char('/') {
                    self.skip_rest_of_line();
                    None
                } else {
                    Some(Token::Slash)
                }
            }

            Some(c) if c.is_digit(10) => self.read_number(),

            Some(c) if c.is_ascii_alphabetic() => self.read_identifier_or_keyword(),

            _ => None,
        };

        token
    }

    // Assumes first digit has already been read.
    fn read_number(&mut self) -> Option<Token> {
        let mut number: Vec<char> = Vec::new();

        if let Some(first_digit) = self.last_read_char {
            //TODO is this a digit?
            number.push(first_digit);
        } else {
            //TODO signa. error
        }

        // TODO collapse this into a songle loop.
        // Read the leading digits
        while let Some(d) = self.peek_char() {
            if !d.is_digit(10) {
                break;
            }

            number.push(d);
            self.read_char();
        }

        // Read optional decimal point
        if let Some('.') = self.peek_char() {
            number.push('.');
            self.read_char();
        }

        // Read trailing digits
        while let Some(d) = self.peek_char() {
            if !d.is_digit(10) {
                break;
            }

            number.push(d);
            self.read_char();
        }

        Some(Token::Number(number.into_iter().collect()))
    }

    // Assumes the opening quote has already been consumed.
    fn read_string_literal(&mut self) -> Option<Token> {
        let mut string: Vec<char> = Vec::new();

        while let Some(character) = self.read_char() {
            if character == '"' {
                // this is the closing quote
                let string: String = string.into_iter().collect();
                return Some(Token::String(string));
            }

            string.push(character);
        }

        // TODO flag error - no terminating double quote
        None
    }

    fn read_identifier_or_keyword(&mut self) -> Option<Token> {
        let mut string: Vec<char> = Vec::new();

        if let Some(first_char) = self.last_read_char {
            string.push(first_char);
        } else {
            //TODO report error
            return None;
        }

        while let Some(next_char) = self.peek_char() {
            if next_char.is_ascii_alphanumeric() {
                self.read_char();
                string.push(next_char);
            } else {
                break;
            }
        }

        let string: String = string.iter().collect();

        // Check if this identifier is a keyword.
        if let Some(token) = self.keyword_to_token.get(string.as_str()) {
            return Some(token.clone());
        }

        Some(Token::Identifier(string))
    }

    // Produces a map that associated string litrals representing keywords to the associated Token.
    fn create_keyword_map() -> HashMap<&'static str, Token> {
        let mut map = HashMap::new();

        map.insert("&", Token::And);
        map.insert("class", Token::Class);
        map.insert("else", Token::Else);
        map.insert("false", Token::False);
        map.insert("fun", Token::Fun);
        map.insert("for", Token::For);
        map.insert("if", Token::If);
        map.insert("nil", Token::Nil);
        map.insert("or", Token::Or);
        map.insert("print", Token::Print);
        map.insert("return", Token::Return);
        map.insert("super", Token::Super);
        map.insert("this", Token::This);
        map.insert("true", Token::True);
        map.insert("var", Token::Var);
        map.insert("while", Token::While);

        map
    }

    // fn error(&mut self, line: u32, message: &str) {
    //     self.report(line, "", message);
    // }

    // fn report(&mut self, line: u32, location: &str, message: &str) {
    //     println!("[line {line}] Error {location} : {message}");
    //     self.haserror = true;
    // }
}

#[derive(Debug, Clone)]
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
