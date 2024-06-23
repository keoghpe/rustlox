use crate::{
    expression::{Expr, Stmt},
    token::{Token, TokenType, Value},
};

#[derive(Debug)]
pub(crate) struct Parser<'a> {
    current: i64,
    tokens: &'a Vec<Token>,
}

#[derive(Debug)]
struct ParseError {
    message: String,
    #[allow(dead_code)]
    line: i64,
    #[allow(dead_code)]
    error_where: String,
}

impl ParseError {
    // fn report(&self) {
    //     println!(
    //         "[line {}] Error{}: {}",
    //         self.line, self.error_where, self.message
    //     );
    // }
}

impl Parser<'_> {
    pub fn new<'a>(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.is_at_end() {
            let declaration = self.declaration();
            // println!("{:?}", declaration);
            statements.push(declaration);
        }

        statements
    }

    fn declaration(&mut self) -> Stmt {
        if self.is_match(vec![TokenType::FUN]) {
            self.function("function".to_string())
        } else if self.is_match(vec![TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        }
        // Call syncronize to recover from errors
    }

    fn function(&mut self, _kind: String) -> Stmt {
        // TODO String interpolation
        let name;
        match self.consume(TokenType::IDENTIFIER, "Expect + kind + name.".to_owned()) {
            Ok(name_token) => name = name_token,
            Err(err) => {
                panic!("Panicked parsing function statement {}", err.message)
            }
        }

        // TODO String interpolation
        match self.consume(
            TokenType::LEFT_PAREN,
            "Expect '(' after + kind + name.".to_owned(),
        ) {
            Ok(_) => (),
            Err(err) => {
                panic!("Panicked parsing function statement {}", err.message)
            }
        }

        let mut parameters = vec![];

        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                if parameters.len() >= 255 {
                    self.error(
                        self.peek(),
                        "Can't have more than 255 parameters.".to_string(),
                    );
                }

                match self.consume(TokenType::IDENTIFIER, "Expect parameter name.".to_string()) {
                    Ok(parameter) => parameters.push(parameter),
                    Err(err) => panic!("Error parsing params {:?}", err),
                }

                if !self.is_match(vec![TokenType::COMMA]) {
                    break;
                }
            }
        }

        // TODO String interpolation
        match self.consume(
            TokenType::RIGHT_PAREN,
            "Expect ')' after parameters.".to_owned(),
        ) {
            Ok(_) => (),
            Err(err) => {
                panic!("Panicked parsing function statement {}", err.message)
            }
        }

        // TODO String interpolation
        match self.consume(
            TokenType::LEFT_BRACE,
            "Expect '{' before + kind + body.".to_owned(),
        ) {
            Ok(_) => (),
            Err(err) => {
                panic!("Panicked parsing function statement {}", err.message)
            }
        }

        let body = self.block();

        Stmt::Function {
            name,
            params: parameters,
            body,
        }
    }

    fn var_declaration(&mut self) -> Stmt {
        let consume_result = self.consume(TokenType::IDENTIFIER, "Expect variable name".to_owned());

        match consume_result {
            Ok(token) => {
                if self.is_match(vec![TokenType::EQUAL]) {
                    let initializer_result = self.expression();

                    match initializer_result {
                        Ok(initializer) => {
                            let semicolon_result = self.consume(
                                TokenType::SEMICOLON,
                                "Expect ';' after value.".to_owned(),
                            );

                            match semicolon_result {
                                Ok(_) => (),
                                Err(err) => {
                                    panic!("Panicked parsing expression statement {}", err.message)
                                }
                            }

                            Stmt::Var {
                                name: token,
                                initializer: Some(initializer),
                            }
                        }
                        Err(_) => panic!("FUCCBARR"),
                    }
                } else {
                    match self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_owned()) {
                        Ok(_) => (),
                        Err(err) => {
                            panic!("Panicked parsing expression statement {}", err.message)
                        }
                    }

                    Stmt::Var {
                        name: token,
                        initializer: None,
                    }
                }
            }
            Err(_) => panic!("Oooooooops"),
        }
    }

    fn statement(&mut self) -> Stmt {
        if self.is_match(vec![TokenType::FOR]) {
            self.for_statement()
        } else if self.is_match(vec![TokenType::IF]) {
            self.if_statement()
        } else if self.is_match(vec![TokenType::WHILE]) {
            self.while_statement()
        } else if self.is_match(vec![TokenType::PRINT]) {
            self.print_statement()
        } else if self.is_match(vec![TokenType::LEFT_BRACE]) {
            Stmt::Block {
                statements: self.block(),
            }
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = vec![];

        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        match self.consume(
            TokenType::RIGHT_BRACE,
            "Expect '}' after block.".to_string(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{:?}", err),
        };

        statements
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr_result = self.expression();
        let semicolon_result =
            self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_owned());

        match semicolon_result {
            Ok(_) => (),
            Err(err) => panic!("Panicked parsing expression statement {}", err.message),
        }

        match expr_result {
            Ok(expr) => Stmt::Expression {
                expr: Box::new(expr),
            },
            Err(err) => panic!("Panicked parsing expression statement {}", err.message),
        }
    }

    fn print_statement(&mut self) -> Stmt {
        let expr_result = self.expression();
        let semicolon_result =
            self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_owned());

        match semicolon_result {
            Ok(_) => (),
            Err(err) => panic!("Panicked parsing expression statement {}", err.message),
        }

        match expr_result {
            Ok(expr) => Stmt::Print {
                expr: Box::new(expr),
            },
            Err(_) => panic!("Panicked parsing expression statement"),
        }
    }

    fn for_statement(&mut self) -> Stmt {
        // Here, we de-sugar a for loop into a while loop
        match self.consume(
            TokenType::LEFT_PAREN,
            "Expect '(' after 'while'.".to_owned(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{:?}", err),
        }

        let initializer;

        if self.is_match(vec![TokenType::SEMICOLON]) {
            initializer = None;
        } else if self.is_match(vec![TokenType::VAR]) {
            initializer = Some(self.var_declaration());
        } else {
            initializer = Some(self.expression_statement());
        }

        let mut condition = None;

        if !self.check(TokenType::SEMICOLON) {
            condition.replace(self.expression());
        }

        match self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after loop condition.".to_owned(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{:?}", err),
        }

        let mut increment = None;

        if !self.check(TokenType::RIGHT_PAREN) {
            increment.replace(self.expression());
        }

        match self.consume(
            TokenType::RIGHT_PAREN,
            "Expect ')' after for clauses.".to_owned(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{:?}", err),
        }

        let mut body = self.statement();

        if let Some(Ok(increment_expr)) = increment {
            // TODO Handle the err
            body = Stmt::Block {
                statements: vec![
                    body,
                    Stmt::Expression {
                        expr: Box::new(increment_expr),
                    },
                ],
            }
        }

        if condition.is_none() {
            condition.replace(Ok(Expr::Literal {
                value: Value::Boolean { value: true },
            }));
        }

        match condition {
            Some(Ok(condition_expr)) => {
                body = Stmt::While {
                    condition: condition_expr,
                    body: Box::new(body),
                };
            }
            Some(Err(err)) => panic!("{:?}", err),
            _ => panic!("This shouldn't happen"),
        }

        match initializer {
            Some(init_stmt) => {
                body = Stmt::Block {
                    statements: vec![init_stmt, body],
                }
            }
            None => (),
        }

        body
    }

    fn while_statement(&mut self) -> Stmt {
        match self.consume(
            TokenType::LEFT_PAREN,
            "Expect '(' after 'while'.".to_owned(),
        ) {
            Ok(_) => (),
            Err(err) => panic!("{:?}", err),
        }

        match self.expression() {
            Ok(condition) => {
                match self.consume(
                    TokenType::RIGHT_PAREN,
                    "Expect ')' after condition.".to_owned(),
                ) {
                    Ok(_) => (),
                    Err(err) => panic!("{:?}", err),
                }

                let body = Box::new(self.statement());

                Stmt::While { condition, body }
            }
            Err(err) => panic!("{:?}", err),
        }
    }

    fn if_statement(&mut self) -> Stmt {
        match self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.".to_owned()) {
            Ok(_) => (),
            Err(err) => panic!("{:?}", err),
        }

        match self.expression() {
            Ok(condition) => {
                match self.consume(
                    TokenType::RIGHT_PAREN,
                    "Expect ')' after if condition.".to_owned(),
                ) {
                    Ok(_) => (),
                    Err(err) => panic!("{:?}", err),
                }

                let then_branch = Box::new(self.statement());
                let mut else_branch = None;

                if self.is_match(vec![TokenType::ELSE]) {
                    else_branch.replace(Box::new(self.statement()));
                }

                Stmt::If {
                    condition,
                    then_branch,
                    else_branch,
                }
            }
            Err(err) => panic!("{:?}", err),
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or();

        if self.is_match(vec![TokenType::EQUAL]) {
            let _equals = self.previous();
            let value = self.assignment();

            match expr {
                Ok(expression) => match value {
                    Ok(value_expr) => match expression {
                        Expr::Variable { name } => {
                            return Ok(Expr::Assign {
                                name: name,
                                value: Box::new(value_expr),
                            })
                        }
                        _ => return Err(self.current_error("Invalid assignment target".to_owned())),
                    },
                    Err(error) => return Err(error),
                },
                Err(error) => return Err(error),
            }
        }

        expr
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        match self.and() {
            Ok(mut expr) => {
                while self.is_match(vec![TokenType::OR]) {
                    let operator = self.previous();
                    match self.and() {
                        Ok(right) => {
                            expr = Expr::Logical {
                                left: Box::new(expr),
                                operator,
                                right: Box::new(right),
                            }
                        }
                        Err(err) => return Err(err),
                    }
                }

                Ok(expr)
            }
            Err(err) => return Err(err),
        }
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        match self.equality() {
            Ok(mut expr) => {
                while self.is_match(vec![TokenType::AND]) {
                    let operator = self.previous();
                    match self.equality() {
                        Ok(right) => {
                            expr = Expr::Logical {
                                left: Box::new(expr),
                                operator,
                                right: Box::new(right),
                            }
                        }
                        Err(err) => return Err(err),
                    }
                }

                Ok(expr)
            }
            Err(err) => return Err(err),
        }
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        match self.comparison() {
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
                        Err(parse_error) => return Err(parse_error),
                    }
                }
                Ok(expr)
            }

            Err(parse_error) => return Err(parse_error),
        }
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        match self.term() {
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
                        Err(parse_error) => return Err(parse_error),
                    }
                }

                Ok(expr)
            }
            Err(parse_error) => return Err(parse_error),
        }
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        match self.factor() {
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
            Err(parse_error) => Err(parse_error),
        }
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        match self.unary() {
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
                        Err(parse_error) => return Err(parse_error),
                    }
                }

                Ok(expr)
            }
            Err(parse_error) => return Err(parse_error),
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

        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        match self.primary() {
            Ok(mut expr) => {
                loop {
                    if self.is_match(vec![TokenType::LEFT_PAREN]) {
                        match self.finish_call(expr) {
                            Ok(finish_call_expr) => expr = finish_call_expr,
                            Err(err) => return Err(err),
                        }
                    } else {
                        break;
                    }
                }

                return Ok(expr);
            }
            Err(err) => return Err(err),
        }
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = vec![];

        if !self.check(TokenType::RIGHT_PAREN) {
            loop {
                if arguments.len() >= 255 {
                    // TODO we don't actually want to return an error here, just report it.
                    return Err(self.error(
                        self.peek(),
                        "Can't have more than 255 arguments".to_string(),
                    ));
                }
                match self.expression() {
                    Ok(expression) => arguments.push(expression),
                    Err(err) => return Err(err),
                }

                if !self.is_match(vec![TokenType::COMMA]) {
                    break;
                }
            }
        }

        match self.consume(
            TokenType::RIGHT_PAREN,
            "Expect ')' after arguments.".to_string(),
        ) {
            Ok(paren) => Ok(Expr::Call {
                callee: Box::new(callee),
                paren,
                arguments,
            }),
            Err(err) => Err(err),
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.is_match(vec![TokenType::FALSE]) {
            return Ok(Expr::Literal {
                value: Value::Boolean { value: false },
            });
        }
        if self.is_match(vec![TokenType::TRUE]) {
            return Ok(Expr::Literal {
                value: Value::Boolean { value: true },
            });
        }
        if self.is_match(vec![TokenType::NIL]) {
            return Ok(Expr::Literal { value: Value::Nil });
        }
        if self.is_match(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.unwrap(),
            });
        }
        if self.is_match(vec![TokenType::IDENTIFIER]) {
            return Ok(Expr::Variable {
                name: self.previous(),
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
                Err(parse_error) => return Err(parse_error),
            }

            match expr_result {
                Ok(expr) => {
                    return Ok(Expr::Grouping {
                        expression: Box::new(expr),
                    });
                }
                Err(parse_error) => return Err(parse_error),
            }
        }
        Err(self.current_error(format!("this shouldn't happen {:?}", self.peek())))
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

    // fn synchronize(&mut self) {
    //     self.advance();

    //     while !self.is_at_end() {
    //         if self.previous().ttype == TokenType::SEMICOLON {
    //             return;
    //         }

    //         match self.peek().ttype {
    //             TokenType::CLASS => return,
    //             TokenType::FUN => return,
    //             TokenType::VAR => return,
    //             TokenType::FOR => return,
    //             TokenType::IF => return,
    //             TokenType::WHILE => return,
    //             TokenType::PRINT => return,
    //             TokenType::RETURN => return,
    //             _ => self.advance(),
    //         };
    //     }
    // }

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
