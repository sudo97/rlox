use crate::{
    common::{self, OpCode, Value},
    tokens::{Token, TokenType, Tokenizer},
};
use std::iter::Peekable;

pub struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Peekable<Tokenizer<'a>>) -> Self {
        Parser { tokens }
    }

    fn consume(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek_precedence(&mut self) -> i32 {
        self.tokens.peek().map_or(0, |t| t.precedence())
    }

    pub fn parse(&mut self, precedence: i32) -> Option<Expr> {
        let token = self.consume().or_else(|| {
            println!("Unexpected end of input"); // TODO: rewrite with Result to avoid this println-s
            None
        })?;
        // println!("{}, Parsing {:?}", precedence, token);
        let mut left = prefix_parselets(token, self)?;
        // println!("{}, Parsed left {:?}", precedence, left);
        while precedence < self.peek_precedence() {
            let token = self.consume().or_else(|| {
                println!("Unexpected end of input");
                None
            })?;
            let mut right = infix_parselets(token, self)?;
            // println!("{}, Parsed right {:?}", precedence, right);
            left.append(&mut right);
            // println!(
            //     "{}, Parsed left after appending right {:?}",
            //     precedence, left
            // );
        }
        // println!("{}, Returning {:?}", precedence, left);
        Some(left)
    }
}

fn prefix_parselets(tok: Token, parser: &mut Parser) -> Option<Expr> {
    match tok.token_type {
        TokenType::Number(n) => {
            let expr = vec![(OpCode::Constant(Value::Number(n)), tok.line)];
            Some(expr)
        }
        TokenType::Str(s) => {
            let expr = vec![(
                OpCode::Constant(Value::Obj(common::Obj::String(s))),
                tok.line,
            )];
            Some(expr)
        }
        TokenType::True => {
            let expr = vec![(OpCode::Constant(Value::Boolean(true)), tok.line)];
            Some(expr)
        }
        TokenType::False => {
            let expr = vec![(OpCode::Constant(Value::Boolean(false)), tok.line)];
            Some(expr)
        }
        TokenType::Nil => {
            let expr = vec![(OpCode::Constant(Value::Nil), tok.line)];
            Some(expr)
        }
        TokenType::Bang => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Not, tok.line));
            Some(expr)
        }
        TokenType::Plus => {
            let expr = parser.parse(tok.precedence())?;
            Some(expr)
        }
        TokenType::Minus => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Negate, tok.line));
            Some(expr)
        }
        TokenType::LeftParen => {
            let expr = parser.parse(tok.precedence())?;
            match parser.consume()?.token_type {
                TokenType::RightParen => Some(expr),
                _ => None,
            }
        }
        _ => {
            println!("Unexpected token: {:?}", tok);
            None
        }
    }
}

fn infix_parselets(tok: Token, parser: &mut Parser) -> Option<Expr> {
    match tok.token_type {
        TokenType::Plus => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Add, tok.line));
            Some(expr)
        }
        TokenType::Minus => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Subtract, tok.line));
            Some(expr)
        }
        TokenType::Star => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Multiply, tok.line));
            Some(expr)
        }
        TokenType::Slash => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Divide, tok.line));
            Some(expr)
        }
        TokenType::Greater => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Greater, tok.line));
            Some(expr)
        }
        TokenType::Less => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Less, tok.line));
            Some(expr)
        }
        TokenType::EqualEqual => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Equal, tok.line));
            Some(expr)
        }
        TokenType::BangEqual => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Equal, tok.line));
            expr.push((OpCode::Not, tok.line));
            Some(expr)
        }
        TokenType::GreaterEqual => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Less, tok.line));
            expr.push((OpCode::Not, tok.line));
            Some(expr)
        }
        TokenType::LessEqual => {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Greater, tok.line));
            expr.push((OpCode::Not, tok.line));
            Some(expr)
        }
        _ => {
            println!("Unexpected token: {:?}", tok);
            None
        }
    }
}

type Expr = Vec<(OpCode, i32)>;

#[cfg(test)]
mod tests {}
