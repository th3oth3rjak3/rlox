use crate::token::Token;

/// Local represents a variable declared within the active function compiler.
///
/// Each local is assigned a slot in the VM value stack and remains active
/// until the compiler exits the scope where it was declared.
#[derive(Debug, Clone, Copy)]
pub struct Local {
    #[allow(dead_code)]
    pub name: Token,
    #[allow(dead_code)]
    pub depth: isize,
}

impl Local {
    /// new creates a new `Local`.
    #[allow(dead_code)]
    pub fn new(name: Token, depth: isize) -> Self {
        Self { name, depth }
    }
}
