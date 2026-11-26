use std::array;

use super::locals::Local;
use crate::chunk::Chunk;

/// UINT8_COUNT represents the maximum number of local
/// variables that can be stored in a function.
pub const UINT8_COUNT: usize = u8::MAX as usize + 1;

/// FunctionCompiler owns the bytecode and function-specific compile state
/// for one active Lox function.
pub struct FunctionContext {
    chunk: Chunk,
    locals: [Option<Local>; UINT8_COUNT],
    local_count: usize,
    scope_depth: usize,
}

impl Default for FunctionContext {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionContext {
    /// Creates an empty function compiler.
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            locals: array::repeat(None),
            local_count: 0,
            scope_depth: 0,
        }
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

    /// Returns the count of local variables.
    pub fn local_count(&self) -> usize {
        self.local_count
    }

    /// Returns the local variable at an active local-slot index.
    #[allow(dead_code)]
    pub fn local_at(&self, index: usize) -> &Local {
        assert!(index < self.local_count, "local index must be active");

        self.locals[index]
            .as_ref()
            .expect("active local slot must contain a local")
    }

    /// Returns mutable access to the local variable at an active local-slot index.
    #[allow(dead_code)]
    pub fn local_at_mut(&mut self, index: usize) -> &mut Local {
        assert!(index < self.local_count, "local index must be active");

        self.locals[index]
            .as_mut()
            .expect("active local slot must contain a local")
    }

    /// Adds a local variable to the next available local slot.
    pub fn add_local(&mut self, local: Local) {
        self.locals[self.local_count] = Some(local);
        self.local_count += 1;
    }

    /// Removes and returns the most recently added local variable.
    pub fn pop_last_local(&mut self) -> Local {
        assert!(
            self.local_count > 0,
            "cannot remove a local from an empty list"
        );

        self.local_count -= 1;

        self.locals[self.local_count]
            .take()
            .expect("active local slot must contain a local")
    }

    /// Returns the current lexical scope nesting depth.
    pub fn scope_depth(&self) -> usize {
        self.scope_depth
    }

    /// Enters one nested lexical scope.
    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    /// Exits the current lexical scope.
    pub fn end_scope(&mut self) {
        assert!(self.scope_depth > 0, "cannot exit the global scope");
        self.scope_depth -= 1;
    }
}
