use crate::{
    chunk::Chunk,
    common::Value,
    debug::Disassembler,
    heap::Heap,
    opcode::*,
    precedence::Precedence,
    scanner::Scanner,
    token::{Token, TokenKind},
};

/// ParseFn is any function that can parse source.
type ParseFn<'src, 'heap> = fn(&mut Compiler<'src, 'heap>);

/// ParseRule is a rule that describes how to parse
/// precedences.
struct ParseRule<'src, 'heap> {
    prefix: Option<ParseFn<'src, 'heap>>,
    infix: Option<ParseFn<'src, 'heap>>,
    precedence: Precedence,
}

impl<'src, 'heap> ParseRule<'src, 'heap> {
    /// Create a new parse rule.
    const fn new(
        prefix: Option<ParseFn<'src, 'heap>>,
        infix: Option<ParseFn<'src, 'heap>>,
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
pub struct Compiler<'src, 'heap> {
    pub source: &'src str,
    pub scanner: Scanner<'src>,
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
    pub chunk: Chunk,
    pub heap: &'heap mut Heap,
}

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// new creates a new compiler. The compiler must be primed with a
    /// call to advance in order to scan the first token.
    pub fn new(source: &'src str, heap: &'heap mut Heap) -> Self {
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
            heap,
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

    /// match_token conditionally advances the compiler
    /// if the current token matches the expected kind.
    pub fn match_token(&mut self, kind: TokenKind) -> bool {
        if !self.check(kind) {
            return false;
        }

        self.advance();
        true
    }

    /// check determines if the current token kind matches the expected
    /// kind.
    pub fn check(&self, kind: TokenKind) -> bool {
        self.current.kind == kind
    }

    /// declaration is the entry point for parsing statements.
    pub fn declaration(&mut self) {
        if self.match_token(TokenKind::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    /// var_declaration parses a variable declaration.
    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if self.match_token(TokenKind::Equal) {
            self.expression();
        } else {
            self.emit_byte(OP_NIL);
        }

        self.consume(
            TokenKind::Semicolon,
            "Expect ';' after variable declaration.",
        );
        self.define_variable(global);
    }

    /// define_variable emits global variable declarations.
    /// It supports variable width globals.
    fn define_variable(&mut self, global: usize) {
        if global <= u8::MAX as usize {
            self.emit_byte(OP_DEFINE_GLOBAL);
            self.emit_byte(global as u8);
        } else {
            self.emit_byte(OP_DEFINE_GLOBAL_LONG);
            self.emit_wide_bytes(global);
        }
    }

    /// named_variable gets a global variable.
    fn named_variable(&mut self, name: Token) {
        let arg = self.identifier_constant(name);
        self.get_global_variable(arg);
    }

    /// get_global_variable emits a request to fetch a global variable.
    fn get_global_variable(&mut self, global: usize) {
        if global <= u8::MAX as usize {
            self.emit_byte(OP_GET_GLOBAL);
            self.emit_byte(global as u8);
        } else {
            self.emit_byte(OP_GET_GLOBAL_LONG);
            self.emit_wide_bytes(global);
        }
    }

    /// parse_variable parses the variable declaration and
    /// returns its index from the constants table.
    fn parse_variable(&mut self, msg: &str) -> usize {
        self.consume(TokenKind::Identifier, msg);
        self.identifier_constant(self.previous)
    }

    /// identifier_constant makes the constant and returns
    /// its index from the constants table.
    fn identifier_constant(&mut self, name: Token) -> usize {
        let lexeme = name.lexeme(self.source);
        let id = self.heap.intern_string(lexeme.to_owned());
        self.make_constant(Value::Obj(id))
    }

    /// statement is the distpatch function for all statement
    /// handling.
    fn statement(&mut self) {
        if self.match_token(TokenKind::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    /// print_statement emits a print statement.
    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after value.");
        self.emit_byte(OP_PRINT);
    }

    /// expression_statement parses an expression for its side-effect.
    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenKind::Semicolon, "Expect ';' after expression.");
        self.emit_byte(OP_POP);
    }

    /// expression is the entry point for the expression parser.
    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    /// number emits constant numeric value.
    fn number(&mut self) {
        let lexeme = self.previous.lexeme(self.source);
        let value: f64 = lexeme
            .parse()
            .expect("Scanner produced invalid number literal.");
        self.emit_constant(Value::Number(value));
    }

    /// grouping emits a grouping expression.
    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression.");
    }

    /// unary emits a unary expression.
    fn unary(&mut self) {
        let operator = self.previous.kind;

        self.parse_precedence(Precedence::Unary);

        match operator {
            TokenKind::Minus => self.emit_byte(OP_NEGATE),
            TokenKind::Bang => self.emit_byte(OP_NOT),
            _ => (),
        }
    }

    /// binary emits a binary expression.
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

    /// literal emits a literal value.
    fn literal(&mut self) {
        match self.previous.kind {
            TokenKind::False => self.emit_byte(OP_FALSE),
            TokenKind::True => self.emit_byte(OP_TRUE),
            TokenKind::Nil => self.emit_byte(OP_NIL),
            _ => (),
        }
    }

    /// string interns the string value in the heap and
    /// then emits the constant handle into the chunk.
    fn string(&mut self) {
        let lexeme = self.previous.lexeme(self.source);
        let lexeme = lexeme[1..lexeme.len() - 1].to_owned();
        let id = self.heap.intern_string(lexeme);
        self.emit_constant(Value::Obj(id));
    }

    /// variable emits a request to fetch a variable.
    fn variable(&mut self) {
        self.named_variable(self.previous);
    }

    /// parse_precedence handles precedence ordering, looking
    /// up the different parse operations with rules.
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

    /// write_constant adds a constant value to the chunk.
    pub fn write_constant(&mut self, value: Value) {
        let chunk = self.current_chunk_mut();
        let index = chunk.add_constant(value);

        if index <= u8::MAX as usize {
            self.emit_byte(OP_CONSTANT);
            self.emit_byte(index as u8);
        } else {
            self.emit_byte(OP_CONSTANT_LONG);
            self.emit_wide_bytes(index);
        }
    }

    /// make_constant adds a constant to the current chunk
    /// and returns its index.
    fn make_constant(&mut self, value: Value) -> usize {
        let chunk = self.current_chunk_mut();
        chunk.add_constant(value)
    }

    /// emit_constant emits a constant value into the chunk.
    fn emit_constant(&mut self, value: Value) {
        self.write_constant(value);
    }

    /// emit_byte writes a byte into the current chunk.
    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.line;
        let chunk = self.current_chunk_mut();
        chunk.write(byte, line);
    }

