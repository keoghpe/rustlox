use crate::{
    expression::Expr,
    token::{Token, TokenType},
};

struct Parser<'a> {
    current: i64,
    tokens: &'a Vec<Token>,
}

impl Parser<'_> {
    fn new(tokens: &Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
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

    fn comparison(&self) -> Expr {
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

    fn term(&self) -> Expr {
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

    fn factor(&self) -> Expr {
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

    fn unary(&self) -> Expr {
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

    fn primary(&self) -> Expr {
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
                value: (&"null").to_string(),
            };
        }
        if self.is_match(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Expr::Literal {
                value: self.previous().literal,
            };
        }
        if self.is_match(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Expr::Grouping {
                expression: Box::new(expr),
            };
        }
        panic!("this shouldn't happen")
    }

    fn is_match(&self, ttypes: Vec<TokenType>) -> bool {
        for ttype in ttypes.iter() {
            if self.check(ttype) {
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

        matches!(self.peek().ttype, ttype)
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().ttype, TokenType::EOF)
    }

    fn peek(&self) -> Token {
        self.tokens[self.current as usize]
    }

    fn previous(&self) -> Token {
        self.tokens[(self.current - 1) as usize]
    }
}
