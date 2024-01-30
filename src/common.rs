use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::Nil => write!(f, "Nil"),
        }
    }
}

pub enum OpCode {
    Return,
    Constant(Value),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Chunk {
    name: String,
    pub code: Vec<(OpCode, i32)>,
}

pub trait Disassembler {
    fn disassemble(&self);
}

impl Chunk {
    pub fn write(&mut self, byte: OpCode, line: i32) {
        self.code.push((byte, line));
    }

    pub fn new(name: String) -> Self {
        Chunk { name, code: vec![] }
    }
}

impl Disassembler for Chunk {
    fn disassemble(&self) {
        println!("=== {} ===", self.name);
    }
}

impl Disassembler for OpCode {
    fn disassemble(&self) {
        match self {
            OpCode::Return => println!("OpReturn"),
            OpCode::Constant(value) => println!("OpConstant {}", value),
            OpCode::Negate => println!("OpNegate"),
            OpCode::Add => println!("OpAdd"),
            OpCode::Subtract => println!("OpSubtract"),
            OpCode::Multiply => println!("OpMultiply"),
            OpCode::Divide => println!("OpDivide"),
        }
    }
}
