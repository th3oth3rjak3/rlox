use super::Compiler;
use crate::token::TokenKind;

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// Advances to the next non-error token.
    pub(super) fn advance_token(&mut self) {
        self.tokens.begin_advance();

        loop {
            self.tokens.scan_next_token();
            if let TokenKind::Error(err) = self.tokens.current().kind {
                self.error_at_current(err);
                continue;
            }

            break;
        }
    }

    /// Advances when the current token has the provided kind, otherwise reports an error.
    pub(super) fn consume_current_token(&mut self, kind: TokenKind, message: &str) {
        if self.tokens.current().kind == kind {
            self.advance_token();
            return;
        }

        self.error_at_current(message);
    }

    /// Advances when the current token has the provided kind.
    pub(super) fn match_current_token(&mut self, kind: TokenKind) -> bool {
        if !self.current_token_is(kind) {
            return false;
        }

        self.advance_token();
        true
    }

    /// Returns whether the current token has the provided kind.
    pub(super) fn current_token_is(&self, kind: TokenKind) -> bool {
        self.tokens.current().kind == kind
    }
}
