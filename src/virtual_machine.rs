use crate::{
    chunk::Chunk,
    common::Value,
    compiler::Compiler,
    debug::{Disassembler, print_value},
    heap::Heap,
    objects::ObjId,
    opcode::*,
    stack::ValueStack,
    token::TokenKind,
    virtual_machine::InterpretResult::RuntimeError,
};

use std::collections::HashMap;

/// InterpretResult is the result
/// of trying to interpret the source code.
#[derive(Debug, Clone, Copy)]
pub enum InterpretResult {
    CompileError,
    RuntimeError,
}

/// VM is the bytecode virtual machine
/// that executes lox programs.
pub struct VM {
    ip: usize,
    stack: ValueStack,
    heap: Heap,
    globals: HashMap<ObjId, Value>,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    /// new creates a new virtual machine.
    pub fn new() -> Self {
        Self {
            ip: 0,
            stack: ValueStack::new(),
            heap: Heap::new(),
            globals: HashMap::new(),
        }
    }

    /// interpret evaluates source code.
    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretResult> {
        self.ip = 0;
        self.stack.reset();

        match self.compile(source) {
            Some(chunk) => self.run(&chunk),
            None => Err(InterpretResult::CompileError),
        }
    }

    /// run is the hot loop where bytecode evaluation occurs.
    fn run(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        cfg_select! {
            debug_assertions => {
                println!("== VM DIAGNOSTICS ==")
            }
            _ => { /* NO-OP */}
        }

        loop {
            cfg_select! {
                debug_assertions => {
                    self.stack.print(&self.heap);
                    Disassembler::new(chunk, &self.heap).disassemble_instruction(self.ip);
                }
                _ => { /* NO-OP */ }
            }

            let instruction = self.read_byte(chunk);
            match instruction {
                OP_CONSTANT => {
                    let constant = self.read_constant(chunk);
                    self.push(constant, chunk)?;
                }
                OP_CONSTANT_LONG => {
                    let constant = self.read_constant_long(chunk);
                    self.push(constant, chunk)?;
                }
                OP_RETURN => {
                    return Ok(());
                }
                OP_NEGATE => self.negate(chunk)?,
                OP_ADD => self.add(chunk)?,
                OP_SUBTRACT => self.subtract(chunk)?,
                OP_MULTIPLY => self.multiply(chunk)?,
                OP_DIVIDE => self.divide(chunk)?,
                OP_NIL => self.push(Value::Nil, chunk)?,
                OP_TRUE => self.push(Value::Bool(true), chunk)?,
                OP_FALSE => self.push(Value::Bool(false), chunk)?,
                OP_NOT => self.not(chunk)?,
                OP_EQUAL => self.equal(chunk)?,
                OP_NOT_EQUAL => self.not_equal(chunk)?,
                OP_LESS => self.less(chunk)?,
                OP_LESS_EQUAL => self.less_equal(chunk)?,
                OP_GREATER => self.greater(chunk)?,
                OP_GREATER_EQUAL => self.greater_equal(chunk)?,
                OP_PRINT => {
                    print_value(self.stack.pop(), &self.heap);
                    println!();
                }
                OP_POP => {
                    self.stack.pop();
                }
                OP_DEFINE_GLOBAL => {
                    let name = self.read_string(chunk);
                    self.globals.insert(name, self.stack.peek(0));
                    self.stack.pop();
                }
                OP_DEFINE_GLOBAL_LONG => {
                    let name = self.read_string_long(chunk);
                    self.globals.insert(name, self.stack.peek(0));
                    self.stack.pop();
                }
                OP_GET_GLOBAL => {
                    let name = self.read_string(chunk);
                    if let Some(&value) = self.globals.get(&name) {
                        self.push(value, chunk)?;
                    } else {
                        let var_name = self.heap.as_string(name);
                        self.runtime_error(chunk, &format!("Undefined variable '{var_name}'"));
                        return Err(RuntimeError);
                    }
                }
                OP_GET_GLOBAL_LONG => {
                    let name = self.read_string_long(chunk);
                    if let Some(&value) = self.globals.get(&name) {
                        self.push(value, chunk)?;
                    } else {
                        let var_name = self.heap.as_string(name);
                        self.runtime_error(chunk, &format!("Undefined variable '{var_name}'"));
                        return Err(RuntimeError);
                    }
                }
                OP_SET_GLOBAL => {
                    let name = self.read_string(chunk);
                    if let Some(value) = self.globals.get_mut(&name) {
                        *value = self.stack.peek(0);
                    } else {
                        let var_name = self.heap.as_string(name);
                        self.runtime_error(chunk, &format!("Undefined variable '{var_name}'"));
                        return Err(RuntimeError);
                    }
                }
                OP_SET_GLOBAL_LONG => {
                    let name = self.read_string_long(chunk);
                    if let Some(value) = self.globals.get_mut(&name) {
                        *value = self.stack.peek(0);
                    } else {
                        let var_name = self.heap.as_string(name);
                        self.runtime_error(chunk, &format!("Undefined variable '{var_name}'"));
                        return Err(RuntimeError);
                    }
                }
                _ => {
                    eprintln!("Unsupported opcode: {instruction}");
                    return Err(InterpretResult::RuntimeError);
                }
            }
        }
    }

