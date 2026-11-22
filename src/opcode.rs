/// OpCode represents all valid operation codes
/// that the bytecode virtual machine supports.
pub type OpCode = u8;

/// Load a constant value.
pub const OP_CONSTANT: OpCode = 0;

/// Load a wide constant value.
pub const OP_CONSTANT_LONG: OpCode = 1;

/// Return from the current function.
pub const OP_RETURN: OpCode = 2;