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

/// A nil literal.
pub const OP_NIL: OpCode = 8;

/// A boolean literal true.
pub const OP_TRUE: OpCode = 9;

/// A boolean literal false.
pub const OP_FALSE: OpCode = 10;

/// OP_NOT represents boolean negation like !true (false)
pub const OP_NOT: OpCode = 11;

/// Equality between two values. e.g. lhs == rhs
pub const OP_EQUAL: OpCode = 12;

/// Inequality between two values. e.g. lhs != rhs
pub const OP_NOT_EQUAL: OpCode = 13;

/// Greater comparison. e.g. lhs > rhs
pub const OP_GREATER: OpCode = 14;

/// Greater or equal comparison. e.g. lhs >= rhs
pub const OP_GREATER_EQUAL: OpCode = 15;

/// Less comparison. e.g. lhs < rhs
pub const OP_LESS: OpCode = 16;

/// Less or equal comparison. e.g. lhs <= rhs
pub const OP_LESS_EQUAL: OpCode = 17;

/// Print a value.
pub const OP_PRINT: OpCode = 18;

/// Pop a value from the stack.
pub const OP_POP: OpCode = 19;

/// Define a global variable with a single byte index.
pub const OP_DEFINE_GLOBAL: OpCode = 20;

/// Define a global variable with a 3-byte index.
pub const OP_DEFINE_GLOBAL_LONG: OpCode = 21;

/// Get a global variable.
pub const OP_GET_GLOBAL: OpCode = 22;

/// Get a global variable with a wide index.
pub const OP_GET_GLOBAL_LONG: OpCode = 23;

/// Set a global variable.
pub const OP_SET_GLOBAL: OpCode = 24;

/// Set a global variable with a wide index.
pub const OP_SET_GLOBAL_LONG: OpCode = 25;

/// Get a local variable.
pub const OP_GET_LOCAL: OpCode = 26;

/// Set a local variable.
pub const OP_SET_LOCAL: OpCode = 27;

/// Jump when a condition is false.
pub const OP_JUMP_IF_FALSE: OpCode = 28;

/// Jump unconditionally.
pub const OP_JUMP: OpCode = 29;
