use super::Compiler;
use super::function_context::UINT8_COUNT;
use super::locals::Local;
use crate::common::Value;
use crate::opcode::*;
use crate::token::{Token, TokenKind};

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// Parses a variable name and returns its constant-table index.
    pub(super) fn parse_variable_name(&mut self, msg: &str) -> usize {
        self.consume_current_token(TokenKind::Identifier, msg);
        self.declare_variable();

        if self.current_function().scope_depth() > 0 {
            return 0;
        }

        self.add_identifier_constant(self.tokens.previous())
    }

    /// Interns an identifier name and adds it to the current constant table.
    pub(super) fn add_identifier_constant(&mut self, name: Token) -> usize {
        let lexeme = name.lexeme(self.source);
        let id = self.heap.intern_string(lexeme.to_owned());
        self.make_constant(Value::Obj(id))
    }

    /// Declares a local variable.
    pub(super) fn declare_variable(&mut self) {
        if self.current_function().scope_depth() == 0 {
            return;
        }

        let name = self.tokens.previous();

        for i in (0..self.current_function().local_count()).rev() {
            let local = self.current_function().local_at(i);
            if local.depth != -1 && (local.depth as usize) < self.current_function().scope_depth() {
                break;
            }

            if name.lexeme(self.source) == local.name.lexeme(self.source) {
                self.error("Already a variable with this name in this scope.");
            }
        }

        self.add_local_variable(name);
    }

    /// Adds a local variable to the function being compiled.
    pub(super) fn add_local_variable(&mut self, name: Token) {
        if self.current_function().local_count() == UINT8_COUNT {
            self.error("Too many local variables in function.");
            return;
        }

        let scope_depth = self.current_function().scope_depth();
        assert!(
            scope_depth <= isize::MAX as usize,
            "scope depth exceeded isize::MAX"
        );

        let local = Local::new(name, scope_depth as isize);
        self.current_function_mut().add_local(local);
    }

    /// Try to resolve a local variable. Returns -1 if not found.
    pub(super) fn resolve_local_variable(&self, name: Token) -> isize {
        for i in (0..self.current_function().local_count()).rev() {
            let local = self.current_function().local_at(i);
            if name.lexeme(self.source) == local.name.lexeme(self.source) {
                return i as isize;
            }
        }

        return -1;
    }

    /// Emits bytecode to define a global variable.
    pub(super) fn emit_define_global(&mut self, global: usize) {
        if self.current_function().scope_depth() > 0 {
            return;
        }

        if global <= u8::MAX as usize {
            self.emit_byte(OP_DEFINE_GLOBAL);
            self.emit_byte(global as u8);
        } else {
            self.emit_byte(OP_DEFINE_GLOBAL_LONG);
            self.emit_wide_bytes(global);
        }
    }

    /// Emits bytecode to read a global variable.
    pub(super) fn emit_get_global(&mut self, global: usize) {
        if global <= u8::MAX as usize {
            self.emit_byte(OP_GET_GLOBAL);
            self.emit_byte(global as u8);
        } else {
            self.emit_byte(OP_GET_GLOBAL_LONG);
            self.emit_wide_bytes(global);
        }
    }

    /// Emits bytecode to assign a global variable.
    pub(super) fn emit_set_global(&mut self, global: usize) {
        if global <= u8::MAX as usize {
            self.emit_byte(OP_SET_GLOBAL);
            self.emit_byte(global as u8);
        } else {
            self.emit_byte(OP_SET_GLOBAL_LONG);
            self.emit_wide_bytes(global);
        }
    }
}
