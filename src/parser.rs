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

impl Parser<'_> {
    pub fn new<'a>(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Expr {
        // try expression
        self.expression()
        // if there is an error, catch it and return null
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_match(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.is_match(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.is_match(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.is_match(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.is_match(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.is_match(vec![TokenType::FALSE]) {
            return Expr::Literal {
                value: (&"false").to_string(),
            };
        }
        if self.is_match(vec![TokenType::TRUE]) {
            return Expr::Literal {
                value: (&"true").to_string(),
            };
        }
        if self.is_match(vec![TokenType::NIL]) {
            return Expr::Literal {
                value: (&"nil").to_string(),
            };
        }
        if self.is_match(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Expr::Literal {
                value: self.previous().literal,
            };
        }
        if self.is_match(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(
                TokenType::RIGHT_PAREN,
                "Expect ')' after expression.".to_string(),
            );
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }
        // TODO ERROR HERE expression expected
        panic!("this shouldn't happen {:?}", self)
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
        matches!(self.peek().ttype, TokenType::EOF)
    }

    fn peek(&self) -> Token {
        self.tokens[self.current as usize].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[(self.current - 1) as usize].clone()
    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Token {
        if self.check(ttype) {
            return self.advance();
        } else {
            panic!()
            // TODO Error handling page 88
        }

        // throw an error
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
}
