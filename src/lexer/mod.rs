use std::collections::HashMap;
use std::path::PathBuf;
use token::{LiteralValue, Token, TokenType};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    file: PathBuf,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String, file: PathBuf, keywords: HashMap<String, TokenType>) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            file,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        // Push EOF at end
        let eof_text = "".to_string();
        self.tokens.push(Token::new(
            TokenType::Eof,
            eof_text,
            self.file.clone(),
            self.line,
            self.current,
        ));
        std::mem::take(&mut self.tokens)
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.add_simple(TokenType::LeftParen),
                ')' => self.add_simple(TokenType::RightParen),
                '[' => self.add_simple(TokenType::LeftBracket),
                ']' => self.add_simple(TokenType::RightBracket),
                '{' => self.add_simple(TokenType::LeftBrace),
                '}' => self.add_simple(TokenType::RightBrace),
                '@' => self.add_simple(TokenType::At),
                '%' => self.add_simple(TokenType::Mod),
                ',' => self.add_simple(TokenType::Comma),
                ':' => self.add_simple(TokenType::Colon),
                '.' => self.add_simple(TokenType::Dot),
                '|' => self.add_simple(TokenType::Pipe),
                '~' => self.add_simple(TokenType::Tilde),
                '^' => self.add_simple(TokenType::Xor),
                '/' => {
                    if self.match_char('/') {
                        // line comment
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else if self.match_char('*') {
                        self.block_comment();
                    } else if self.match_char('=') {
                        self.add_simple(TokenType::SlashAssign);
                    } else {
                        self.add_simple(TokenType::Slash);
                    }
                }
                '_' => {
                    let t = if self.match_char('_') {
                        TokenType::DoubleUnderscore
                    } else {
                        TokenType::Underscore
                    };
                    self.add_simple(t);
                }
                '\'' | '"' => self.string(),
                '#' => self.add_simple(TokenType::Hash),
                '!' => {
                    let t = if self.match_char('=') {
                        TokenType::NotEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_simple(t);
                }
                '*' => {
                    let t = if self.match_char('=') {
                        TokenType::AsteriskAssign
                    } else if self.match_char('*') {
                        TokenType::Exponent
                    } else {
                        TokenType::Asterisk
                    };
                    self.add_simple(t);
                }
                '>' => {
                    let t = if self.match_char('=') {
                        TokenType::GreaterThanEqual
                    } else if self.match_char('>') {
                        TokenType::RightShift
                    } else {
                        TokenType::GreaterThan
                    };
                    self.add_simple(t);
                }
                '<' => {
                    let t = if self.match_char('=') {
                        TokenType::LessThanEqual
                    } else if self.match_char('<') {
                        TokenType::LeftShift
                    } else {
                        TokenType::LessThan
                    };
                    self.add_simple(t);
                }
                '=' => {
                    let t = if self.match_char('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_simple(t);
                }
                '+' => {
                    let t = if self.match_char('+') {
                        TokenType::PlusPlus
                    } else if self.match_char('=') {
                        TokenType::PlusAssign
                    } else {
                        TokenType::Plus
                    };
                    self.add_simple(t);
                }
                '-' => {
                    let t = if self.match_char('-') {
                        TokenType::MinusMinus
                    } else if self.match_char('=') {
                        TokenType::MinusAssign
                    } else {
                        TokenType::Minus
                    };
                    self.add_simple(t);
                }
                ' ' | '\r' | '\t' => { /* ignore whitespace */ }
                '\n' => {
                    self.line += 1;
                }
                c if c.is_ascii_digit() => self.number(),
                c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),
                _ => {
                    eprintln!(
                        "[Line {}, Col {}] Unexpected '{}', skipping.",
                        self.line, self.start, c
                    );
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        let mut iter = self.source[self.current..].char_indices();
        if let Some((_, ch)) = iter.next() {
            let next_pos = iter.next().map(|(i, _)| i).unwrap_or(ch.len_utf8());
            self.current += next_pos;
            Some(ch)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
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

    fn add_simple(&mut self, ttype: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            ttype,
            text.to_owned(),
            self.file.clone(),
            self.line,
            self.start,
        ));
    }

    fn add_literal(&mut self, ttype: TokenType, value: LiteralValue) {
        self.tokens.push(
            Token::new(
                ttype,
                value.to_string(),
                self.file.clone(),
                self.line,
                self.start,
            )
            .with_literal(value),
        );
    }

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }
        // Unterminated?
        if self.is_at_end() {
            eprintln!("Unterminated string at line {}", self.line);
            return;
        }
        // consume closing '"'
        self.advance();
        let raw = &self.source[self.start + 1..self.current - 1];
        let lit = LiteralValue::String(raw.to_owned());
        self.add_literal(TokenType::String, lit);
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
            self.advance(); // consume '.'
            while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                self.advance();
            }
            let text = &self.source[self.start..self.current];
            if let Ok(val) = text.parse::<f64>() {
                self.add_literal(TokenType::Float, LiteralValue::Float(val));
            }
        } else {
            let text = &self.source[self.start..self.current];
            if let Ok(val) = text.parse::<i64>() {
                self.add_literal(TokenType::Integer, LiteralValue::Integer(val));
            }
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
        let ttype = self
            .keywords
            .get(text)
            .cloned()
            .unwrap_or(TokenType::Identifier);
        self.add_simple(ttype);
    }

    fn block_comment(&mut self) {
        while !(self.peek() == Some('*') && self.peek_next() == Some('/')) && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }
        // consume '*/'
        if !self.is_at_end() {
            self.advance();
            self.advance();
        }
    }
}