    /// emit_wide_bytes handles the emission of a 3-byte
    /// index.
    fn emit_wide_bytes(&mut self, global: usize) {
        assert!(global <= 0xFF_FFFF, "Too many constants");
        self.emit_byte((global & 0xFF) as u8);
        self.emit_byte(((global >> 8) & 0xFF) as u8);
        self.emit_byte(((global >> 16) & 0xFF) as u8);
    }

    /// end_compiler finishes compilation and must be called
    /// in order to produce the final return statement.
    pub fn end_compiler(&mut self) {
        self.emit_return();

        cfg_select! {
            debug_assertions => {
                if !self.had_error {
                    let disassembler = Disassembler::new(&self.chunk, self.heap);
                    disassembler.disassemble_chunk("code");
                }
            }
            _ => { /* NO-OP */}
        }
    }

    /// emit_return emits a return instruction to the current chunk.
    fn emit_return(&mut self) {
        self.emit_byte(OP_RETURN);
    }

    /// current_chunk_mut takes a mutable borrow to the
    /// current chunk.
    fn current_chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    /// synchronize is used to advance the compiler
    /// past errors until the next statement boundary
    /// when it has experienced a panic condition.
    fn synchronize(&mut self) {
        self.panic_mode = false;

        while self.current.kind != TokenKind::EOF {
            if self.previous.kind == TokenKind::Semicolon {
                return;
            }

            match self.current.kind {
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => self.advance(),
            }
        }
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

    /// get_rule finds the precedence-based parse rule associated
    /// with the TokenKind.
    fn get_rule(&self, kind: TokenKind) -> ParseRule<'src, 'heap> {
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
            TokenKind::String => ParseRule::new(Some(Self::string), None, Precedence::None),
            TokenKind::Identifier => ParseRule::new(Some(Self::variable), None, Precedence::None),
            _ => ParseRule::new(None, None, Precedence::None),
        }
    }
}
