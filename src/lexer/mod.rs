use std::collections::HashMap;
use std::path::PathBuf;
use token::Token;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    file: PathBuf,
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
            file,
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
        let Some(c) = self.advance() else {
            return;
        };
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '@' => self.add_token(TokenType::At),
            '%' => self.add_token(TokenType::Mod),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    } else if self.match_char('*') {
                        self.block_comment();
                    } else if self.match_char('=') {
                        self.add_token(TokenType::SlashAssign);
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
            }
            '!' => {
                let t = if self.match_char('=') {
                    TokenType::NotEqual
                } else {
                    TokenType::Not
                };
                self.add_token(t);
            }
            '*' => {
                let t = if self.match_char('=') {
                    TokenType::AsteriskAssign
                } else if self.match_char('*'){
                    TokenType::Exponent
                }
                else {
                    TokenType::Asterisk
                };
                self.add_token(t);
            }
            '>' => {
                let t = if self.match_char('=') {
                    TokenType::GreaterThanEqual
                } else if self.match_char('>') {
                    TokenType::RightShift
                } else {
                    TokenType::GreaterThan
                };
                self.add_token(t);
            }
            '<' => {
                let t = if self.match_char('=') {
                    TokenType::LessThanEqual
                } else if self.match_char('<') {
                    TokenType::LeftShift
                } else {
                    TokenType::LessThan
                };
                self.add_token(t);
            }
            '=' => {
                let t = if self.match_char('=') {
                    TokenType::Equality
                } else {
                    TokenType::Assign
                };
                self.add_token(t)
            }
            '+' => {
                let t = if self.match_char('+') {
                    TokenType::PlusPlusIncrement
                } else if self.match_char('=') {
                    TokenType::PlusAssign
                } else {
                    TokenType::Plus
                };
                self.add_token(t);
            }

            '-' => {
                let t = if self.match_char('-') {
                    TokenType::MinusMinusDecrement
                } else if self.match_char('=') {
                    TokenType::MinusAssign
                } else {
                    TokenType::Minus
                };
                self.add_token(t);
            }
            ' '| '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }


            _ => self.add_token(TokenType::Illegal),
        }
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let ch = self.source[self.current..].chars().next()?;
        let char_len = ch.len_utf8();
        self.current += char_len;
        Some(ch)
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        self.source[self.current..].chars().next()
    }
    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        let mut chars = self.source[self.current..].chars();
        chars.next();
        chars.next()
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn add_token(&mut self, ttype: TokenType, literal: Option<LiteralValue>) {
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
