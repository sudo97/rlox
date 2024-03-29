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

macro_rules! val_constr {
    ($a: expr, $b: expr, <) => {
        Value::Boolean($a < $b)
    };
    ($a: expr, $b: expr, >) => {
        Value::Boolean($a > $b)
    };
    ($a: expr, $b: expr, ==) => {
        Value::Boolean($a == $b)
    };
    ($a: expr, $b: expr, $op:tt) => {
        Value::Number($a $op $b)
    };
}

macro_rules! binary_op {
    ($vm:ident, $op:tt, $line:expr) => {
        match ($vm.stack.pop(), $vm.stack.pop()) {
            (Some(Value::Number(a)), Some(Value::Number(b))) => {
                $vm.stack.push(val_constr!(b, a, $op))
            }
            _ => {
                eprintln!("Error at line {}, Operands must be numbers", $line);
                return InterpretResult::RuntimeError;
            }
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
            use OpCode::*;
            match instruction {
                Return => {
                    self.stack.pop();
                    return InterpretResult::Ok;
                }
                Constant(value) => {
                    self.stack.push(*value);
                }
                Negate => match self.stack.pop() {
                    Some(Value::Number(value)) => self.stack.push(Value::Number(-value)),
                    _ => return InterpretResult::RuntimeError,
                },
                Add => {
                    binary_op!(self, +, line);
                }
                Subtract => {
                    binary_op!(self, -, line);
                }
                Multiply => {
                    binary_op!(self, *, line);
                }
                Divide => {
                    binary_op!(self, /, line);
                }
                Not => match self.stack.pop() {
                    Some(Value::Nil) => self.stack.push(Value::Boolean(true)),
                    Some(Value::Number(x)) => self.stack.push(Value::Boolean(x != 0.0)),
                    Some(Value::Boolean(value)) => self.stack.push(Value::Boolean(!value)),
                    _ => {
                        eprintln!("Error at line {}. Operand must be a boolean", line);
                        return InterpretResult::RuntimeError;
                    }
                },
                Equal => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(a)), Some(Value::Number(b))) => {
                        self.stack.push(Value::Boolean(a == b))
                    }
                    (Some(Value::Boolean(a)), Some(Value::Boolean(b))) => {
                        self.stack.push(Value::Boolean(a == b))
                    }
                    (Some(Value::Nil), Some(Value::Nil)) => self.stack.push(Value::Boolean(true)),
                    _ => {
                        eprintln!("Error at line {}, Operands must be of the same type", line);
                        return InterpretResult::RuntimeError;
                    }
                },
                Greater => {
                    binary_op!(self, >, line);
                }
                Less => {
                    binary_op!(self, <, line);
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
    RuntimeError,
}