    /// push places a value on the stack and handles stack overflow errors.
    #[inline(always)]
    fn push(&mut self, value: Value, chunk: &Chunk) -> Result<(), InterpretResult> {
        self.stack.push(value).map_err(|err| {
            self.runtime_error(chunk, &err.to_string());
            RuntimeError
        })
    }

    /// runtime_error handles any runtime errors and resets the stack.
    #[cold]
    #[inline(never)]
    fn runtime_error(&mut self, chunk: &Chunk, message: &str) {
        eprintln!("{}", message);

        let instruction = self.ip - 1;
        let line = chunk.get_line(instruction);
        eprintln!("[line {}] in script", line);

        self.stack.reset();
    }

    /// read_byte gets the byte at the instruction pointer
    /// and increments the internal pointer so it's ready
    /// for the next read.
    #[inline(always)]
    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    /// read_constant reads the index at the instruction
    /// pointer and then looks up the constant in the constants
    /// list inside the chunk.
    #[inline(always)]
    fn read_constant(&mut self, chunk: &Chunk) -> Value {
        let index = self.read_byte(chunk);
        chunk.constants[index as usize]
    }

    /// read_constant_long reads a constant where the index is greater
    /// than 255 using a 3-byte operand.
    #[inline(always)]
    fn read_constant_long(&mut self, chunk: &Chunk) -> Value {
        let index = (self.read_byte(chunk) as usize)
            | (self.read_byte(chunk) as usize) << 8
            | (self.read_byte(chunk) as usize) << 16;

        chunk.constants[index]
    }

    /// read_string reads the constant. This function
    /// returns the object id, the caller is expected to lookup
    /// the string itself if that's needed from the heap.
    #[inline(always)]
    fn read_string(&mut self, chunk: &Chunk) -> ObjId {
        self.read_constant(chunk).as_obj()
    }

    /// read_string_long reads a constant that has a 3-byte
    /// index. This function returns the object id, the caller
    /// is expected to lookup the string itself if that's needed
    /// from the heap.
    #[inline(always)]
    fn read_string_long(&mut self, chunk: &Chunk) -> ObjId {
        self.read_constant_long(chunk).as_obj()
    }

    /// negate does a numerical negation on the value at the top
    /// of the stack. This only works for numbers.
    #[inline(always)]
    fn negate(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        if !self.stack.peek(0).is_number() {
            self.runtime_error(chunk, "Operand must be a number.");
            return Err(RuntimeError);
        }
        let value = self.stack.pop().as_number();
        self.push(Value::Number(-value), chunk)
    }

    /// add does numerical addition on the top two values from the stack.
    #[inline(always)]
    fn add(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        if self.stack.peek(0).is_string(&self.heap) && self.stack.peek(1).is_string(&self.heap) {
            return self.concatenate(chunk);
        } else if self.stack.peek(0).is_number() && self.stack.peek(1).is_number() {
            let right = self.stack.pop().as_number();
            let left = self.stack.pop().as_number();
            return self.push(Value::Number(left + right), chunk);
        }

        self.runtime_error(chunk, "Operands must be numbers or strings.");
        Err(RuntimeError)
    }

