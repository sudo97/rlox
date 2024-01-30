use crate::common::{Chunk, Disassembler, OpCode, Value};

#[derive(PartialEq, Eq, Clone, Copy)]
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
    ($vm:ident, $op:tt, $line:expr) => {
        match ($vm.stack.pop(), $vm.stack.pop()) {
            (Some(Value::Number(a)), Some(Value::Number(b))) => $vm.stack.push(Value::Number(b $op a)),
            _ => {
                eprintln!("Error at line {}, Operands must be numbers", $line);
                return InterpretResult::RuntimeError
            },
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
            let (instruction, line) = &chunk.code[ip];
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
                    binary_op!(self, +, line);
                }
                OpCode::Subtract => {
                    binary_op!(self, -, line);
                }
                OpCode::Multiply => {
                    binary_op!(self, *, line);
                }
                OpCode::Divide => {
                    binary_op!(self, /, line);
                }
                OpCode::Not => match self.stack.pop() {
                    Some(Value::Boolean(value)) => self.stack.push(Value::Boolean(!value)),
                    _ => {
                        eprintln!("Error at line {}. Operand must be a boolean", line);
                        return InterpretResult::RuntimeError;
                    }
                },
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
    RuntimeError,
}
