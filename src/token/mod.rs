use ::path::Pathbuf;
use once_cell::sync::Lazy;
use std::collections::Hashmap;
use std::fmt;

pub enum TokenType {
    // Special Characters
    Illegal,
    Eof,
    Newline,
    Indent,
    Dedent,

    // Identifiers and Literals
    Identifier,
    Integer,
    Float,
    String,
    Docstring,

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Mod,
    Exponent,
    PlusAssign,
    MinusAssign,
    AsteriskAssign,
    SlashAssign,
    PlusPlusIncrement,
    MinusMinusDecrement,
    Equality,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Shebang,
    Ampersand,
    Hash,
    At,
    // Delimiters
    Comma,
    Colon,
    Pipe,
    Dot,
    LeftShift,
    RightShift,
    Xor,
    Tilde,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Underscore,
    DoubleUnderscore,

    // Keywords
    Init,
    SelfKeyword,
    Spell,
    Grimoire,
    True,
    False,
    If,
    Else,
    Otherwise,
    For,
    In,
    While,
    Stop,
    Skip,
    Ignore,
    Return,
    Import,
    Match,
    Case,
    Attempt,
    Resolve,
    Ensnare,
    Raise,
    As,
    Maybe,
    Arcane,
    ArcaneSpell,
    Super,
    Fstring,
    Check,
    NoneKeyword,

    // Operators
    And,
    Or,
    Not,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub file_name: Pathbuf,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        literal: String,
        file_name: PathBuf,
        line: usize,
        column: usize,
    ) -> Self {
        Token {
            token_type,
            literal,
            file_name,
            line,
            column,
        }
    }

    pub fn simple(token_type: TokenType, ch: char) -> Self {
        Token {
            token_type,
            literal: ch.to_string(),
            file_name: PathBuf::new(),
            line: 0,
            column: 0,
        }
    }
}
