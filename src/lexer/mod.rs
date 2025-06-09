//! lexer.rs  ― Carrion language

use crate::token::{KEYWORDS, Token, TokenType};
use std::path::PathBuf;

/// Scans a UTF-8 source file into a stream of `Token`s.
///
/// Call `scan_tokens()` once; it returns the finished vector.
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    file: PathBuf,
    indent_stack: Vec<usize>,
    at_line_start: bool,
    pending_dedents: usize,
    max_nesting_depth: usize,
}

impl Lexer {
    /// Create a new lexer for the given source string and filename.
    pub fn new(source: String, file: PathBuf) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            file,
            indent_stack: vec![0], // Start with base indentation of 0
            at_line_start: true,
            pending_dedents: 0,
            max_nesting_depth: 50, // Production limit
        }
    }

    /// Scan the entire file and hand back the token list (consumes `self.tokens`).
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // Generate final dedents to close all open indentation levels
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.add_simple(TokenType::Dedent);
        }

        // Append the final EOF token
        self.tokens.push(Token::new(
            TokenType::Eof,
            "",
            self.file.clone(),
            self.line,
            self.current,
        ));

        std::mem::take(&mut self.tokens)
    }

    // ─── CHARACTER-LEVEL HELPERS ──────────────────────────────────────────────

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Return the current char and advance `self.current` in bytes.
    fn advance(&mut self) -> Option<char> {
        let slice = &self.source[self.current..];
        let ch = slice.chars().next()?;
        self.current += ch.len_utf8();
        Some(ch)
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut iter = self.source[self.current..].chars();
        iter.next();
        iter.next()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    // ─── TOKEN EMISSION HELPERS ───────────────────────────────────────────────

    fn add_simple(&mut self, kind: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            kind,
            text,
            self.file.clone(),
            self.line,
            self.start,
        ));
    }

    fn add_literal<V: ToString>(&mut self, kind: TokenType, value: V) {
        self.tokens.push(Token::new(
            kind,
            value.to_string(),
            self.file.clone(),
            self.line,
            self.start,
        ));
    }

    // ─── HIGH-LEVEL SCANNING LOGIC ────────────────────────────────────────────

    fn scan_token(&mut self) {
        // Emit any pending dedent tokens
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            self.add_simple(TokenType::Dedent);
            return;
        }

        // Handle indentation at the start of a line
        if self.at_line_start {
            if let Err(e) = self.handle_indentation_safe() {
                eprintln!("Indentation error at line {}: {}", self.line, e);
                return;
            }
            self.at_line_start = false;
            
            // If we generated dedent tokens, return to emit them
            if self.pending_dedents > 0 {
                return;
            }
            
            if self.is_at_end() {
                return;
            }
        }

        let c = match self.advance() {
            Some(ch) => ch,
            None => return,
        };

        match c {
            // single-char tokens ----------------------------------------------
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
            '#' => self.add_simple(TokenType::Hash),
            '&' => self.add_simple(TokenType::Ampersand),
            // operators that need look-ahead -----------------------------------
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
                let kind = if self.match_char('_') {
                    TokenType::DoubleUnderscore
                } else {
                    TokenType::Underscore
                };
                self.add_simple(kind);
            }
            '!' => {
                let kind = if self.match_char('=') {
                    TokenType::NotEqual
                } else {
                    TokenType::Shebang
                };
                self.add_simple(kind);
            }
            '*' => {
                let kind = if self.match_char('=') {
                    TokenType::AsteriskAssign
                } else if self.match_char('*') {
                    TokenType::Exponent
                } else {
                    TokenType::Asterisk
                };
                self.add_simple(kind);
            }
            '>' => {
                let kind = if self.match_char('=') {
                    TokenType::GreaterThanEqual
                } else if self.match_char('>') {
                    TokenType::RightShift
                } else {
                    TokenType::GreaterThan
                };
                self.add_simple(kind);
            }
            '<' => {
                let kind = if self.match_char('=') {
                    TokenType::LessThanEqual
                } else if self.match_char('<') {
                    TokenType::LeftShift
                } else if self.match_char('-') {
                    TokenType::LeftArrow
                } else {
                    TokenType::LessThan
                };
                self.add_simple(kind);
            }
            '=' => {
                let kind = if self.match_char('=') {
                    TokenType::Equality
                } else {
                    TokenType::Assign
                };
                self.add_simple(kind);
            }
            '+' => {
                let kind = if self.match_char('+') {
                    TokenType::Increment
                } else if self.match_char('=') {
                    TokenType::PlusAssign
                } else {
                    TokenType::Plus
                };
                self.add_simple(kind);
            }
            '-' => {
                let kind = if self.match_char('-') {
                    TokenType::Decrement
                } else if self.match_char('=') {
                    TokenType::MinusAssign
                } else if self.match_char('-') {
                    TokenType::RightArrow
                } else {
                    TokenType::Minus
                };
                self.add_simple(kind);
            }

            // whitespace / newlines -------------------------------------------
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.at_line_start = true;
                self.add_simple(TokenType::Newline);
            }

            // literals ---------------------------------------------------------
            '\'' | '"' => self.string(c),
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.identifier(),

            // catch-all --------------------------------------------------------
            _ => eprintln!(
                "[Line {}, Col {}] Unexpected '{}', skipping.",
                self.line, self.start, c
            ),
        }
    }

    // ─── LEXEME-LEVEL ROUTINES ───────────────────────────────────────────────

    /// Consume a quoted string. `quote` is the opening char (' or ").
    fn string(&mut self, quote: char) {
        while self.peek() != Some(quote) && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Unterminated string at line {}", self.line);
            return;
        }

        self.advance(); // consume closing quote
        let raw = &self.source[self.start + 1..self.current - 1];
        let lexeme = raw.to_owned();
        self.add_literal(TokenType::StringLit, lexeme);
    }

    fn number(&mut self) {
        while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            self.advance();
        }

        // fractional?
        let is_float = self.peek() == Some('.')
            && self
                .peek_next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false);

        if is_float {
            self.advance(); // consume '.'
            while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                self.advance();
            }
        }

        let text = &self.source[self.start..self.current];
        let lexeme = text.to_owned();
        let kind = if is_float {
            TokenType::Float
        } else {
            TokenType::Integer
        };
        self.add_literal(kind, lexeme);
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
        let key = text.to_ascii_lowercase(); // lexer is case-insensitive
        let kind = KEYWORDS
            .get(key.as_str())
            .copied()
            .unwrap_or(TokenType::Identifier);
        self.add_simple(kind);
    }

    /// Skip a C-style block comment `/* ... */`.
    fn block_comment(&mut self) {
        let start_pos = self.current;
        
        while !(self.peek() == Some('*') && self.peek_next() == Some('/')) && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
            
            // Safety check to prevent infinite loops
            if self.current > start_pos + 10000 {
                eprintln!("Warning: Block comment too long, stopping at line {}", self.line);
                break;
            }
        }

        // Consume the trailing "*/" if present.
        if !self.is_at_end() && self.peek() == Some('*') && self.peek_next() == Some('/') {
            self.advance(); // consume '*'
            self.advance(); // consume '/'
        }
    }

    /// Handle indentation at the beginning of a line with production-ready error handling
    fn handle_indentation_safe(&mut self) -> Result<(), String> {
        let mut indent_level = 0;
        let start_pos = self.current;
        
        // Count spaces and tabs at the beginning of the line
        while let Some(ch) = self.peek() {
            match ch {
                ' ' => {
                    indent_level += 1;
                    self.advance();
                }
                '\t' => {
                    indent_level += 8; // Consistent tab width
                    self.advance();
                }
                '\n' | '\r' => {
                    // Empty line, skip indentation handling
                    return Ok(());
                }
                '#' => {
                    // Line comment, skip entire line
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                    return Ok(());
                }
                '/' if self.peek_next() == Some('/') => {
                    // C++ style comment, skip entire line
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                    return Ok(());
                }
                '/' if self.peek_next() == Some('*') => {
                    // Block comment at start of line
                    self.advance(); // consume '/'
                    self.advance(); // consume '*'
                    self.block_comment();
                    if self.peek() == Some('\n') || self.is_at_end() {
                        return Ok(());
                    }
                    // Continue checking for more whitespace/indentation
                }
                _ => break,
            }
            
            // Safety check: prevent infinite loops
            if self.current > start_pos + 1000 {
                return Err("Excessive whitespace at line start".to_string());
            }
        }

        // Validate indentation level
        if indent_level > 1000 {
            return Err("Indentation level too deep".to_string());
        }

        let current_indent = *self.indent_stack.last().unwrap_or(&0);
        
        if indent_level > current_indent {
            // Check nesting depth limit
            if self.indent_stack.len() >= self.max_nesting_depth {
                return Err(format!("Maximum nesting depth ({}) exceeded", self.max_nesting_depth));
            }
            
            // Increased indentation - emit INDENT
            self.indent_stack.push(indent_level);
            self.add_simple(TokenType::Indent);
        } else if indent_level < current_indent {
            // Decreased indentation - count how many dedents we need
            let mut dedent_count = 0;
            let mut temp_stack = self.indent_stack.clone();
            
            while let Some(&stack_level) = temp_stack.last() {
                if stack_level <= indent_level {
                    break;
                }
                temp_stack.pop();
                dedent_count += 1;
            }
            
            // Check if we have a matching indentation level
            if temp_stack.last() != Some(&indent_level) {
                return Err("Inconsistent indentation level".to_string());
            }
            
            // Validate reasonable dedent count
            if dedent_count > 20 {
                return Err("Too many dedent levels at once".to_string());
            }
            
            // Apply the dedents
            for _ in 0..dedent_count {
                self.indent_stack.pop();
            }
            
            // Queue dedent tokens (emit one this cycle, queue the rest)
            if dedent_count > 0 {
                self.pending_dedents = dedent_count - 1;
                self.add_simple(TokenType::Dedent);
            }
        }
        // If indent_level == current_indent, no change needed
        
        Ok(())
    }

    /// Legacy indentation handler for backward compatibility
    fn handle_indentation(&mut self) {
        if let Err(e) = self.handle_indentation_safe() {
            eprintln!("Indentation error at line {}: {}", self.line, e);
        }
    }
}
