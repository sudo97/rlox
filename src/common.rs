pub enum OpCode {
    OpReturn,
    OpConstant(f64),
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
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
            OpCode::OpReturn => println!("OpReturn"),
            OpCode::OpConstant(value) => println!("OpConstant {}", value),
            OpCode::OpNegate => println!("OpNegate"),
            OpCode::OpAdd => println!("OpAdd"),
            OpCode::OpSubtract => println!("OpSubtract"),
            OpCode::OpMultiply => println!("OpMultiply"),
            OpCode::OpDivide => println!("OpDivide"),
        }
    }
}
