use crate::{common::Value, debug::print_value, heap::Heap};
use std::{array, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackOverflow;

impl Display for StackOverflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Stack overflow.")
    }
}

impl std::error::Error for StackOverflow {}

/// STACK_MAX is the maximum number of values that
/// can be placed in the current stack at a given
/// time. This value may be adjusted later depending
/// on profiling and needs.
const STACK_MAX: usize = 512;

/// ValueStack is a stack implementation that is
/// meant to be used in the VM during hot path operations.
#[derive(Debug, Clone)]
pub struct ValueStack {
    stack_top: usize,
    backing_storage: [Value; STACK_MAX],
}

impl Default for ValueStack {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueStack {
    /// new creates a new value stack filled with nil values.
    pub fn new() -> Self {
        Self {
            stack_top: 0,
            backing_storage: array::from_fn(|_| Value::Nil),
        }
    }

    /// reset places the stack in a new condition.
    pub fn reset(&mut self) {
        self.stack_top = 0;
    }

    /// push places a new value at the top of the stack when there is room.
    /// When the stack is out of room, this will produce a stack overflow
    /// error. This stack overflow error should be used to recover the vm
    /// without a program crash in order to produce meaningful diagnostics
    /// to the user.
    pub fn push(&mut self, value: Value) -> Result<(), StackOverflow> {
        if self.stack_top == STACK_MAX {
            return Err(StackOverflow);
        }

        self.backing_storage[self.stack_top] = value;
        self.stack_top += 1;
        Ok(())
    }

    /// pop returns the top value from the stack. Calling pop on an
    /// empty stack would be considered a programmer error.
    pub fn pop(&mut self) -> Value {
        debug_assert!(self.stack_top > 0, "Pop called on an empty stack.");

        self.stack_top -= 1;
        self.backing_storage[self.stack_top]
    }

    /// peek returns the value `distance` positions below the stack top.
    /// Calling peek with a `distance` that is greater than the stack top
    /// would be considered a programmer error.
    pub fn peek(&self, distance: usize) -> Value {
        debug_assert!(
            distance < self.stack_top,
            "Peek distance ({distance}) out of bounds for stack height ({})",
            self.stack_top
        );

        self.backing_storage[self.stack_top - 1 - distance]
    }

    /// Peek at the value on the stack at the index.
    pub fn peek_at(&self, index: usize) -> Value {
        debug_assert!(
            index < self.stack_top,
            "Peek at index ({index}) out of bounds for stack height ({})",
            self.stack_top
        );
        self.backing_storage[index]
    }

    /// Set the stack value at the provided index.
    pub fn set_at(&mut self, index: usize, value: Value) {
        debug_assert!(
            index < self.stack_top,
            "Set at index ({index}) is out of bounds for stack height ({})",
            self.stack_top
        );
        self.backing_storage[index] = value;
    }

    /// as_slice returns the active values in the stack.
    pub fn as_slice(&self) -> &[Value] {
        &self.backing_storage[0..self.stack_top]
    }

    /// print writes the current value stack to stdout.
    pub fn print(&self, heap: &Heap) {
        let slice = self.as_slice();

        if slice.is_empty() {
            println!("[ ]");
            return;
        }

        for value in slice {
            print!("[ ");
            print_value(*value, heap);
            print!(" ]");
        }
        println!();
    }
}
