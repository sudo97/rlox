use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
    Obj(Obj),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({})", n),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::Nil => write!(f, "Nil"),
            Value::Obj(obj) => write!(f, "Obj({:?})", obj),
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Obj {
    String(String),
    // more to come
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
