use std::{iter::Peekable, str::Chars};

use crate::compile::Source;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tokenizer_tests {
    use super::*;
    #[test]
    fn test_left_paren() {
        let source = Source("(".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::LeftParen));
    }
    #[test]
    fn test_right_paren() {
        let source = Source(")".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::RightParen));
    }

    #[test]
    fn test_left_brace() {
        let source = Source("{".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::LeftBrace));
    }

    #[test]
    fn test_right_brace() {
        let source = Source("}".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::RightBrace));
    }

    #[test]
    fn test_comma() {
        let source = Source(",".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Comma));
    }

    #[test]
    fn test_dot() {
        let source = Source(".".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Dot));
    }

    #[test]
    fn test_minus() {
        let source = Source("-".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Minus));
    }

    #[test]
    fn test_plus() {
        let source = Source("+".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Plus));
    }

    #[test]
    fn test_semicolon() {
        let source = Source(";".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Semicolon));
    }

    #[test]
    fn test_slash() {
        let source = Source("/".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Slash));
    }

    #[test]
    fn test_star() {
        let source = Source("*".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Star));
    }

    #[test]
    fn test_bang() {
        let source = Source("!".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Bang));
    }

    #[test]
    fn test_bang_equal() {
        let source = Source("!=".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::BangEqual));
    }

    #[test]
    fn test_equal() {
        let source = Source("=".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Equal));
    }

    #[test]
    fn test_equal_equal() {
        let source = Source("==".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::EqualEqual));
    }

    #[test]
    fn test_greater() {
        let source = Source(">".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Greater));
    }

    #[test]
    fn test_greater_equal() {
        let source = Source(">=".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::GreaterEqual));
    }

    #[test]
    fn test_less() {
        let source = Source("<".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Less));
    }

    #[test]
    fn test_less_equal() {
        let source = Source("<=".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::LessEqual));
    }

    #[test]
    fn test_identifier() {
        let source = Source("identifier".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Identifier(_)));
    }

    #[test]
    fn test_string() {
        let source = Source("\"string\"".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Str(_)));
    }

    #[test]
    fn test_number() {
        let source = Source("123".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Number(_)));
    }

    #[test]
    fn test_and() {
        let source = Source("and".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::And));
    }
    #[test]
    fn test_class() {
        let source = Source("class".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Class));
    }

    #[test]
    fn test_else() {
        let source = Source("else".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Else));
    }

    #[test]
    fn test_false() {
        let source = Source("false".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::False));
    }

    #[test]
    fn test_for() {
        let source = Source("for".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::For));
    }

    #[test]
    fn test_fun() {
        let source = Source("fun".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Fun));
    }

    #[test]
    fn test_if() {
        let source = Source("if".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::If));
    }

    #[test]
    fn test_nil() {
        let source = Source("nil".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Nil));
    }

    #[test]
    fn test_or() {
        let source = Source("or".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Or));
    }

    #[test]
    fn test_print() {
        let source = Source("print".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Print));
    }

    #[test]
    fn test_return() {
        let source = Source("return".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Return));
    }

    #[test]
    fn test_super() {
        let source = Source("super".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Super));
    }

    #[test]
    fn test_this() {
        let source = Source("this".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::This));
    }

    #[test]
    fn test_true() {
        let source = Source("true".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::True));
    }

    #[test]
    fn test_var() {
        let source = Source("var".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Var));
    }

    #[test]
    fn test_while() {
        let source = Source("while".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::While));
    }

    #[test]
    fn test_error() {
        let source = Source("£".into());
        let token = Tokenizer::new(&source).next().unwrap();
        assert!(matches!(token.token_type, TokenType::Error));
    }

    #[test]
    fn test_multiple_tokens() {
        let source = Source("while (true) { print \"Hello, World!\"; }".into());
        let mut tokenizer = Tokenizer::new(&source);
        let tokens = vec![
            TokenType::While,
            TokenType::LeftParen,
            TokenType::True,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::Print,
            TokenType::Str("Hello, World!".into()),
            TokenType::Semicolon,
            TokenType::RightBrace,
        ];
        for token in tokens {
            assert_eq!(tokenizer.next().unwrap().token_type, token);
        }
    }

    #[test]
    fn test_rparen_after_identifier() {
        let source = Source("identifier)".into());
        let mut tokenizer = Tokenizer::new(&source);
        let tokens = vec![
            TokenType::Identifier("identifier".into()),
            TokenType::RightParen,
        ];
        for token in tokens {
            assert_eq!(tokenizer.next().unwrap().token_type, token);
        }
    }
}
#[cfg(test)]
mod token_precedence_tests {
    use super::*;

    #[test]
    fn bang_has_higher_precedence_than_star() {
        let bang_token = Token {
            token_type: TokenType::Bang,
            line: 0,
        };
        let star_token = Token {
            token_type: TokenType::Star,
            line: 0,
        };
        assert!(bang_token.precedence() > star_token.precedence());
    }

    #[test]
    fn star_has_higher_precedence_than_plus() {
        let star_token = Token {
            token_type: TokenType::Star,
            line: 0,
        };
        let plus_token = Token {
            token_type: TokenType::Plus,
            line: 0,
        };
        assert!(star_token.precedence() > plus_token.precedence());
    }

    #[test]
    fn plus_has_higher_precedence_than_greater() {
        let plus_token = Token {
            token_type: TokenType::Plus,
            line: 0,
        };
        let greater_token = Token {
            token_type: TokenType::Greater,
            line: 0,
        };
        assert!(plus_token.precedence() > greater_token.precedence());
    }

    #[test]
    fn greater_has_higher_precedence_than_equal_equal() {
        let greater_token = Token {
            token_type: TokenType::Greater,
            line: 0,
        };
        let equal_equal_token = Token {
            token_type: TokenType::EqualEqual,
            line: 0,
        };
        assert!(greater_token.precedence() > equal_equal_token.precedence());
    }

    #[test]
    fn equal_equal_has_higher_precedence_than_number() {
        let equal_equal_token = Token {
            token_type: TokenType::EqualEqual,
            line: 0,
        };
        let number_token = Token {
            token_type: TokenType::Number(0.0),
            line: 0,
        };
        assert!(equal_equal_token.precedence() > number_token.precedence());
    }

    #[test]
    fn number_has_same_precedence_as_identifier() {
        let number_token = Token {
            token_type: TokenType::Number(0.0),
            line: 0,
        };
        let identifier_token = Token {
            token_type: TokenType::Identifier("a".to_string()),
            line: 0,
        };
        assert_eq!(number_token.precedence(), identifier_token.precedence());
    }

    #[test]
    fn identifier_has_higher_precedence_than_left_paren() {
        let identifier_token = Token {
            token_type: TokenType::Identifier("a".to_string()),
            line: 0,
        };
        let left_paren_token = Token {
            token_type: TokenType::LeftParen,
            line: 0,
        };
        assert!(identifier_token.precedence() > left_paren_token.precedence());
    }

    #[test]
    fn left_paren_has_higher_precedence_than_error() {
        let left_paren_token = Token {
            token_type: TokenType::LeftParen,
            line: 0,
        };
        let error_token = Token {
            token_type: TokenType::Error,
            line: 0,
        };
        assert!(left_paren_token.precedence() > error_token.precedence());
    }
}
