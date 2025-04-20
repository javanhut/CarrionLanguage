use std::collections::HashMap;
use token::Token;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(
        source: String,
        tokens: Vec<Token>,
        start: usize,
        current: usize,
        line: usize,
    ) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "".into(), ..));
        std::mem::take(&mut self.tokens)
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),

            _ => self.add_token(TokenType::Illegal),
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn advance(&mut self) -> char {
        let ch = self.source.as_byte()[self.current] as char;
        self.current += 1;
        ch
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }
    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.current += 1;
            true
        } else {
            false
        }
    }
    fn add_token(&mut self, ttype: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            ttype,
            text.to_string(),
            file.clone(),
            self.line,
            self.start,
        ));
    }
    fn add_token_literal(&mut self, ttype: TokenType, literal: &str) {
        let lexeme = literal.to_string();
        self.tokens.push(Token::new(
            token_type,
            lexeme,
            PathBuf::new(),
            self.line,
            self.start,
        ));
    }
    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return;
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::String, value);
    }
    fn number(&mut self) {
        while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            self.advance();
        }
        if self.peek() == Some('.')
            && self
                .peek_next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false)
        {
            self.advance();
            while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                self.advance();
            }
            let text = &self.source[self.start..self.current];
            self.add_token_literal(TokenType::Float, text);
        } else {
            let text = &self.source[self.start..self.current];
            self.add_token_literal(TokenType::Integer, text);
        }
    }
    fn identifier(&mut self) {
        while self
            .peek()
            .map(|c| c.is_alphanumeric() || c == '_')
            .unwrap_or(false)
        {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = lookup_identifer(text);
        self.add_token_literal(token_type, text);
    }
}
