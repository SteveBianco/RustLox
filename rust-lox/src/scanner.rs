use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;

pub struct Scanner<'a> {
    input: &'a str,
    current_pos: usize,
    last_read_char: Option<char>,
    keyword_to_token: HashMap<&'static str, TokenType>,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn scan(input: &'a String) -> Vec<Token> {
        let mut scanner = Scanner::new(&input);
        let mut tokens = Vec::<Token>::new();

        while let Some(token) = scanner.next_token() {
            tokens.push(token);
        }

        tokens.push(Token::create(TokenType::Eof, scanner.line));

        tokens
    }

    fn new(input: &'a String) -> Self {
        Scanner {
            input: input,
            current_pos: 0,
            last_read_char: None,
            keyword_to_token: Self::create_keyword_map(),
            line: 0,
        }
    }

    // Consumes the next character in the input.
    // Updates current_position, next_position and current_char.
    fn read_char(&mut self) -> Option<char> {
        let next_char = self.peek_char();

        // Update current_pos. Char type has 4 bytes. The utf-8 representation can be 1-4 bytes.
        self.current_pos = self.current_pos + next_char.map_or(0, |c| c.len_utf8());
        self.last_read_char = next_char;

        if next_char == Some('\n') {
            self.line = self.line + 1;
        }
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

    fn create_token(&self, token_type: TokenType) -> Option<Token> {
        Some(Token::create(token_type, self.line))
    }

    fn create_token_with_string(&self, token_type: TokenType, string: String) -> Option<Token> {
        Some(Token::create_with_string(token_type, self.line, string))
    }

    fn create_token_with_number(&self, token_type: TokenType, number: f64) -> Option<Token> {
        Some(Token::create_with_number(token_type, self.line, number))
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_white_space();

        let token = match self.read_char() {
            // Some tokens correspond to a single character
            Some('(') => self.create_token(TokenType::LeftParen),
            Some(')') => self.create_token(TokenType::RightParen),
            Some('{') => self.create_token(TokenType::LeftBrace),
            Some('}') => self.create_token(TokenType::RightBrace),
            Some(',') => self.create_token(TokenType::Comma),
            Some('.') => self.create_token(TokenType::Dot),
            Some('-') => self.create_token(TokenType::Minus),
            Some('+') => self.create_token(TokenType::Plus),
            Some(';') => self.create_token(TokenType::Semicolon),
            Some('*') => self.create_token(TokenType::Star),

            // Some characters can correspond to a single token, or be the initial character of a two character sequence
            Some('!') => {
                if self.match_char('=') {
                    self.create_token(TokenType::BangEqual)
                } else {
                    self.create_token(TokenType::Bang)
                }
            }
            Some('=') => {
                if self.match_char('=') {
                    self.create_token(TokenType::EqualEqual)
                } else {
                    self.create_token(TokenType::Equal)
                }
            }
            Some('>') => {
                if self.match_char('=') {
                    self.create_token(TokenType::GreaterEqual)
                } else {
                    self.create_token(TokenType::Greater)
                }
            }
            Some('<') => {
                if self.match_char('=') {
                    self.create_token(TokenType::LessEqual)
                } else {
                    self.create_token(TokenType::Less)
                }
            }

            Some('"') => self.read_string_literal(),

            // Single line comments are proceeded by double slash
            Some('/') => {
                if self.match_char('/') {
                    self.skip_rest_of_line();
                    None
                } else {
                    self.create_token(TokenType::Slash)
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

        // Convert chars to an f64
        let number: String = number.into_iter().collect();
        let number = number.parse::<f64>();

        if let Ok(number) = number {
            return self.create_token_with_number(TokenType::Number, number);
        }

        //TODO report error
        None
    }

    // Assumes the opening quote has already been consumed.
    fn read_string_literal(&mut self) -> Option<Token> {
        let mut string: Vec<char> = Vec::new();

        while let Some(character) = self.read_char() {
            if character == '"' {
                // this is the closing quote
                let string: String = string.into_iter().collect();
                return self.create_token_with_string(TokenType::String, string);
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
        if let Some(token_type) = self.keyword_to_token.get(string.as_str()) {
            return self.create_token_with_string(token_type.clone(), string);
        }

        self.create_token_with_string(TokenType::Identifier, string)
    }

    // Produces a map that associated string litrals representing keywords to the associated Token.
    fn create_keyword_map() -> HashMap<&'static str, TokenType> {
        let mut map = HashMap::new();

        map.insert("and", TokenType::And);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("fun", TokenType::Fun);
        map.insert("for", TokenType::For);
        map.insert("if", TokenType::If);
        map.insert("nil", TokenType::Nil);
        map.insert("or", TokenType::Or);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);

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
