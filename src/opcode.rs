/// OpCode represents all valid operation codes
/// that the bytecode virtual machine supports.
pub type OpCode = u8;

/// Load a constant value.
pub const OP_CONSTANT: OpCode = 0;

/// Load a wide constant value.
pub const OP_CONSTANT_LONG: OpCode = 1;

/// Return from the current function.
pub const OP_RETURN: OpCode = 2;

/// Negate a numeric value. e.g. -> 3 becomes -3
pub const OP_NEGATE: OpCode = 3;

/// Add two values.
pub const OP_ADD: OpCode = 4;

/// Subtract two values.
pub const OP_SUBTRACT: OpCode = 5;

/// Multiply two values.
pub const OP_MULTIPLY: OpCode = 6;

/// Divide two values.
pub const OP_DIVIDE: OpCode = 7;
