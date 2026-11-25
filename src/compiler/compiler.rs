use super::function_context::*;
use super::token_cursor::*;
use crate::{chunk::Chunk, heap::Heap, token::TokenKind};

/// Compiler is a source code compiler that iterates over the source and
/// produces executable bytecode chunks.
pub struct Compiler<'src, 'heap> {
    pub(super) source: &'src str,
    pub(super) tokens: TokenCursor<'src>,
    pub(super) had_error: bool,
    pub(super) panic_mode: bool,
    pub(super) functions: Vec<FunctionContext>,
    pub(super) heap: &'heap mut Heap,
}

impl<'src, 'heap> Compiler<'src, 'heap> {
    /// new creates a new compiler. The compiler must be primed with a
    /// call to advance in order to scan the first token.
    pub fn new(source: &'src str, heap: &'heap mut Heap) -> Self {
        Self {
            tokens: TokenCursor::new(source),
            had_error: false,
            panic_mode: false,
            source,
            functions: vec![FunctionContext::new()],
            heap,
        }
    }

    /// compile compiles all the source code and
    /// produces a chunk when it succeeds.
    pub fn compile(mut self) -> Option<Chunk> {
        self.advance_token();

        while !self.match_current_token(TokenKind::EOF) {
            self.compile_declaration();
        }

        self.end_compiler();

        if self.had_error {
            None
        } else {
            Some(
                self.functions
                    .pop()
                    .expect("compiler must have a script function")
                    .into_chunk(),
            )
        }
    }

    /// Returns the function context currently receiving emitted bytecode.
    pub(super) fn current_function(&self) -> &FunctionContext {
        self.functions
            .last()
            .expect("compiler must have an active function context")
    }

    /// Returns mutable access to the function context currently receiving
    /// emitted bytecode.
    pub(super) fn current_function_mut(&mut self) -> &mut FunctionContext {
        self.functions
            .last_mut()
            .expect("compiler must have an active function context")
    }
}
