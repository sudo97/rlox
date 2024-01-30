use crate::{
    common::{OpCode, Value},
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
        let prefix_parselet = prefix_parselets(token);
        let mut left = prefix_parselet(self)?;
        while precedence < self.peek_precedence() {
            let token = self.consume().or_else(|| {
                println!("Unexpected end of input");
                None
            })?;
            let infix_parselet = infix_parselets(token);
            let mut right = infix_parselet(self)?;
            left.append(&mut right);
        }
        Some(left)
    }
}

fn prefix_parselets(tok: Token) -> Parselet {
    match tok.token_type {
        TokenType::Number(n) => Box::new(move |_| {
            let expr = vec![(OpCode::Constant(Value::Number(n)), tok.line)];
            Some(expr)
        }),
        TokenType::True => Box::new(move |_| {
            let expr = vec![(OpCode::Constant(Value::Boolean(true)), tok.line)];
            Some(expr)
        }),
        TokenType::False => Box::new(move |_| {
            let expr = vec![(OpCode::Constant(Value::Boolean(false)), tok.line)];
            Some(expr)
        }),
        TokenType::Nil => Box::new(move |_| {
            let expr = vec![(OpCode::Constant(Value::Nil), tok.line)];
            Some(expr)
        }),
        TokenType::Bang => Box::new(move |parser| {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Not, tok.line));
            Some(expr)
        }),
        TokenType::Plus => Box::new(move |parser| {
            let expr = parser.parse(tok.precedence())?;
            Some(expr)
        }),
        TokenType::Minus => Box::new(move |parser| {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Negate, tok.line));
            Some(expr)
        }),
        TokenType::LeftParen => Box::new(move |parser| {
            let expr = parser.parse(tok.precedence())?;
            match parser.consume()?.token_type {
                TokenType::RightParen => Some(expr),
                _ => None,
            }
        }),
        _ => Box::new(move |_| {
            println!("Unexpected token: {:?}", tok);
            None
        }),
    }
}

fn infix_parselets(tok: Token) -> Parselet {
    match tok.token_type {
        TokenType::Plus => Box::new(move |parser| {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Add, tok.line));
            Some(expr)
        }),
        TokenType::Minus => Box::new(move |parser| {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Subtract, tok.line));
            Some(expr)
        }),
        TokenType::Star => Box::new(move |parser| {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Multiply, tok.line));
            Some(expr)
        }),
        TokenType::Slash => Box::new(move |parser| {
            let mut expr = parser.parse(tok.precedence())?;
            expr.push((OpCode::Divide, tok.line));
            Some(expr)
        }),
        _ => Box::new(move |_| {
            println!("Unexpected token: {:?}", tok);
            None
        }),
    }
}

type Expr = Vec<(OpCode, i32)>;

type Parselet = Box<dyn Fn(&mut Parser) -> Option<Expr>>;
