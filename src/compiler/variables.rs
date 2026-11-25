use super::Compiler;
use crate::common::Value;
use crate::opcode::*;
use crate::token::{Token, TokenKind};

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// Parses a variable name and returns its constant-table index.
    pub(super) fn parse_variable_name(&mut self, msg: &str) -> usize {
        self.consume_current_token(TokenKind::Identifier, msg);
        self.add_identifier_constant(self.tokens.previous())
    }

    /// Interns an identifier name and adds it to the current constant table.
    pub(super) fn add_identifier_constant(&mut self, name: Token) -> usize {
        let lexeme = name.lexeme(self.source);
        let id = self.heap.intern_string(lexeme.to_owned());
        self.make_constant(Value::Obj(id))
    }

    /// Emits bytecode to define a global variable.
    pub(super) fn emit_define_global(&mut self, global: usize) {
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
