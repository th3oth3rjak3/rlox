use crate::chunk::Chunk;
use crate::common::Value;
use crate::compiler::Compiler;
use crate::opcode::*;

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// write_constant adds a constant value to the chunk.
    pub(super) fn write_constant(&mut self, value: Value) {
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
    pub(super) fn make_constant(&mut self, value: Value) -> usize {
        let chunk = self.current_chunk_mut();
        chunk.add_constant(value)
    }

    /// emit_constant emits a constant value into the chunk.
    pub(super) fn emit_constant(&mut self, value: Value) {
        self.write_constant(value);
    }

    /// emit_byte writes a byte into the current chunk.
    pub(super) fn emit_byte(&mut self, byte: u8) {
        let line = self.tokens.previous().line;
        let chunk = self.current_chunk_mut();
        chunk.write(byte, line);
    }

    /// emit_return emits a return instruction to the current chunk.
    pub(super) fn emit_return(&mut self) {
        self.emit_byte(OP_RETURN);
    }

    /// emit_wide_bytes handles the emission of a 3-byte
    /// index.
    pub(super) fn emit_wide_bytes(&mut self, global: usize) {
        assert!(global <= 0xFF_FFFF, "Too many constants");
        self.emit_byte((global & 0xFF) as u8);
        self.emit_byte(((global >> 8) & 0xFF) as u8);
        self.emit_byte(((global >> 16) & 0xFF) as u8);
    }

    /// current_chunk takes an immutable borrow to the current
    /// chunk.
    #[allow(dead_code)]
    pub(super) fn current_chunk(&self) -> &Chunk {
        self.current_function().chunk()
    }

    /// current_chunk_mut takes a mutable borrow to the
    /// current chunk.
    pub(super) fn current_chunk_mut(&mut self) -> &mut Chunk {
        self.current_function_mut().chunk_mut()
    }

    /// end_compiler finishes compilation and must be called
    /// in order to produce the final return statement.
    pub(super) fn end_compiler(&mut self) {
        self.emit_return();

        cfg_select! {
            debug_assertions => {
                if !self.had_error {
                    let disassembler = crate::debug::Disassembler::new(self.current_chunk(), self.heap);
                    disassembler.disassemble_chunk("code");
                }
            }
            _ => { /* NO-OP */}
        }
    }
}
