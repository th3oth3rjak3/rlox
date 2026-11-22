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
    chunk.write(OP_RETURN, 123);

    for i in 0..258 {
        chunk.write_constant(i as f64, 123);
    }

    disassemble_chunk(&chunk, "test chunk");
    chunk.reset();
    exit(0)
}
