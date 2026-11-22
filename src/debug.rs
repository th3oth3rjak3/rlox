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
fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    
    let instruction = chunk.code[offset];

    match instruction {
        OP_CONSTANT => return constant_instruction("OP_CONSTANT", chunk, offset),
        OP_RETURN => return simple_instruction("OP_RETURN", offset),
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

/// print_value handles printing of the common Value type.
fn print_value(value: Value) {
    print!("{value}");
}