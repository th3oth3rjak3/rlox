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
        } else if self.match_current_token(TokenKind::For) {
            self.compile_for_statement();
        } else if self.match_current_token(TokenKind::If) {
            self.compile_if_statement();
        } else if self.match_current_token(TokenKind::While) {
            self.compile_while_statement();
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

    /// Compile an if statement with an optional else branch.
    pub(super) fn compile_if_statement(&mut self) {
        self.consume_current_token(TokenKind::LeftParen, "Expect '(' after 'if'.");
        self.compile_expression();
        self.consume_current_token(TokenKind::RightParen, "Expect ')' after condition.");
        let then_jump = self.emit_jump(OP_JUMP_IF_FALSE);
        self.emit_byte(OP_POP);
        self.compile_statement();
        let else_jump = self.emit_jump(OP_JUMP);
        self.patch_jump(then_jump);
        self.emit_byte(OP_POP);

        if self.match_current_token(TokenKind::Else) {
            self.compile_statement();
        }

        self.patch_jump(else_jump);
    }

    /// Compile a while loop statement.
    pub(super) fn compile_while_statement(&mut self) {
        let loop_start = self.current_chunk().code.len();
        self.consume_current_token(TokenKind::LeftParen, "Expect '(' after 'while'.");
        self.compile_expression();
        self.consume_current_token(TokenKind::RightParen, "Expect ')' after condition.");

        let exit_jump = self.emit_jump(OP_JUMP_IF_FALSE);
        self.emit_byte(OP_POP);
        self.compile_statement();
        self.emit_loop(loop_start);

        self.patch_jump(exit_jump);
        self.emit_byte(OP_POP);
    }

    /// Compile a for loop statement.
    pub(super) fn compile_for_statement(&mut self) {
        self.begin_scope();
        self.consume_current_token(TokenKind::LeftParen, "Expect '(' after 'for'.");
        if self.match_current_token(TokenKind::Semicolon) {
            // No Initializer
        } else if self.match_current_token(TokenKind::Var) {
            self.compile_variable_declaration();
        } else {
            self.compile_expression_statement();
        }

        let mut loop_start = self.current_chunk().code.len();
        let mut exit_jump: Option<usize> = None;

        if !self.match_current_token(TokenKind::Semicolon) {
            self.compile_expression();
            self.consume_current_token(TokenKind::Semicolon, "Expect ';' after loop condition.");
            exit_jump = Some(self.emit_jump(OP_JUMP_IF_FALSE));
            self.emit_byte(OP_POP); // condition
        }

        if !self.match_current_token(TokenKind::RightParen) {
            let body_jump = self.emit_jump(OP_JUMP);
            let increment_start = self.current_chunk().code.len();
            self.compile_expression();
            self.emit_byte(OP_POP);
            self.consume_current_token(TokenKind::RightParen, "Expect ')' after for clauses");

            self.emit_loop(loop_start);
            loop_start = increment_start;
            self.patch_jump(body_jump);
        }

        self.compile_statement();
        self.emit_loop(loop_start);

        if let Some(exit) = exit_jump {
            self.patch_jump(exit);
            self.emit_byte(OP_POP); // condition
        }
        self.end_scope();
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
