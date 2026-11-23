use crate::{
    chunk::Chunk,
    common::Value,
    debug::disassemble_chunk,
    opcode::*,
    precedence::Precedence,
    scanner::Scanner,
    token::{Token, TokenKind},
};

/// ParseFn is any function that can parse source.
type ParseFn<'src> = fn(&mut Compiler<'src>);

/// ParseRule is a rule that describes how to parse
/// precedences.
struct ParseRule<'src> {
    prefix: Option<ParseFn<'src>>,
    infix: Option<ParseFn<'src>>,
    precedence: Precedence,
}

impl<'src> ParseRule<'src> {
    /// Create a new parse rule.
    const fn new(
        prefix: Option<ParseFn<'src>>,
        infix: Option<ParseFn<'src>>,
        precedence: Precedence,
    ) -> Self {
        Self {
            prefix,
            infix,
            precedence,
        }
    }
}

/// Compiler is a source code compiler that iterates over the source and
/// produces executable bytecode chunks.
pub struct Compiler<'src> {
    pub source: &'src str,
    pub scanner: Scanner<'src>,
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
    pub chunk: Chunk,
}

impl<'src> Compiler<'src> {
    /// new creates a new compiler. The compiler must be primed with a
    /// call to advance in order to scan the first token.
    pub fn new(source: &'src str) -> Self {
        // error_token is expected to fill current and previous until the first real
        // "advance" is called on the parser to populate the current token.
        let error_token = Token::new(
            TokenKind::Error("Parser did not advance before reading token."),
            0,
            0,
            0,
        );

        Self {
            scanner: Scanner::new(source),
            current: error_token,
            previous: error_token,
            had_error: false,
            panic_mode: false,
            source,
            chunk: Chunk::new(),
        }
    }

    /// advance scasns the next token, bypassing an error
    /// tokens.
    pub fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();
            if let TokenKind::Error(err) = self.current.kind {
                self.error_at_current(err);
                continue;
            }

            break;
        }
    }

    /// consume advances when the current token contains the expected
    /// kind. Otherwise it prints an error message.
    pub fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.current.kind == kind {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    /// expression is the entry point for the expression parser.
    pub fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn number(&mut self) {
        let lexeme = self.previous.lexeme(self.source);
        let value: f64 = lexeme
            .parse()
            .expect("Scanner produced invalid number literal.");
        self.emit_constant(Value::Number(value));
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator = self.previous.kind;

        self.parse_precedence(Precedence::Unary);

        match operator {
            TokenKind::Minus => self.emit_byte(OP_NEGATE),
            TokenKind::Bang => self.emit_byte(OP_NOT),
            _ => (),
        }
    }

    fn binary(&mut self) {
        let operator = self.previous.kind;
        let rule = self.get_rule(operator);
        self.parse_precedence(rule.precedence.next());

        match operator {
            TokenKind::BangEqual => self.emit_byte(OP_NOT_EQUAL),
            TokenKind::EqualEqual => self.emit_byte(OP_EQUAL),
            TokenKind::Greater => self.emit_byte(OP_GREATER),
            TokenKind::GreaterEqual => self.emit_byte(OP_GREATER_EQUAL),
            TokenKind::Less => self.emit_byte(OP_LESS),
            TokenKind::LessEqual => self.emit_byte(OP_LESS_EQUAL),
            TokenKind::Plus => self.emit_byte(OP_ADD),
            TokenKind::Minus => self.emit_byte(OP_SUBTRACT),
            TokenKind::Star => self.emit_byte(OP_MULTIPLY),
            TokenKind::Slash => self.emit_byte(OP_DIVIDE),
            _ => (),
        }
    }

    fn literal(&mut self) {
        match self.previous.kind {
            TokenKind::False => self.emit_byte(OP_FALSE),
            TokenKind::True => self.emit_byte(OP_TRUE),
            TokenKind::Nil => self.emit_byte(OP_NIL),
            _ => (),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(self.previous.kind).prefix;
        if prefix_rule.is_none() {
            self.error("Expect expression.");
            return;
        }

        prefix_rule.unwrap()(self);

        while precedence <= self.get_rule(self.current.kind).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.previous.kind).infix;
            infix_rule.expect("infix rule should not have been None")(self);
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let line = self.previous.line;
        let chunk = self.current_chunk_mut();
        chunk.write_constant(value, line);
    }

    /// emit_byte writes a byte into the current chunk.
    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.line;
        let chunk = self.current_chunk_mut();
        chunk.write(byte, line);
    }

    /// end_compiler finishes compilation.
    pub fn end_compiler(&mut self) {
        self.emit_return();

        cfg_select! {
            debug_assertions => {
                if !self.had_error {
                    disassemble_chunk(self.current_chunk(), "code");
                }
            }
            _ => { /* NO-OP */}
        }
    }

    /// emit_return emits a return instruction to the current chunk.
    fn emit_return(&mut self) {
        self.emit_byte(OP_RETURN);
    }

    /// emit_bytes emits two bytes together to the current chunk.
    #[allow(unused)]
    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    /// current_chunk takes a borrow to the current
    /// chunk.
    fn current_chunk(&self) -> &Chunk {
        &self.chunk
    }

    /// current_chunk_mut takes a mutable borrow to the
    /// current chunk.
    fn current_chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    /// error produces an error at the previous token.
    fn error(&mut self, message: &str) {
        self.error_at(self.previous, message);
    }

    /// error_at_current produces an error at the current token.
    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message);
    }

    /// error_at is used to produce errors for different tokens.
    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);
        if token.kind == TokenKind::EOF {
            eprint!(" at end");
        } else if matches!(token.kind, TokenKind::Error(_)) {
            // Do nothing because there was already an error.
        } else {
            eprint!(" at {}", token.lexeme(self.source))
        }

        eprintln!(": {message}");
        self.had_error = true;
    }

    fn get_rule(&self, kind: TokenKind) -> ParseRule<'src> {
        match kind {
            TokenKind::LeftParen => ParseRule::new(Some(Self::grouping), None, Precedence::None),
            TokenKind::Minus => {
                ParseRule::new(Some(Self::unary), Some(Self::binary), Precedence::Term)
            }
            TokenKind::Plus => ParseRule::new(None, Some(Self::binary), Precedence::Term),
            TokenKind::Slash => ParseRule::new(None, Some(Self::binary), Precedence::Factor),
            TokenKind::Star => ParseRule::new(None, Some(Self::binary), Precedence::Factor),
            TokenKind::Number => ParseRule::new(Some(Self::number), None, Precedence::None),
            TokenKind::True | TokenKind::False | TokenKind::Nil => {
                ParseRule::new(Some(Self::literal), None, Precedence::None)
            }
            TokenKind::Bang => ParseRule::new(Some(Self::unary), None, Precedence::None),
            TokenKind::BangEqual | TokenKind::EqualEqual => {
                ParseRule::new(None, Some(Self::binary), Precedence::Equality)
            }
            TokenKind::Greater
            | TokenKind::GreaterEqual
            | TokenKind::Less
            | TokenKind::LessEqual => {
                ParseRule::new(None, Some(Self::binary), Precedence::Comparison)
            }
            _ => ParseRule::new(None, None, Precedence::None),
        }
    }
}
