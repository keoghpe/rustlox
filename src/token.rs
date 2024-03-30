use core::fmt;
use lazy_static::lazy_static;
use std::{borrow::Borrow, collections::HashMap, env, fmt::Debug};

use crate::{
    environment::{self, Environment},
    expression::Stmt,
    interpreter::Interpreter,
};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("and", TokenType::AND),
        ("class", TokenType::CLASS),
        ("else", TokenType::ELSE),
        ("false", TokenType::FALSE),
        ("for", TokenType::FOR),
        ("fun", TokenType::FUN),
        ("if", TokenType::IF),
        ("nil", TokenType::NIL),
        ("or", TokenType::OR),
        ("print", TokenType::PRINT),
        ("return", TokenType::RETURN),
        ("super", TokenType::SUPER),
        ("this", TokenType::THIS),
        ("true", TokenType::TRUE),
        ("var", TokenType::VAR),
        ("while", TokenType::WHILE),
    ]);
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Single-character tokens.
            TokenType::LEFT_PAREN => write!(f, "LEFT_PAREN"),
            TokenType::RIGHT_PAREN => write!(f, "RIGHT_PAREN"),
            TokenType::LEFT_BRACE => write!(f, "LEFT_BRACE"),
            TokenType::RIGHT_BRACE => write!(f, "RIGHT_BRACE"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::DOT => write!(f, "DOT"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::SEMICOLON => write!(f, "SEMICOLON"),
            TokenType::SLASH => write!(f, "SLASH"),
            TokenType::STAR => write!(f, "STAR"),
            // One or two character tokens.
            TokenType::BANG => write!(f, "BANG"),
            TokenType::BANG_EQUAL => write!(f, "BANG_EQUAL"),
            TokenType::EQUAL => write!(f, "EQUAL"),
            TokenType::EQUAL_EQUAL => write!(f, "EQUAL_EQUAL"),
            TokenType::GREATER => write!(f, "GREATER"),
            TokenType::GREATER_EQUAL => write!(f, "GREATER_EQUAL"),
            TokenType::LESS => write!(f, "LESS"),
            TokenType::LESS_EQUAL => write!(f, "LESS_EQUAL"),
            // Literals.
            TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenType::STRING => write!(f, "STRING"),
            TokenType::NUMBER => write!(f, "NUMBER"),
            // Keywords.
            TokenType::AND => write!(f, "AND"),
            TokenType::CLASS => write!(f, "CLASS"),
            TokenType::ELSE => write!(f, "ELSE"),
            TokenType::FALSE => write!(f, "FALSE"),
            TokenType::FUN => write!(f, "FUN"),
            TokenType::FOR => write!(f, "FOR"),
            TokenType::IF => write!(f, "IF"),
            TokenType::NIL => write!(f, "NIL"),
            TokenType::OR => write!(f, "OR"),
            TokenType::PRINT => write!(f, "PRINT"),
            TokenType::RETURN => write!(f, "RETURN"),
            TokenType::SUPER => write!(f, "SUPER"),
            TokenType::THIS => write!(f, "THIS"),
            TokenType::TRUE => write!(f, "TRUE"),
            TokenType::VAR => write!(f, "VAR"),
            TokenType::WHILE => write!(f, "WHILE"),
            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Callable {
    NativeFunction {
        arity: i8,
        call: fn(&Interpreter, &Vec<Value>) -> Value,
        value: String,
    },
    Function {
        declaration: Box<Stmt>,
    },
}

impl Callable {
    pub fn arity(&self) -> i8 {
        match self {
            Callable::NativeFunction {
                arity,
                call: _,
                value: _,
            } => arity.clone(),
            Callable::Function { declaration } => {
                if let Stmt::Function {
                    name: _,
                    params,
                    body: _,
                } = declaration.as_ref()
                {
                    params.len() as i8
                } else {
                    panic!("No params")
                }
            }
        }
    }

    pub fn call(&self, interpreter: &Interpreter, values: &Vec<Value>) -> Value {
        match self {
            Callable::NativeFunction {
                arity: _,
                call,
                value: _,
            } => call(interpreter, values),
            Callable::Function { declaration } => {
                if let Stmt::Function {
                    name: _,
                    params,
                    body,
                } = declaration.as_ref()
                {
                    // TODO Environment should have globals as it's enclosing.
                    let environment = Environment::new(None);

                    for (i, param) in params.iter().enumerate() {
                        environment.define(param, &values[i]);
                    }

                    interpreter.execute_block(body, environment);
                    // TODO Why do we return nil here?
                    Value::Nil
                } else {
                    panic!("Nope!")
                }
            }
        }
    }

    pub fn value(&self) -> String {
        match self {
            Callable::NativeFunction {
                arity: _,
                call: _,
                value,
            } => value.clone(),
            Callable::Function { declaration } => {
                if let Stmt::Function {
                    name,
                    params: _,
                    body: _,
                } = declaration.as_ref()
                {
                    name.to_string()
                } else {
                    panic!("Nope!")
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Boolean { value: bool },
    Double { value: f64 },
    String { value: String },
    Nil,
    Callable { callable: Callable },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Boolean { value } => f.write_str(&value.to_string()),
            Value::Double { value } => f.write_str(&value.to_string()),
            Value::String { value } => f.write_str(&value.to_string()),
            Value::Nil => f.write_str(&"Nil".to_string()),
            Value::Callable { callable } => f.write_str(&callable.value()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub(crate) ttype: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Value>,
    pub(crate) line: i64,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_string = self.ttype.to_string() + " " + &self.lexeme;

        match &self.literal {
            Some(literal_value) => debug_string = debug_string + " " + &literal_value.to_string(),
            None => (),
        }

        f.write_str(&(debug_string + " " + &self.line.to_string()))
    }
}

pub struct Scanner<'a> {
    source: &'a str,
    start: i64,
    current: i64,
    line: i64,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        loop {
            if self.is_at_end() {
                break;
            }

            self.start = self.current;
            // scan token
            self.scan_token();
        }

        self.tokens.push(Token {
            ttype: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token_no_literal(TokenType::LEFT_PAREN),
            ')' => self.add_token_no_literal(TokenType::RIGHT_PAREN),
            '{' => self.add_token_no_literal(TokenType::LEFT_BRACE),
            '}' => self.add_token_no_literal(TokenType::RIGHT_BRACE),
            ',' => self.add_token_no_literal(TokenType::COMMA),
            '.' => self.add_token_no_literal(TokenType::DOT),
            '-' => self.add_token_no_literal(TokenType::MINUS),
            '+' => self.add_token_no_literal(TokenType::PLUS),
            ';' => self.add_token_no_literal(TokenType::SEMICOLON),
            '*' => self.add_token_no_literal(TokenType::STAR),
            '!' => {
                if self.is_match('=') {
                    self.add_token_no_literal(TokenType::BANG_EQUAL);
                } else {
                    self.add_token_no_literal(TokenType::BANG);
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token_no_literal(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token_no_literal(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token_no_literal(TokenType::LESS_EQUAL);
                } else {
                    self.add_token_no_literal(TokenType::LESS);
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token_no_literal(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token_no_literal(TokenType::GREATER);
                }
            }
            '/' => {
                if self.is_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        } else {
                            self.advance();
                        }
                    }
                } else {
                    self.add_token_no_literal(TokenType::SLASH)
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),

            _ => {
                if Self::is_digit(c) {
                    self.number()
                } else if Self::is_alpha(c) {
                    self.identifier()
                } else {
                    // error
                }
            }
        }
    }

    fn string(&mut self) {
        loop {
            if self.peek() == '"' || self.is_at_end() {
                break;
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                }
                self.advance();
            }
        }

        if self.is_at_end() {
            // TODO ERROR
            return;
        }

        // Closing "
        self.advance();

        let value = self.current_string();

        self.add_token(
            TokenType::STRING,
            Some(Value::String {
                value: value[1..value.len() - 1].to_string(),
            }),
        )
    }

    fn number(&mut self) {
        loop {
            if Self::is_digit(self.peek()) {
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            loop {
                if Self::is_digit(self.peek()) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        self.add_token(
            TokenType::NUMBER,
            Some(Value::Double {
                value: self.current_string().parse().unwrap(),
            }),
        )
    }

    fn identifier(&mut self) {
        loop {
            if Self::is_alpha_numeric(self.peek()) {
                self.advance();
            } else {
                break;
            }
        }

        let text = self.current_string();
        let identifier = KEYWORDS.get(&text.borrow());

        match identifier {
            Some(keyword) => self.add_token_no_literal(*keyword),
            None => self.add_token_no_literal(TokenType::IDENTIFIER),
        }
    }

    fn add_token_no_literal(&mut self, ttype: TokenType) {
        self.add_token(ttype, None)
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Value>) {
        self.tokens.push(Token {
            ttype: ttype,
            literal: literal,
            lexeme: self.current_string(),
            line: self.line,
        });
    }

    fn advance(&mut self) -> char {
        let current_char = self.current_char();
        self.current += 1;
        current_char
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if expected != self.current_char() {
            return false;
        }

        self.current += 1;

        true
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as i64
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.current_char()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() as i64 {
            '\0'
        } else {
            self.source
                .chars()
                .nth((self.current + 1) as usize)
                .unwrap()
        }
    }

    fn current_string(&self) -> String {
        self.source[self.start as usize..self.current as usize].to_string()
    }

    fn current_char(&self) -> char {
        self.source.chars().nth((self.current) as usize).unwrap()
    }

    fn is_alpha_numeric(c: char) -> bool {
        Self::is_digit(c) || Self::is_alpha(c)
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }
}
