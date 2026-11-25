use crate::{
    scanner::Scanner,
    token::{Token, TokenKind},
};

/// TokenCursor owns scanner state and maintains the current and previous
/// tokens needed by the compiler's recursive-descent parsing operations.
pub struct TokenCursor<'src> {
    scanner: Scanner<'src>,
    current: Token,
    previous: Token,
}

impl<'src> TokenCursor<'src> {
    /// Creates a token cursor positioned before the first source token.
    ///
    /// The compiler must call `advance()` before reading `current` or
    /// `previous` as meaningful tokens.
    pub fn new(source: &'src str) -> Self {
        let empty = Token::new(
            TokenKind::Error("Compiler did not advance before reading a token."),
            0,
            0,
            0,
        );

        Self {
            scanner: Scanner::new(source),
            current: empty,
            previous: empty,
        }
    }

    /// Moves the current token into the previous-token slot.
    pub fn begin_advance(&mut self) {
        self.previous = self.current;
    }

    /// Scans and stores the next token as the current token.
    pub fn scan_current(&mut self) {
        self.current = self.scanner.scan_token();
    }

    /// Returns the token currently being examined by the compiler.
    pub fn current(&self) -> Token {
        self.current
    }

    /// Returns the most recently consumed token.
    pub fn previous(&self) -> Token {
        self.previous
    }
}
