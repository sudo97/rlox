use crate::common::{Chunk, Disassembler, OpCode, Value};

#[derive(PartialEq, Eq)]
pub enum InterpretMode {
    Debug,
    Release,
}

pub struct VM {
    pub stack: Vec<Value>,
}

impl Disassembler for Vec<Value> {
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
            (Some(Value::Number(a)), Some(Value::Number(b))) => $vm.stack.push(Value::Number(b $op a)),
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
                OpCode::Return => {
                    self.stack.pop();
                    return InterpretResult::Ok;
                }
                OpCode::Constant(value) => {
                    self.stack.push(*value);
                }
                OpCode::Negate => match self.stack.pop() {
                    Some(Value::Number(value)) => self.stack.push(Value::Number(-value)),
                    _ => return InterpretResult::RuntimeError,
                },
                OpCode::Add => {
                    binary_op!(self, +);
                }
                OpCode::Subtract => {
                    binary_op!(self, -);
                }
                OpCode::Multiply => {
                    binary_op!(self, *);
                }
                OpCode::Divide => {
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
