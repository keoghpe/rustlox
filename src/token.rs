use core::fmt;

#[derive(Copy, Clone)]
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

#[derive(Clone)]
pub struct Token {
    pub(crate) ttype: TokenType,
    pub(crate) lexeme: String,
    // This should reference an object / be a generic
    pub(crate) literal: String,
    pub(crate) line: i64,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &(self.ttype.to_string()
                + " "
                + &self.lexeme
                + " "
                + &self.literal
                + " "
                + &self.line.to_string()),
        )
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
            literal: "".to_string(),
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
                } else {
                    // error
                }
            }
        }
    }

    fn string(&mut self) {
        loop {
            if (self.peek() == '"' || self.is_at_end()) {
                break;
            } else {
                if (self.peek() == '\n') {
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

        self.add_token(TokenType::STRING, &value[1..value.len() - 1])
    }

    fn number(&mut self) {
        loop {
            if Self::is_digit(self.peek()) {
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == '.' && Self::is_digit(self.peekNext()) {
            self.advance();

            loop {
                if Self::is_digit(self.peek()) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // TODO Convert literal to a Double
        self.add_token(TokenType::NUMBER, &self.current_string())
    }

    fn add_token_no_literal(&mut self, ttype: TokenType) {
        self.add_token(ttype, "")
    }

    fn add_token(&mut self, ttype: TokenType, literal: &str) {
        self.tokens.push(Token {
            ttype: ttype,
            literal: literal.to_string(),
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

    fn peekNext(&self) -> char {
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

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
}
