use std::process::exit;

use crate::chunk::Chunk;
use crate::common::Value;
use crate::debug::disassemble_chunk;
use crate::opcode::*;
use crate::virtual_machine::{InterpretResult, VM};

pub mod chunk;
pub mod common;
pub mod debug;
pub mod opcode;
pub mod stack;
pub mod virtual_machine;

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();

    // Evaluates: -((1.2 + 3.4) / 2.0) * 3.0 - 1.1 == -8.0
    chunk.write_constant(Value::Number(1.2), 123);
    chunk.write_constant(Value::Number(3.4), 123);
    chunk.write(OP_ADD, 123);

    chunk.write_constant(Value::Number(2.0), 123);
    chunk.write(OP_DIVIDE, 123);

    chunk.write(OP_NEGATE, 123);

    chunk.write_constant(Value::Number(3.0), 123);
    chunk.write(OP_MULTIPLY, 123);

    chunk.write_constant(Value::Number(1.1), 123);
    chunk.write(OP_SUBTRACT, 123);

    chunk.write(OP_RETURN, 123);

    disassemble_chunk(&chunk, "test chunk");

    let exit_code = match vm.interpret(&chunk) {
        InterpretResult::Ok => 0,
        InterpretResult::CompileError => 1,
        InterpretResult::RuntimeError => 2,
    };

    exit(exit_code);
}
