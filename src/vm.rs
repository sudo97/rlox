use std::rc::Rc;

use crate::common::{Chunk, Disassembler, Obj, OpCode, Value};
// use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum InterpretMode {
    Debug,
    Release,
}

pub struct VM {
    pub stack: Vec<Rc<Value>>,
    // pub global_env: HashMap<String, Value>,
}

impl Disassembler for Vec<Rc<Value>> {
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

macro_rules! concat_strings {
    ($a:expr, $b:expr) => {
        Value::Obj(Obj::String(format!("{}{}", $b, $a)))
    };
}

macro_rules! binary_op {
    ($vm:ident, $op:tt, $line:expr) => {
        match ($vm.stack.pop(), $vm.stack.pop()) {
            (Some(a), Some(b)) => match (&*a, &*b) {
                (Value::Number(a), Value::Number(b)) => {
                    $vm.stack.push(Rc::new(val_constr!(b, a, $op)));
                }
                (Value::Obj(Obj::String(a)), Value::Obj(Obj::String(b))) => {
                    $vm.stack.push(Rc::new(concat_strings!(a, b)));
                }
                _ => {
                    eprintln!("Error at line {}, Operands are incompatible", $line);
                    return InterpretResult::RuntimeError;
                }
            },
            _ => {
                eprintln!("Error at line {}, Operands are incompatible", $line);
                return InterpretResult::RuntimeError;
            }
        }
    };
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: vec![],
            // global_env: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk, mode: InterpretMode) -> InterpretResult {
        // self.global_env
        //     .insert("Let's try it".into(), Value::Boolean(false)); // Temporarily
        let mut ip = 0;
        if mode == InterpretMode::Debug {
            println!("Disassembling...");
            chunk.disassemble();
            println!("Interpreting...");
        }
        loop {
            let (instruction, line) = &chunk.code[ip];
            if mode == InterpretMode::Debug {
                print!("// ");
                instruction.disassemble();
            }
            use OpCode::*;
            match instruction {
                Return => {
                    self.stack.pop();
                    return InterpretResult::Ok;
                }
                Constant(value) => self.stack.push(Rc::clone(value)),
                Negate => match self.stack.pop() {
                    Some(val) => match *val {
                        Value::Number(n) => self.stack.push(Rc::new(Value::Number(-n))),
                        _ => return InterpretResult::RuntimeError,
                    },
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
                    Some(val) => match *val {
                        Value::Nil => self.stack.push(Rc::new(Value::Boolean(true))),
                        Value::Number(x) => self.stack.push(Rc::new(Value::Boolean(x != 0.0))),
                        Value::Boolean(value) => self.stack.push(Rc::new(Value::Boolean(!value))),
                        Value::Obj(_) => {
                            eprintln!("Error at line {}. Operand must be a boolean", line);
                            return InterpretResult::RuntimeError;
                        }
                    },
                    _ => {
                        eprintln!("Error at line {}. Operand must be a boolean", line);
                        return InterpretResult::RuntimeError;
                    }
                },
                Equal => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => match (&*a, &*b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Rc::new(Value::Boolean(a == b)))
                        }
                        (Value::Boolean(a), Value::Boolean(b)) => {
                            self.stack.push(Rc::new(Value::Boolean(a == b)))
                        }
                        (Value::Nil, Value::Nil) => self.stack.push(Rc::new(Value::Boolean(true))),
                        (Value::Obj(a), Value::Obj(b)) => {
                            self.stack.push(Rc::new(Value::Boolean(a == b)));
                        }
                        _ => {
                            eprintln!("Error at line {}, Operands must be of the same type", line);
                            return InterpretResult::RuntimeError;
                        }
                    },
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
                Print => match self.stack.pop() {
                    Some(val) => println!("{}", val.print_lox()),
                    None => eprintln!(
                        "Error at line {}, nothing to print, the stack is empty",
                        line
                    ),
                },
                Pop => {
                    self.stack.pop();
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
