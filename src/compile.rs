use crate::{
    common::{Chunk, OpCode},
    parse::Parser,
    tokens::Tokenizer,
    vm::InterpretMode,
};

pub struct Source(pub String);

impl Source {
    pub fn compile(self, file_name: &str, mode: InterpretMode) -> Option<Chunk> {
        let mut chunk = Chunk::new(file_name);
        let tokenizer = Tokenizer::new(&self);
        if let InterpretMode::Debug = mode {
            for token in Tokenizer::new(&self) {
                println!("{:?}", token);
            }
        }
        let bytecode = Parser::new(tokenizer.peekable()).parse()?;
        chunk.code = bytecode;
        chunk.write(OpCode::Return, 0);
        Some(chunk)
    }
}
