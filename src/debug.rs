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
        OP_CONSTANT => return constant_instruction("OP_CONSTANT", chunk, offset),
        OP_CONSTANT_LONG => return constant_long_instruction("OP_CONSTANT_LONG", chunk, offset),
        OP_RETURN => return simple_instruction("OP_RETURN", offset),
        OP_NEGATE => return simple_instruction("OP_NEGATE", offset),
        OP_ADD => return simple_instruction("OP_ADD", offset),
        OP_SUBTRACT => return simple_instruction("OP_SUBTRACT", offset),
        OP_MULTIPLY => return simple_instruction("OP_MULTIPLY", offset),
        OP_DIVIDE => return simple_instruction("OP_DIVIDE", offset),

        _ => {
            println!("Unknown opcode {}", instruction);
            return offset + 1;
        }
    }
}

/// simple_instruction prints the name of a simple instruction
/// and increments the offset.
fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{name}");
    return offset + 1;
}

/// constant_instruction prints the constant index position and its value.
fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    print!("{:<16} {:4} '", name, constant);
    print_value(chunk.constants[constant as usize]);
    print!("'\n");
    return offset + 2;
}

/// constant_long_instruction prints the constant long instruction index
/// position and its value.
fn constant_long_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let index = (chunk.code[offset + 1] as usize)
        | ((chunk.code[offset + 2] as usize) << 8)
        | ((chunk.code[offset + 3] as usize) << 16);

    println!("{:<16} {:4} '{}'", name, index, chunk.constants[index]);

    offset + 4
}

/// print_value handles printing of the common Value type.
fn print_value(value: Value) {
    print!("{value}");
}
