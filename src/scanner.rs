use crate::token::{Token, TokenKind};

/// Scanner iterates over source code and produces lexical tokens.
pub struct Scanner<'src> {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub source: &'src [u8],
}

impl<'src> Scanner<'src> {
    /// new creates a new scanner.
    pub fn new(source: &'src str) -> Self {
        Self {
            start: 0,
            current: 0,
            line: 1,
            source: source.as_bytes(),
        }
    }

    /// scan_token scans the next token from the input source.
    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenKind::EOF);
        }

        let byte = self.advance_byte();

        if is_alpha(byte) {
            return self.identifier_token();
        }

        if is_digit(byte) {
            return self.number_token();
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
                if self.match_byte(b'=') {
                    self.make_token(TokenKind::BangEqual)
                } else {
                    self.make_token(TokenKind::Bang)
                }
            }
            b'=' => {
                if self.match_byte(b'=') {
                    self.make_token(TokenKind::EqualEqual)
                } else {
                    self.make_token(TokenKind::Equal)
                }
            }
            b'<' => {
                if self.match_byte(b'=') {
                    self.make_token(TokenKind::LessEqual)
                } else {
                    self.make_token(TokenKind::Less)
                }
            }
            b'>' => {
                if self.match_byte(b'=') {
                    self.make_token(TokenKind::GreaterEqual)
                } else {
                    self.make_token(TokenKind::Greater)
                }
            }
            b'"' => self.string_token(),
            _ => self.error_token("Unexpected character."),
        }
    }

    /// is_at_end checks to see if the current scanner position
    /// has reached the end of the source code.
    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
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
    fn advance_byte(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }

    /// advance increases the current position of the scanner by
    /// one regardless of the location in the source.
    #[inline(always)]
    fn advance(&mut self) {
        self.current += 1;
    }

    /// match_byte checks the given byte and advances the current
    /// if they match and returns true, otherwise returns false.
    fn match_byte(&mut self, byte: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != byte {
            return false;
        }

        self.current += 1;
        true
    }

    /// peek returns the byte at the current position.
    fn peek(&self) -> u8 {
        self.source.get(self.current).copied().unwrap_or(0)
    }

    /// peek_next returns the byte at the next position.
    fn peek_next(&self) -> u8 {
        self.source.get(self.current + 1).copied().unwrap_or(0)
    }

    /// skip_whitespace advances the scanner until no current
    /// whitespace characters are found.
    fn skip_whitespace(&mut self) {
        if self.is_at_end() {
            return;
        }

        loop {
            match self.peek() {
                b' ' | b'\r' | b'\t' => {
                    self.advance();
                }
                b'\n' => {
                    self.line += 1;
                    self.advance();
                }
                b'/' if self.peek_next() == b'/' => {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    /// string_token advances until the whole quoted string
    /// is found or the scanner reaches the end of the source input.
    fn string_token(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token("Unterminated string.");
        }

        self.advance(); // closing quote
        self.make_token(TokenKind::String)
    }

    /// number_token advances the scanner to create a numeric
    /// token handling both integer and floating point types.
    fn number_token(&mut self) -> Token {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && is_digit(self.peek_next()) {
            self.advance(); // consume '.'

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        self.make_token(TokenKind::Number)
    }

    /// identifier_tooken advances the scanner while the
    /// input looks like an identifier. This function handles
    /// both keywords and user defined identifiers.
    fn identifier_token(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        let ident_type = self.identifier_type();
        self.make_token(ident_type)
    }

    /// identifier_type uses a trie to determine if an identifier
    /// is a user generated one or a keyword in the language.
    fn identifier_type(&self) -> TokenKind {
        match &self.source[self.start..self.current] {
            b"and" => TokenKind::And,
            b"class" => TokenKind::Class,
            b"else" => TokenKind::Else,
            b"false" => TokenKind::False,
            b"for" => TokenKind::For,
            b"fun" => TokenKind::Fun,
            b"if" => TokenKind::If,
            b"nil" => TokenKind::Nil,
            b"or" => TokenKind::Or,
            b"print" => TokenKind::Print,
            b"return" => TokenKind::Return,
            b"super" => TokenKind::Super,
            b"this" => TokenKind::This,
            b"true" => TokenKind::True,
            b"var" => TokenKind::Var,
            b"while" => TokenKind::While,
            _ => TokenKind::Identifier,
        }
    }
}

/// is_digit checks to see if the provided byte
/// is an ascii digit.
fn is_digit(byte: u8) -> bool {
    byte.is_ascii_digit()
}

/// is_alpha checks to see if the provided byte
/// is an ascii alphabetic character or an underscore.
fn is_alpha(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'_')
}
