use crate::chunk::Chunk;
use crate::common::Value;
use crate::opcode::*;

/// disassemble_chunk prints the associated instructions and
/// their offsets in the code for debugging purposes.
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {name} ==");

    let mut offset: usize = 0;

    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

/// disassemble_instruction prints the offset for the current instruction
/// and the related visual representation for each instruction.
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.get_line(offset));
    }

    let instruction = chunk.code[offset];

    match instruction {
        OP_CONSTANT => constant_instruction("OP_CONSTANT", chunk, offset),
        OP_CONSTANT_LONG => constant_long_instruction("OP_CONSTANT_LONG", chunk, offset),
        OP_RETURN => simple_instruction("OP_RETURN", offset),
        OP_NEGATE => simple_instruction("OP_NEGATE", offset),
        OP_ADD => simple_instruction("OP_ADD", offset),
        OP_SUBTRACT => simple_instruction("OP_SUBTRACT", offset),
        OP_MULTIPLY => simple_instruction("OP_MULTIPLY", offset),
        OP_DIVIDE => simple_instruction("OP_DIVIDE", offset),
        OP_NIL => simple_instruction("OP_NIL", offset),
        OP_TRUE => simple_instruction("OP_TRUE", offset),
        OP_FALSE => simple_instruction("OP_FALSE", offset),
        OP_NOT => simple_instruction("OP_NOT", offset),
        OP_EQUAL => simple_instruction("OP_EQUAL", offset),
        OP_NOT_EQUAL => simple_instruction("OP_NOT_EQUAL", offset),
        OP_LESS => simple_instruction("OP_LESS", offset),
        OP_LESS_EQUAL => simple_instruction("OP_LESS_EQUAL", offset),
        OP_GREATER => simple_instruction("OP_GREATER", offset),
        OP_GREATER_EQUAL => simple_instruction("OP_GREATER_EQUAL", offset),
        _ => {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }
    }
}

/// simple_instruction prints the name of a simple instruction
/// and increments the offset.
fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}

/// constant_instruction prints the constant index position and its value.
fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let index = chunk.code[offset + 1] as usize;
    print_constant(name, chunk, index);
    offset + 2
}

/// constant_long_instruction prints the constant long instruction index
/// position and its value.
fn constant_long_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let index = (chunk.code[offset + 1] as usize)
        | ((chunk.code[offset + 2] as usize) << 8)
        | ((chunk.code[offset + 3] as usize) << 16);

    print_constant(name, chunk, index);
    offset + 4
}

fn print_constant(name: &str, chunk: &Chunk, index: usize) {
    print!("{:<16} {:4} '", name, index);
    print_value(chunk.constants[index]);
    println!("'");
}

/// print_value handles printing of the common Value type.
pub fn print_value(value: Value) {
    match value {
        Value::Nil => print!("nil"),
        Value::Bool(b) => print!("{b}"),
        Value::Number(n) => print!("{n}"),
    }
}
