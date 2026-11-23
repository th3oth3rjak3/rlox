#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
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
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
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

    // Special
    Error(&'static str),
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub start: usize,
    pub length: usize,
}

impl Token {
    /// new creates a new token.
    pub fn new(kind: TokenKind, start: usize, length: usize, line: usize) -> Self {
        Self {
            kind,
            start,
            length,
            line,
        }
    }

    pub fn lexeme<'a>(&self, source: &'a str) -> &'a str {
        if self.start >= source.len() {
            return "";
        }

        &source[self.start..self.start + self.length]
    }
}