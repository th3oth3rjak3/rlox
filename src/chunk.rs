use crate::{common::Value, opcode::*};

/// LineRun represents source line information
/// associated with byte offsets. It's used as
/// a form of run length encoding to reduce
/// line storage size in memory.
#[derive(Debug, Clone, Copy)]
pub struct LineRun {
    line: usize,
    count: usize,
}

/// Chunk represents a compiled chunk of bytecode
/// for a single file program.
#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<LineRun>,
}

impl Chunk {
    /// new creates a new Chunk.
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    /// reset sets the state of the chunk to its original
    /// values when created with new. It's the equivalent
    /// of freeChunk in the clox implementation.
    pub fn reset(&mut self) {
        self.code = Vec::new();
        self.constants = Vec::new();
        self.lines = Vec::new();
    }

    /// write adds the byte to the chunk code.
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);

        if let Some(last) = self.lines.last_mut() {
            if last.line == line {
                last.count += 1;
                return;
            }
        }

        self.lines.push(LineRun { line, count: 1 });
    }

    /// get_line retrieves the line of the source code file
    /// related to the bytecode offset.
    pub fn get_line(&self, offset: usize) -> usize {
        let mut current_offset = 0;

        for run in &self.lines {
            current_offset += run.count;

            if offset < current_offset {
                return run.line;
            }
        }

        panic!("Invalid bytecode offset");
    }

    /// write_constant adds a constant value to the chunk.
    pub fn write_constant(&mut self, value: Value, line: usize) {
        let index = self.add_constant(value);

        if index <= u8::MAX as usize {
            self.write(OP_CONSTANT, line);
            self.write(index as u8, line);
        } else {
            assert!(index <= 0xFF_FFFF, "Too many constants");

            self.write(OP_CONSTANT_LONG, line);
            self.write((index & 0xFF) as u8, line);
            self.write(((index >> 8) & 0xFF) as u8, line);
            self.write(((index >> 16) & 0xFF) as u8, line);
        }
    }

    /// add_constant adds the constant value and produces
    /// the index value used to later lookup.
    fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        return self.constants.len() - 1;
    }
}
