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

#[derive(Debug)]
pub enum OpCode {
    Return,
    Constant(Value),
    Not,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Greater,
    Less,
}

pub struct Chunk<'a> {
    name: &'a str,
    pub code: Vec<(OpCode, i32)>,
}

pub trait Disassembler {
    fn disassemble(&self);
}

impl<'a> Chunk<'a> {
    pub fn write(&mut self, byte: OpCode, line: i32) {
        self.code.push((byte, line));
    }

    pub fn new(name: &'a str) -> Self {
        Chunk { name, code: vec![] }
    }
}

impl Disassembler for Chunk<'_> {
    fn disassemble(&self) {
        println!("=== {} ===", self.name);
    }
}

impl Disassembler for OpCode {
    fn disassemble(&self) {
        match self {
            OpCode::Return => println!("Return"),
            OpCode::Constant(value) => println!("Constant {}", value),
            OpCode::Negate => println!("Negate"),
            OpCode::Add => println!("Add"),
            OpCode::Subtract => println!("Subtract"),
            OpCode::Multiply => println!("Multiply"),
            OpCode::Divide => println!("Divide"),
            OpCode::Not => println!("Not"),
            OpCode::Equal => println!("Equal"),
            OpCode::Greater => println!("Greater"),
            OpCode::Less => println!("Less"),
        }
    }
}
