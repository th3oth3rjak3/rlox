use crate::{
    chunk::Chunk, common::Value, compiler::Compiler, debug, opcode::*, stack::ValueStack,
    token::TokenKind,
};

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
                    println!("{}", self.stack);
                    debug::disassemble_instruction(chunk, self.ip);
                }
                _ => { /* NO-OP */ }
            }

            let instruction = self.read_byte(chunk);
            match instruction {
                OP_CONSTANT => {
                    let constant = self.read_constant(chunk);
                    self.push(constant)?;
                }
                OP_CONSTANT_LONG => {
                    let constant = self.read_constant_long(chunk);
                    self.push(constant)?;
                }
                OP_RETURN => {
                    println!("{}", self.stack.pop());
                    return Ok(());
                }
                OP_NEGATE => self.negate()?,
                OP_ADD => self.add()?,
                OP_SUBTRACT => self.subtract()?,
                OP_MULTIPLY => self.multiply()?,
                OP_DIVIDE => self.divide()?,
                _ => {
                    eprintln!("Unsupported opcode: {instruction}");
                    return Err(InterpretResult::RuntimeError);
                }
            }
        }
    }

    /// push places a value on the stack and handles stack overflow errors.
    #[inline(always)]
    fn push(&mut self, value: Value) -> Result<(), InterpretResult> {
        self.stack.push(value).map_err(|err| {
            self.runtime_error(&err.to_string());
            InterpretResult::RuntimeError
        })
    }

    /// runtime_error handles any runtime errors and resets the stack.
    #[cold]
    #[inline(never)]
    fn runtime_error(&mut self, message: &str) {
        eprintln!("{message}");
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

    /// negate does a numerical negation on the value at the top
    /// of the stack. This only works for numbers.
    #[inline(always)]
    fn negate(&mut self) -> Result<(), InterpretResult> {
        match self.stack.pop() {
            Value::Number(n) => self.push(Value::Number(-n)),
            _ => {
                self.runtime_error("Operand must be a number.");
                Err(InterpretResult::RuntimeError)
            }
        }
    }

    /// add does numerical addition on the top two values from the stack.
    #[inline(always)]
    fn add(&mut self) -> Result<(), InterpretResult> {
        let right = self.stack.pop();
        let left = self.stack.pop();

        match (left, right) {
            (Value::Number(l), Value::Number(r)) => self.push(Value::Number(l + r)),
            _ => {
                self.runtime_error("Operands must be two numbers.");
                Err(InterpretResult::RuntimeError)
            }
        }
    }

    /// subtract does numerical subtraction on the top two values from the stack.
    #[inline(always)]
    fn subtract(&mut self) -> Result<(), InterpretResult> {
        let right = self.stack.pop();
        let left = self.stack.pop();

        match (left, right) {
            (Value::Number(l), Value::Number(r)) => self.push(Value::Number(l - r)),
            _ => {
                self.runtime_error("Operands must be two numbers.");
                Err(InterpretResult::RuntimeError)
            }
        }
    }

    /// multiply does numerical multiplication on the top two values from the stack.
    #[inline(always)]
    fn multiply(&mut self) -> Result<(), InterpretResult> {
        let right = self.stack.pop();
        let left = self.stack.pop();

        match (left, right) {
            (Value::Number(l), Value::Number(r)) => self.push(Value::Number(l * r)),
            _ => {
                self.runtime_error("Operands must be two numbers.");
                Err(InterpretResult::RuntimeError)
            }
        }
    }

    /// divide does numerical division on the top two values from the stack.
    #[inline(always)]
    fn divide(&mut self) -> Result<(), InterpretResult> {
        let right = self.stack.pop();
        let left = self.stack.pop();

        match (left, right) {
            (Value::Number(l), Value::Number(r)) => self.push(Value::Number(l / r)),
            _ => {
                self.runtime_error("Operands must be two numbers.");
                Err(InterpretResult::RuntimeError)
            }
        }
    }

    /// compile takes the source code and produces a new chunk to run.
    fn compile(&mut self, source: &str) -> Option<Chunk> {
        let mut compiler = Compiler::new(source);
        compiler.advance();
        compiler.expression();
        compiler.consume(TokenKind::EOF, "Expect end of expression.");

        // TODO: make scanner scan tokens.
        // TODO: have parser make chunk.
        // TODO: return chunk without reference data.

        if compiler.had_error {
            None
        } else {
            Some(compiler.chunk)
        }
    }
}
