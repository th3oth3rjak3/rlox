use std::process::exit;

use crate::chunk::Chunk;
use crate::debug::disassemble_chunk;
use crate::opcode::*;

pub mod chunk;
pub mod common;
pub mod debug;
pub mod opcode;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OP_RETURN);
    let constant = chunk.add_constant(1.2);
    chunk.write(OP_CONSTANT);
    chunk.write(constant as u8);
    disassemble_chunk(&chunk, "test chunk");
    chunk.reset();
    exit(0)
}
