use crate::chunk::Chunk;
use crate::common::Value;
use crate::heap::Heap;
use crate::opcode::*;

/// Disassembler formats bytecode instructions and their operands
/// for debugging and diagnostic output.
pub struct Disassembler<'a> {
    chunk: &'a Chunk,
    heap: &'a Heap,
}

impl<'a> Disassembler<'a> {
    /// new creates a new Disassembler for the given chunk and heap.
    pub fn new(chunk: &'a Chunk, heap: &'a Heap) -> Self {
        Self { chunk, heap }
    }

    /// disassemble_chunk prints all instructions and their offsets
    /// in the chunk under the given header name.
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {name} ==");

        let mut offset: usize = 0;

        while offset < self.chunk.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    /// disassemble_instruction prints the offset, line number, and
    /// formatted representation for the instruction at offset.
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.chunk.get_line(offset) == self.chunk.get_line(offset - 1) {
            print!("   | ");
        } else {
            print!("{:4} ", self.chunk.get_line(offset));
        }

        let instruction = self.chunk.code[offset];

        match instruction {
            OP_CONSTANT => self.constant_instruction("OP_CONSTANT", offset),
            OP_CONSTANT_LONG => self.constant_long_instruction("OP_CONSTANT_LONG", offset),
            OP_RETURN => self.simple_instruction("OP_RETURN", offset),
            OP_NEGATE => self.simple_instruction("OP_NEGATE", offset),
            OP_ADD => self.simple_instruction("OP_ADD", offset),
            OP_SUBTRACT => self.simple_instruction("OP_SUBTRACT", offset),
            OP_MULTIPLY => self.simple_instruction("OP_MULTIPLY", offset),
            OP_DIVIDE => self.simple_instruction("OP_DIVIDE", offset),
            OP_NIL => self.simple_instruction("OP_NIL", offset),
            OP_TRUE => self.simple_instruction("OP_TRUE", offset),
            OP_FALSE => self.simple_instruction("OP_FALSE", offset),
            OP_NOT => self.simple_instruction("OP_NOT", offset),
            OP_EQUAL => self.simple_instruction("OP_EQUAL", offset),
            OP_NOT_EQUAL => self.simple_instruction("OP_NOT_EQUAL", offset),
            OP_LESS => self.simple_instruction("OP_LESS", offset),
            OP_LESS_EQUAL => self.simple_instruction("OP_LESS_EQUAL", offset),
            OP_GREATER => self.simple_instruction("OP_GREATER", offset),
            OP_GREATER_EQUAL => self.simple_instruction("OP_GREATER_EQUAL", offset),
            OP_PRINT => self.simple_instruction("OP_PRINT", offset),
            OP_POP => self.simple_instruction("OP_POP", offset),
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }

    /// simple_instruction prints the name of a simple instruction
    /// and increments the offset.
    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    /// constant_instruction prints the constant index position and its value.
    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let index = self.chunk.code[offset + 1] as usize;
        self.print_constant(name, index);
        offset + 2
    }

    /// constant_long_instruction prints the constant long instruction index
    /// position and its value.
    fn constant_long_instruction(&self, name: &str, offset: usize) -> usize {
        let index = (self.chunk.code[offset + 1] as usize)
            | ((self.chunk.code[offset + 2] as usize) << 8)
            | ((self.chunk.code[offset + 3] as usize) << 16);

        self.print_constant(name, index);
        offset + 4
    }

    /// print_constant prints the instruction name, constant index, and
    /// formatted constant value.
    fn print_constant(&self, name: &str, index: usize) {
        print!("{:<16} {:4} '", name, index);
        print_value(self.chunk.constants[index], self.heap);
        println!("'");
    }
}

/// print_value handles printing of the common Value type.
pub fn print_value(value: Value, heap: &Heap) {
    match value {
        Value::Nil => print!("nil"),
        Value::Bool(b) => print!("{b}"),
        Value::Number(n) => print!("{n}"),
        Value::Obj(id) => {
            if heap.is_string(id) {
                print!("{}", heap.as_string(id))
            }
        }
    }
}
