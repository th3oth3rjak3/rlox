use std::io::{Write, stdin, stdout};

use crate::virtual_machine::{InterpretResult, VM};

pub mod chunk;
pub mod common;
pub mod debug;
pub mod opcode;
pub mod scanner;
pub mod stack;
pub mod token;
pub mod virtual_machine;

fn main() {
    let mut args = std::env::args();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        let path = args
            .nth(1)
            .expect("args[1] should be present since len is 2");

        run_file(path);
    } else {
        eprintln!("Usage: rlox [path]");
        std::process::exit(64);
    }
}

fn repl() {
    let mut vm = VM::new();

    let mut line_buffer = String::new();
    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!("> ");
        if let Err(_) = stdout.flush() {
            println!();
            break;
        }

        line_buffer.clear();
        if let Err(_) = stdin.read_line(&mut line_buffer) {
            println!();
            break;
        }

        _ = vm.interpret(&line_buffer);
    }
}

fn run_file(file: String) {
    let source = load_source(&file);
    let mut vm = VM::new();

    let exit_code = match vm.interpret(&source) {
        Ok(()) => 0,
        Err(InterpretResult::RuntimeError) => 70,
        Err(InterpretResult::CompileError) => 65,
    };

    std::process::exit(exit_code);
}

fn load_source(file: &str) -> String {
    match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Could not open file: {file}");
            std::process::exit(74);
        }
    }
}
