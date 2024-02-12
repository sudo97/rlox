use crate::{
    common::{Chunk, OpCode},
    parse::Parser,
    tokens::Tokenizer,
};

pub struct Source(pub String);

impl Source {
    pub fn compile(self, file_name: &str) -> Option<Chunk> {
        let mut chunk = Chunk::new(file_name);
        let tokenizer = Tokenizer::new(&self);
        for token in Tokenizer::new(&self) {
            println!("{:?}", token);
        }
        let bytecode = Parser::new(tokenizer.peekable()).parse(0)?;
        for (instruction, line) in bytecode {
            chunk.write(instruction, line);
        }
        chunk.write(OpCode::Return, 0);
        Some(chunk)
    }
}
