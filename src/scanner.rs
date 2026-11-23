use crate::token::{Token, TokenKind};

/// Scanner iterates over source code and produces lexical tokens.
pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Scanner {
    /// new creates a new scanner.
    pub fn new() -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// scan_token scans the next token from the input source.
    pub fn scan_token(&mut self, source: &str) -> Token {
        self.skip_whitespace(source);
        self.start = self.current;

        if self.is_at_end(source) {
            return self.make_token(TokenKind::EOF);
        }

        let byte = self.advance_byte(source);

        if is_alpha(byte) {
            return self.identifier_token(source);
        }

        if is_digit(byte) {
            return self.number_token(source);
        }

        match byte {
            b'(' => self.make_token(TokenKind::LeftParen),
            b')' => self.make_token(TokenKind::RightParen),
            b'{' => self.make_token(TokenKind::LeftBrace),
            b'}' => self.make_token(TokenKind::RightBrace),
            b';' => self.make_token(TokenKind::Semicolon),
            b',' => self.make_token(TokenKind::Comma),
            b'.' => self.make_token(TokenKind::Dot),
            b'-' => self.make_token(TokenKind::Minus),
            b'+' => self.make_token(TokenKind::Plus),
            b'/' => self.make_token(TokenKind::Slash),
            b'*' => self.make_token(TokenKind::Star),
            b'!' => {
                if self.match_byte(b'=', source) {
                    return self.make_token(TokenKind::BangEqual);
                } else {
                    return self.make_token(TokenKind::Bang);
                }
            }
            b'=' => {
                if self.match_byte(b'=', source) {
                    return self.make_token(TokenKind::EqualEqual);
                } else {
                    return self.make_token(TokenKind::Equal);
                }
            }
            b'<' => {
                if self.match_byte(b'=', source) {
                    return self.make_token(TokenKind::LessEqual);
                } else {
                    return self.make_token(TokenKind::Less);
                }
            }
            b'>' => {
                if self.match_byte(b'=', source) {
                    return self.make_token(TokenKind::GreaterEqual);
                } else {
                    return self.make_token(TokenKind::Greater);
                }
            }
            b'"' => self.string_token(source),
            _ => self.error_token("Unexpected character."),
        }
    }

    /// is_at_end checks to see if the current scanner position
    /// has reached the end of the source code.
    fn is_at_end(&self, source: &str) -> bool {
        self.current == source.len()
    }

    /// make_token produces a token based on the current location
    /// of the scanner and using the provided kind.
    fn make_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    /// error_token produces an error token with a static message.
    fn error_token(&self, message: &'static str) -> Token {
        Token {
            kind: TokenKind::Error(message),
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    /// advance increases the current position of the scanner
    /// and returns the current byte in the source string.
    fn advance_byte(&mut self, source: &str) -> u8 {
        self.current += 1;
        source.as_bytes()[self.current - 1]
    }

    /// advance increases the current position of the scanner by
    /// one regardless of the location in the source.
    #[inline(always)]
    fn advance(&mut self) {
        self.current += 1;
    }

    /// match_byte checks the given byte and advances the current
    /// if they match and returns true, otherwise returns false.
    fn match_byte(&mut self, byte: u8, source: &str) -> bool {
        if self.is_at_end(source) {
            return false;
        }

        if source.as_bytes()[self.current] != byte {
            return false;
        }

        self.current += 1;
        return true;
    }

    /// peek returns the byte at the current position.
    fn peek(&self, source: &str) -> u8 {
        source.as_bytes()[self.current]
    }

    fn peek_next(&self, source: &str) -> u8 {
        if self.is_at_end(source) {
            return 0;
        }

        return source.as_bytes()[self.current + 1];
    }

    /// skip_whitespace advances the scanner until no current
    /// whitespace characters are found.
    fn skip_whitespace(&mut self, source: &str) {
        if self.is_at_end(source) {
            return;
        }

        loop {
            match self.peek(source) {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                b'/' => {
                    if self.peek_next(source) == b'/' {
                        while self.peek(source) != b'\n' && !self.is_at_end(source) {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    /// string_token advances until the whole quoted string
    /// is found or the scanner reaches the end of the source input.
    fn string_token(&mut self, source: &str) -> Token {
        while self.peek(source) != b'"' && !self.is_at_end(source) {
            if self.peek(source) == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end(source) {
            return self.error_token("Unterminated string.");
        }

        self.advance(); // closing quote
        return self.make_token(TokenKind::String);
    }

    /// number_token advances the scanner to create a numeric
    /// token handling both integer and floating point types.
    fn number_token(&mut self, source: &str) -> Token {
        while is_digit(self.peek(source)) {
            self.advance();
        }

        if self.peek(source) == b'.' && is_digit(self.peek_next(source)) {
            self.advance(); // consume '.'

            while is_digit(self.peek(source)) {
                self.advance();
            }
        }

        return self.make_token(TokenKind::Number);
    }

    /// identifier_tooken advances the scanner while the
    /// input looks like an identifier. This function handles
    /// both keywords and user defined identifiers.
    fn identifier_token(&mut self, source: &str) -> Token {
        while is_alpha(self.peek(source)) || is_digit(self.peek(source)) {
            self.advance();
        }

        let ident_type = self.identifier_type(source);
        return self.make_token(ident_type);
    }

    /// identifier_type uses a trie to determine if an identifier
    /// is a user generated one or a keyword in the language.
    fn identifier_type(&self, source: &str) -> TokenKind {
        match &source[self.start..self.current] {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        }
    }
}

/// is_digit checks to see if the provided byte
/// is an ascii digit.
fn is_digit(byte: u8) -> bool {
    matches!(byte, b'0'..=b'9')
}

/// is_alpha checks to see if the provided byte
/// is an ascii alphabetic character or an underscore.
fn is_alpha(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'_')
}
