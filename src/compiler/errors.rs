use super::Compiler;
use crate::token::{Token, TokenKind};

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// Synchronizes token parsing after a compilation error.
    pub fn synchronize(&mut self) {
        self.panic_mode = false;

        while self.tokens.current().kind != TokenKind::EOF {
            if self.tokens.previous().kind == TokenKind::Semicolon {
                return;
            }

            match self.tokens.current().kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => self.advance_token(),
            }
        }
    }

    /// Reports a compilation error at the previously consumed token.
    pub fn error(&mut self, message: &str) {
        self.error_at(self.tokens.previous(), message);
    }

    /// Reports a compilation error at the current token.
    pub fn error_at_current(&mut self, message: &str) {
        self.error_at(self.tokens.current(), message);
    }

    /// Reports a compilation error at a specific token.
    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        if token.kind == TokenKind::EOF {
            eprint!(" at end");
        } else if !matches!(token.kind, TokenKind::Error(_)) {
            eprint!(" at {}", token.lexeme(self.source));
        }

        eprintln!(": {message}");
        self.had_error = true;
    }
}
