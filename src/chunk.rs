use crate::common::Value;

/// Chunk represents a compiled chunk of bytecode
/// for a single file program.
#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Chunk {
    /// new creates a new Chunk.
    pub fn new() -> Self {
        Self{
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    /// reset sets the state of the chunk to its original
    /// values when created with new. It's the equivalent 
    /// of freeChunk in the clox implementation.
    pub fn reset(&mut self) {
        self.code = Vec::new();
        self.constants = Vec::new();
    }

    /// write adds the byte to the chunk code.
    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        return self.constants.len() - 1;
    }
}