use crate::token::Token;

/// Local represents a variable declared within the active function compiler.
///
/// Each local is assigned a slot in the VM value stack and remains active
/// until the compiler exits the scope where it was declared.
#[derive(Debug, Clone, Copy)]
pub struct Local {
    pub name: Token,
    pub depth: Option<usize>,
}

impl Local {
    /// new creates a new `Local`.
    pub fn new(name: Token, depth: Option<usize>) -> Self {
        Self { name, depth }
    }
}
