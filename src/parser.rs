use std::fmt::Display;

use crate::{
    expression::Expr,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub(crate) struct Parser<'a> {
    current: i64,
    tokens: &'a Vec<Token>,
}

struct ParseError {
    message: String,
    line: i64,
    error_where: String,
}

impl ParseError {
    fn report(&self) {
        println!(
            "[line {}] Error{}: {}",
            self.line, self.error_where, self.message
        );
    }
}

impl Parser<'_> {
    pub fn new<'a>(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Expr {
        let expr_result = self.expression();

        match expr_result {
            Ok(expr) => {
                return expr;
            }
            Err(err) => {
                err.report();
                return Expr::Literal {
                    value: "null".to_string(),
                };
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let expr_result = self.comparison();

        match expr_result {
            Ok(mut expr) => {
                while self.is_match(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
                    let operator = self.previous();
                    let right_result = self.comparison();

                    match right_result {
                        Ok(right) => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operator,
                                right: Box::new(right),
                            };
                        }
                        Err(parseError) => return Err(parseError),
                    }
                }
                Ok(expr)
            }

            Err(parseError) => return Err(parseError),
        }
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let expr_result = self.term();

        match expr_result {
            Ok(mut expr) => {
                while self.is_match(vec![
                    TokenType::GREATER,
                    TokenType::GREATER_EQUAL,
                    TokenType::LESS,
                    TokenType::LESS_EQUAL,
                ]) {
                    let operator = self.previous();
                    let right_result = self.term();

                    match right_result {
                        Ok(right) => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operator,
                                right: Box::new(right),
                            };
                        }
                        Err(parseError) => return Err(parseError),
                    }
                }

                Ok(expr)
            }
            Err(parseError) => return Err(parseError),
        }
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let left_result = self.factor();

        match left_result {
            Ok(mut expr) => {
                while self.is_match(vec![TokenType::MINUS, TokenType::PLUS]) {
                    let operator = self.previous();
                    let right_result = self.factor();

                    match right_result {
                        Ok(right) => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operator,
                                right: Box::new(right),
                            };
                        }
                        Err(_) => {}
                    }
                }

                Ok(expr)
            }
            Err(parseError) => Err(parseError),
        }
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let left_result = self.unary();

        match left_result {
            Ok(mut expr) => {
                while self.is_match(vec![TokenType::SLASH, TokenType::STAR]) {
                    let operator = self.previous();
                    let right_result = self.unary();
                    match right_result {
                        Ok(right) => {
                            expr = Expr::Binary {
                                left: Box::new(expr),
                                operator,
                                right: Box::new(right),
                            };
                        }
                        Err(parseError) => return Err(parseError),
                    }
                }

                Ok(expr)
            }
            Err(parseError) => return Err(parseError),
        }
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.is_match(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right_result = self.unary();

            match right_result {
                Ok(right) => {
                    return Ok(Expr::Unary {
                        operator,
                        right: Box::new(right),
                    });
                }
                Err(err) => return Err(err),
            }
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.is_match(vec![TokenType::FALSE]) {
            return Ok(Expr::Literal {
                value: (&"false").to_string(),
            });
        }
        if self.is_match(vec![TokenType::TRUE]) {
            return Ok(Expr::Literal {
                value: (&"true").to_string(),
            });
        }
        if self.is_match(vec![TokenType::NIL]) {
            return Ok(Expr::Literal {
                value: (&"nil").to_string(),
            });
        }
        if self.is_match(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal {
                value: self.previous().literal,
            });
        }
        if self.is_match(vec![TokenType::LEFT_PAREN]) {
            let expr_result = self.expression();

            let result = self.consume(
                TokenType::RIGHT_PAREN,
                "Expect ')' after expression.".to_string(),
            );

            match result {
                Ok(_) => (),
                Err(parseError) => return Err(parseError),
            }

            match expr_result {
                Ok(expr) => {
                    return Ok(Expr::Grouping {
                        expression: Box::new(expr),
                    });
                }
                Err(parseError) => return Err(parseError),
            }
        }
        Err(self.current_error("this shouldn't happen".to_string()))
    }

    fn is_match(&mut self, ttypes: Vec<TokenType>) -> bool {
        for ttype in ttypes.iter() {
            if self.check(ttype.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ttype == ttype
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current as usize].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[(self.current - 1) as usize].clone()
    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, ParseError> {
        if self.check(ttype) {
            return Ok(self.advance());
        } else {
            let error = self.current_error(message);
            Err(error)
        }
    }

    fn current_error(&mut self, message: String) -> ParseError {
        self.error(self.peek(), message)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn syncronize(&self) -> Token {
        todo!()
    }

    fn error(&self, token: Token, message: String) -> ParseError {
        if token.ttype == TokenType::EOF {
            ParseError {
                message,
                line: token.line,
                error_where: " at end ".to_owned(),
            }
        } else {
            ParseError {
                message,
                line: token.line,
                error_where: (" at '".to_string() + &token.lexeme.to_string() + "'"),
            }
        }
    }
}
