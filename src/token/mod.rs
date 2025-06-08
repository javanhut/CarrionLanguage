use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

// ─── Token kinds ──────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    StringLit,
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

    // Logical operators
    And,
    Or,
    Not,
}

// ─── Token struct ─────────────────────────────────────────────────────────────
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub file_name: PathBuf,
    pub line: usize,
    pub column: usize,
}

// Display implementations give you `to_string()` for free
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}  {}({})",
            self.file_name.display(),
            self.line,
            self.column,
            self.token_type,
            self.literal
        )
    }
}

// ─── Convenience constructors ────────────────────────────────────────────────
impl Token {
    pub fn new(
        token_type: TokenType,
        literal: impl Into<String>,
        file_name: impl Into<PathBuf>,
        line: usize,
        column: usize,
    ) -> Self {
        Token {
            token_type,
            literal: literal.into(),
            file_name: file_name.into(),
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

// ─── Keyword lookup table ─────────────────────────────────────────────────────
pub static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    use TokenType::*;

    let mut map = HashMap::with_capacity(40);
    map.insert("import", Import);
    map.insert("match", Match);
    map.insert("case", Case);
    map.insert("spell", Spell);
    map.insert("self", SelfKeyword);
    map.insert("init", Init);
    map.insert("grim", Grimoire);
    map.insert("true", True);
    map.insert("false", False);
    map.insert("if", If);
    map.insert("else", Else);
    map.insert("otherwise", Otherwise);
    map.insert("for", For);
    map.insert("loop", Loop);
    map.insert("in", In);
    map.insert("stop", Stop);
    map.insert("skip", Skip);
    map.insert("ignore", Ignore);
    map.insert("and", And);
    map.insert("or", Or);
    map.insert("not", Not);
    map.insert("return", Return);
    map.insert("attempt", Attempt);
    map.insert("resolve", Resolve);
    map.insert("ensnare", Ensnare);
    map.insert("raise", Raise);
    map.insert("as", As);
    map.insert("arcane", Arcane);
    map.insert("arcanespell", ArcaneSpell);
    map.insert("super", Super);
    map.insert("check", Check);
    map.insert("maybe", Maybe);
    map.insert("none", NoneKeyword);
    map.insert("while", While);
    map
});

pub fn lookup_identifier(ident: &str) -> TokenType {
    KEYWORDS
        .get(&ident.to_ascii_lowercase()[..])
        .copied()
        .unwrap_or(TokenType::Identifier)
}
