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
    Loop,
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

static KEYWORDS: Lazy<Hashmap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = Hashmap::new();
    map.insert("import", TokenType::Import);
    map.insert("match", TokenType::Match);
    map.insert("case", TokenType::Case);
    map.insert("spell", TokenType::Spell);
    map.insert("self", TokenType::SelfKeyword);
    map.insert("init", TokenType::Init);
    map.insert("grim", TokenType::Grimoire);
    map.insert("True", TokenType::True);
    map.insert("False", TokenType::False);
    map.insert("if", TokenType::If);
    map.insert("else", TokenType::Else);
    map.insert("otherwise", TokenType::Otherwise);
    map.insert("for", TokenType::For);
    map.insert("loop", TokenType::Loop);
    map.insert("in", TokenType::In);
    map.insert("stop", TokenType::Stop);
    map.insert("skip", TokenType::Skip);
    map.insert("ignore", TokenType::Ignore);
    map.insert("and", TokenType::And);
    map.insert("or", TokenType::Or);
    map.insert("not", TokenType::Not);
    map.insert("return", TokenType::Return);
    map.insert("attempt", TokenType::Attempt);
    map.insert("resolve", TokenType::Resolve);
    map.insert("ensnare", TokenType::Ensnare);
    map.insert("raise", TokenType::Raise);
    map.insert("as", TokenType::As);
    map.insert("arcane", TokenType::Arcane);
    map.insert("arcanespell", TokenType::ArcaneSpell);
    map.insert("super", TokenType::Super);
    map.insert("check", TokenType::Check);
    map.insert("maybe", TokenType::Maybe);
    map.insert("None", TokenType::NoneKeyword);
    map.insert("while", TokenType::While);
    map
});

pub fn lookup_identifier(identifier: &str) -> TokenType {
    KEYWORDS
        .get(identifier)
        .copied()
        .unwrap_or(TokenType::Identifier)
}
