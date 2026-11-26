use super::Compiler;
use crate::opcode::*;
use crate::token::TokenKind;

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// Compiles the next top-level declaration or statement and synchronizes
    /// parsing if compilation enters panic mode.
    pub(super) fn compile_declaration(&mut self) {
        if self.match_current_token(TokenKind::Var) {
            self.compile_variable_declaration();
        } else {
            self.compile_statement();
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    /// Compiles a variable declaration, including its optional initializer and
    /// the definition of the resulting local or global variable.
    pub(super) fn compile_variable_declaration(&mut self) {
        let global = self.parse_variable_name("Expect variable name.");

        if self.match_current_token(TokenKind::Equal) {
            self.compile_expression();
        } else {
            self.emit_byte(OP_NIL);
        }

        self.consume_current_token(
            TokenKind::Semicolon,
            "Expect ';' after variable declaration.",
        );
        self.define_variable(global);
    }

    /// Compiles the next statement based on the current token.
    pub(super) fn compile_statement(&mut self) {
        if self.match_current_token(TokenKind::Print) {
            self.compile_print_statement();
        } else if self.match_current_token(TokenKind::LeftBrace) {
            self.begin_scope();
            self.compile_block_statement();
            self.end_scope();
        } else {
            self.compile_expression_statement();
        }
    }

    /// Compiles a print statement and emits bytecode to print its expression value.
    pub(super) fn compile_print_statement(&mut self) {
        self.compile_expression();
        self.consume_current_token(TokenKind::Semicolon, "Expect ';' after value.");
        self.emit_byte(OP_PRINT);
    }

    /// Compiles an expression statement and discards its resulting value.
    pub(super) fn compile_expression_statement(&mut self) {
        self.compile_expression();
        self.consume_current_token(TokenKind::Semicolon, "Expect ';' after expression.");
        self.emit_byte(OP_POP);
    }

    /// Compiles a block statement enclosed in curly braces.
    pub(super) fn compile_block_statement(&mut self) {
        while !self.current_token_is(TokenKind::RightBrace)
            && !self.current_token_is(TokenKind::EOF)
        {
            self.compile_declaration();
        }

        self.consume_current_token(TokenKind::RightBrace, "Expect '}' after block.");
    }
}