    /// concatenate joins two string values together.
    #[inline(always)]
    fn concatenate(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        let b = self.heap.as_string(self.stack.pop().as_obj());
        let a = self.heap.as_string(self.stack.pop().as_obj());
        let new_str = format!("{a}{b}");
        let id = self.heap.intern_string(new_str);
        self.push(Value::Obj(id), chunk)
    }

    /// subtract does numerical subtraction on the top two values from the stack.
    #[inline(always)]
    fn subtract(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        if !self.stack.peek(0).is_number() || !self.stack.peek(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(RuntimeError);
        }

        let right = self.stack.pop().as_number();
        let left = self.stack.pop().as_number();

        self.push(Value::Number(left - right), chunk)
    }

    /// multiply does numerical multiplication on the top two values from the stack.
    #[inline(always)]
    fn multiply(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        if !self.stack.peek(0).is_number() || !self.stack.peek(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(RuntimeError);
        }

        let right = self.stack.pop().as_number();
        let left = self.stack.pop().as_number();

        self.push(Value::Number(left * right), chunk)
    }

    /// divide does numerical division on the top two values from the stack.
    #[inline(always)]
    fn divide(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        if !self.stack.peek(0).is_number() || !self.stack.peek(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(RuntimeError);
        }

        let right = self.stack.pop().as_number();
        let left = self.stack.pop().as_number();

        self.push(Value::Number(left / right), chunk)
    }

    /// not handles boolean negation.
    #[inline(always)]
    fn not(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        let value = self.stack.pop().is_falsey();
        self.push(Value::Bool(value), chunk)
    }

    /// equal handles equality.
    #[inline(always)]
    fn equal(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        let b = self.stack.pop();
        let a = self.stack.pop();
        self.push(Value::Bool(a == b), chunk)
    }

    /// not_equal handles inequality.
    #[inline(always)]
    fn not_equal(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        let b = self.stack.pop();
        let a = self.stack.pop();
        self.push(Value::Bool(a != b), chunk)
    }

    /// less does a numeric comparison for lhs < rhs.
    #[inline(always)]
    fn less(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        if !self.stack.peek(0).is_number() || !self.stack.peek(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(RuntimeError);
        }

        let right = self.stack.pop().as_number();
        let left = self.stack.pop().as_number();

        self.push(Value::Bool(left < right), chunk)
    }

    /// less_equal does a numeric comparison for lhs <= rhs.
    #[inline(always)]
    fn less_equal(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        if !self.stack.peek(0).is_number() || !self.stack.peek(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(RuntimeError);
        }

        let right = self.stack.pop().as_number();
        let left = self.stack.pop().as_number();

        self.push(Value::Bool(left <= right), chunk)
    }

    /// greater does a numeric comparison for lhs > rhs.
    #[inline(always)]
    fn greater(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        if !self.stack.peek(0).is_number() || !self.stack.peek(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(RuntimeError);
        }

        let right = self.stack.pop().as_number();
        let left = self.stack.pop().as_number();

        self.push(Value::Bool(left > right), chunk)
    }

    /// greater_equal does a numeric comparison for lhs >= rhs.
    #[inline(always)]
    fn greater_equal(&mut self, chunk: &Chunk) -> Result<(), InterpretResult> {
        if !self.stack.peek(0).is_number() || !self.stack.peek(1).is_number() {
            self.runtime_error(chunk, "Operands must be numbers.");
            return Err(RuntimeError);
        }

        let right = self.stack.pop().as_number();
        let left = self.stack.pop().as_number();

        self.push(Value::Bool(left >= right), chunk)
    }

    /// compile takes the source code and produces a new chunk to run.
    fn compile(&mut self, source: &str) -> Option<Chunk> {
        let mut compiler = Compiler::new(source, &mut self.heap);
        compiler.advance();

        while !compiler.match_token(TokenKind::EOF) {
            compiler.declaration();
        }

        compiler.end_compiler();

        if compiler.had_error {
            None
        } else {
            Some(compiler.chunk)
        }
    }
}
