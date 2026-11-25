#![allow(clippy::module_inception)]

mod bytecode;
mod compiler;
mod errors;
mod expressions;
mod function_context;
mod locals;
mod parse_rule;
mod precedence;
mod scopes;
mod statements;
mod token_cursor;
mod tokens;
mod variables;

pub use compiler::Compiler;
