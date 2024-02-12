use std::{iter::Peekable, str::Chars};

use crate::compile::Source;

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier(String),
    Str(String),
    Number(f64),
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: i32,
}

impl Token {
    pub fn precedence(&self) -> i32 {
        match self.token_type {
            TokenType::Bang => 7,
            TokenType::Star | TokenType::Slash => 6,
            TokenType::Plus | TokenType::Minus => 5,
            TokenType::Greater
            | TokenType::Less
            | TokenType::GreaterEqual
            | TokenType::LessEqual => 4,
            TokenType::EqualEqual | TokenType::BangEqual => 3,
            TokenType::Number(_) => 2, // Assuming you want literals to have a precedence.
            TokenType::Nil | TokenType::True | TokenType::False => 2,
            TokenType::Identifier(_) => 2,
            TokenType::LeftParen | TokenType::RightParen => 1, // Parentheses to control precedence explicitly.
            _ => 0,
        }
    }
}

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    line: i32,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a Source) -> Self {
        Tokenizer {
            chars: input.0.chars().peekable(),
            line: 0,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.chars.next()?;
        let token_type = match next_char {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' => {
                if self.chars.peek() == Some(&'=') {
                    self.chars.next();
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.chars.peek() == Some(&'=') {
                    self.chars.next();
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.chars.peek() == Some(&'=') {
                    self.chars.next();
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.chars.peek() == Some(&'=') {
                    self.chars.next();
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '/' => {
                if let Some('/') = self.chars.peek() {
                    while let Some('\n') = self.chars.peek() {
                        self.chars.next();
                    }
                    self.line += 1;
                    return self.next();
                } else {
                    TokenType::Slash
                }
            }
            ' ' => return self.next(),
            '\r' => return self.next(),
            '\t' => return self.next(),
            '\n' => {
                self.line += 1;
                return self.next();
            }
            '"' => {
                let mut string = String::new();
                while let Some(&ch) = self.chars.peek() {
                    if ch == '"' {
                        self.chars.next();
                        break;
                    } else {
                        string.push(ch);
                        self.chars.next();
                    }
                }
                TokenType::Str(string)
            }
            _ if next_char.is_ascii_digit() => {
                let mut number = String::new();
                number.push(next_char);
                while let Some(&ch) = self.chars.peek() {
                    if ch.is_ascii_digit() {
                        number.push(ch);
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                TokenType::Number(number.parse().unwrap()) // safe to unwrap, since we only accepted digits
            }
            _ if next_char.is_alphabetic() => {
                let mut identifier = String::new();
                identifier.push(next_char);
                while let Some(&ch) = self.chars.peek() {
                    if ch.is_alphanumeric() {
                        identifier.push(ch);
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "and" => TokenType::And,
                    "class" => TokenType::Class,
                    "else" => TokenType::Else,
                    "false" => TokenType::False,
                    "for" => TokenType::For,
                    "fun" => TokenType::Fun,
                    "if" => TokenType::If,
                    "nil" => TokenType::Nil,
                    "or" => TokenType::Or,
                    "print" => TokenType::Print,
                    "return" => TokenType::Return,
                    "super" => TokenType::Super,
                    "this" => TokenType::This,
                    "true" => TokenType::True,
                    "var" => TokenType::Var,
                    "while" => TokenType::While,
                    _ => TokenType::Identifier(identifier),
                }
            }
            _ => TokenType::Error,
        };
        Some(Token {
            token_type,
            line: self.line,
        })
    }
}
