use crate::chunk::Chunk;

/// FunctionCompiler owns the bytecode and function-specific compile state
/// for one active Lox function.
#[derive(Default)]
pub struct FunctionCompiler {
    chunk: Chunk,
}

impl FunctionCompiler {
    /// Creates an empty function compiler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the bytecode chunk being built for this function.
    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }

    /// Returns mutable access to the bytecode chunk being built.
    pub fn chunk_mut(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    /// Consumes this function compiler and returns its completed chunk.
    pub fn into_chunk(self) -> Chunk {
        self.chunk
    }
}
