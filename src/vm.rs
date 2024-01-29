use crate::common::{Chunk, Disassembler, OpCode};

#[derive(PartialEq, Eq)]
pub enum InterpretMode {
    Debug,
    Release,
}

pub struct VM {
    pub stack: Vec<f64>,
}

impl Disassembler for Vec<f64> {
    fn disassemble(&self) {
        for slot in self.iter() {
            print!("[ {} ]", slot);
        }
        println!();
    }
}

macro_rules! binary_op {
    ($vm:ident, $op:tt) => {
        match ($vm.stack.pop(), $vm.stack.pop()) {
            (Some(b), Some(a)) => $vm.stack.push(b $op a),
            _ => return InterpretResult::RuntimeError,
        }
    };
}

impl VM {
    pub fn new() -> Self {
        VM { stack: vec![] }
    }

    pub fn interpret(&mut self, chunk: Chunk, mode: InterpretMode) -> InterpretResult {
        let mut ip = 0;
        if mode == InterpretMode::Debug {
            chunk.disassemble();
        }
        loop {
            let (instruction, _) = &chunk.code[ip];
            if mode == InterpretMode::Debug {
                instruction.disassemble();
            }
            match instruction {
                OpCode::OpReturn => {
                    self.stack.pop();
                    return InterpretResult::Ok;
                }
                OpCode::OpConstant(value) => {
                    self.stack.push(*value);
                }
                OpCode::OpNegate => match self.stack.pop() {
                    Some(value) => self.stack.push(-value),
                    None => return InterpretResult::RuntimeError,
                },
                OpCode::OpAdd => {
                    binary_op!(self, +);
                }
                OpCode::OpSubtract => {
                    binary_op!(self, -);
                }
                OpCode::OpMultiply => {
                    binary_op!(self, *);
                }
                OpCode::OpDivide => {
                    binary_op!(self, /);
                }
            }
            if mode == InterpretMode::Debug {
                self.stack.disassemble();
            }
            ip += 1;
        }
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
