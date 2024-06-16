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

    fn not_eof(&mut self) -> bool {
        self.tokens.peek().is_some()
    }

    fn declaration(&mut self) -> Option<Expr> {
        self.statement()
    }

    fn print_statement(&mut self) -> Option<Expr> {
        let t = self.consume()?;
        if !matches!(
            t,
            Token {
                token_type: TokenType::Print,
                ..
            }
        ) {
            panic!(
                "Expected print statement to be established by the previous level. This should never happen"
            );
        }
        let mut expr = self.expression(0)?;
        expr.push((OpCode::Print, self.tokens.peek()?.line));
        let tok = self.consume()?;
        match tok.token_type {
            TokenType::Semicolon => Some(expr),
            _ => {
                println!("Expected ;");
                None
            }
        }
    }

    fn statement(&mut self) -> Option<Expr> {
        match self.tokens.peek() {
            Some(Token {
                token_type: TokenType::Print,
                line: _,
            }) => self.print_statement(),
            _ => self.expression_statement(),
        }
    }

    fn expression_statement(&mut self) -> Option<Expr> {
        let mut expr = self.expression(0)?;
        match self.consume() {
            Some(Token {
                token_type: TokenType::Semicolon,
                line,
            }) => {
                expr.push((OpCode::Pop, line));
                Some(expr)
            }
            _ => {
                println!("Expected ;");
                None
            }
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        let mut result = vec![];
        while self.not_eof() {
            let mut expr = self.declaration()?;
            result.append(&mut expr);
        }
        Some(result)
    }

    pub fn expression(&mut self, precedence: i32) -> Option<Expr> {
        let token = self.consume().or_else(|| {
            println!("Unexpected end of input"); // TODO: rewrite with Result to avoid this println-s
            None
        })?;
        let mut left = prefix_parselets(token, self)?;
        while precedence < self.peek_precedence() {
            let token = self.consume().or_else(|| {
                println!("Unexpected end of input");
                None
            })?;
            let mut right = infix_parselets(token, self)?;
            left.append(&mut right);
        }
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
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Not, tok.line));
            Some(expr)
        }
        TokenType::Plus => {
            let expr = parser.expression(tok.precedence())?;
            Some(expr)
        }
        TokenType::Minus => {
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Negate, tok.line));
            Some(expr)
        }
        TokenType::LeftParen => {
            let expr = parser.expression(tok.precedence())?;
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
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Add, tok.line));
            Some(expr)
        }
        TokenType::Minus => {
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Subtract, tok.line));
            Some(expr)
        }
        TokenType::Star => {
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Multiply, tok.line));
            Some(expr)
        }
        TokenType::Slash => {
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Divide, tok.line));
            Some(expr)
        }
        TokenType::Greater => {
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Greater, tok.line));
            Some(expr)
        }
        TokenType::Less => {
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Less, tok.line));
            Some(expr)
        }
        TokenType::EqualEqual => {
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Equal, tok.line));
            Some(expr)
        }
        TokenType::BangEqual => {
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Equal, tok.line));
            expr.push((OpCode::Not, tok.line));
            Some(expr)
        }
        TokenType::GreaterEqual => {
            let mut expr = parser.expression(tok.precedence())?;
            expr.push((OpCode::Less, tok.line));
            expr.push((OpCode::Not, tok.line));
            Some(expr)
        }
        TokenType::LessEqual => {
            let mut expr = parser.expression(tok.precedence())?;
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
mod test_expr {
    use super::*;
    use crate::{compile::Source, tokens::Tokenizer};

    #[test]
    fn test_parse_number() {
        let input = Source("42".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0).unwrap();
        assert_eq!(expr, vec![(OpCode::Constant(Value::Number(42.0)), 0)]);
    }

    #[test]
    fn test_parse_greater() {
        let input = Source(">".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(expr, None);
    }

    #[test]
    fn test_x_gt_y() {
        let input = Source("10 > 5".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(
            expr,
            Some(vec![
                (OpCode::Constant(Value::Number(10.0)), 0),
                (OpCode::Constant(Value::Number(5.0)), 0),
                (OpCode::Greater, 0)
            ])
        );
    }

    #[test]
    fn test_parse_less() {
        let input = Source("<".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(expr, None);
    }

    #[test]
    fn test_x_lt_y() {
        let input = Source("10 < 5".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(
            expr,
            Some(vec![
                (OpCode::Constant(Value::Number(10.0)), 0),
                (OpCode::Constant(Value::Number(5.0)), 0),
                (OpCode::Less, 0)
            ])
        );
    }

    #[test]
    fn test_parse_greater_equal() {
        let input = Source(">=".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(expr, None);
    }

    #[test]
    fn test_x_gte_y() {
        let input = Source("10 >= 5".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(
            expr,
            Some(vec![
                (OpCode::Constant(Value::Number(10.0)), 0),
                (OpCode::Constant(Value::Number(5.0)), 0),
                (OpCode::Less, 0),
                (OpCode::Not, 0)
            ])
        );
    }

    #[test]
    fn test_parse_less_equal() {
        let input = Source("<=".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(expr, None);
    }

    #[test]
    fn test_x_lte_y() {
        let input = Source("10 <= 5".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(
            expr,
            Some(vec![
                (OpCode::Constant(Value::Number(10.0)), 0),
                (OpCode::Constant(Value::Number(5.0)), 0),
                (OpCode::Greater, 0),
                (OpCode::Not, 0)
            ])
        );
    }

    #[test]
    fn test_parse_equal_equal() {
        let input = Source("==".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(expr, None);
    }

    #[test]
    fn test_x_eq_y() {
        let input = Source("10 == 5".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(
            expr,
            Some(vec![
                (OpCode::Constant(Value::Number(10.0)), 0),
                (OpCode::Constant(Value::Number(5.0)), 0),
                (OpCode::Equal, 0)
            ])
        );
    }

    #[test]
    fn test_parse_bang_equal() {
        let input = Source("!=".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(expr, None);
    }

    #[test]
    fn test_x_neq_y() {
        let input = Source("10 != 5".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0);
        assert_eq!(
            expr,
            Some(vec![
                (OpCode::Constant(Value::Number(10.0)), 0),
                (OpCode::Constant(Value::Number(5.0)), 0),
                (OpCode::Equal, 0),
                (OpCode::Not, 0)
            ])
        );
    }

    #[test]
    fn test_parse_string() {
        let input = Source("\"hello world\"".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.expression(0).unwrap();
        assert_eq!(
            expr,
            vec![(
                OpCode::Constant(Value::Obj(common::Obj::String("hello world".to_string()))),
                0
            )]
        );
    }
}

#[cfg(test)]
mod test_parse {
    use super::*;
    use crate::{compile::Source, tokens::Tokenizer};

    #[test]
    fn parse_print_statement() {
        let input = Source("print 42;".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.parse();
        assert_eq!(
            expr,
            Some(vec![
                (OpCode::Constant(Value::Number(42.0)), 0),
                (OpCode::Print, 0),
            ])
        );
    }

    #[test]
    fn test_parse_number() {
        let input = Source("42;".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.parse();
        assert_eq!(
            expr,
            Some(vec![
                (OpCode::Constant(Value::Number(42.0)), 0),
                (OpCode::Pop, 0)
            ])
        );
    }

    #[test]
    fn parse_print_with_paren() {
        let input = Source("print (42 + 5);".into());
        let tokenizer = Tokenizer::new(&input).peekable();
        let mut parser = Parser::new(tokenizer);
        let expr = parser.parse();
        assert_eq!(
            expr,
            Some(vec![
                (OpCode::Constant(Value::Number(42.0)), 0),
                (OpCode::Constant(Value::Number(5.0)), 0),
                (OpCode::Add, 0),
                (OpCode::Print, 0)
            ])
        );
    }
}
