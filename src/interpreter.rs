use core::panic;
use std::{
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    environment::Environment,
    expression::{Expr, ExprVisitor, Stmt, StmtVisitor},
    token::{Callable, Token, TokenType, Value},
};

pub struct Interpreter {
    pub global: Rc<Environment>,
    environment: Rc<Environment>,
}

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    RuntimeError {
        // TODO Replace operator with token so we can print the line number in the error
        operator: TokenType,
        error: String,
    },
    Return {
        value: Value,
    },
}

impl InterpreterError {
    pub fn new_runtime_error(operator: TokenType, error: String) -> Self {
        Self::RuntimeError { operator, error }
    }

    // fn to_string(&self) -> String {
    //     format!("Error: {} ({})", self.error, self.operator)
    // }
}

pub type StatementResult = Result<(), InterpreterError>;
pub type ExpressionResult = Result<Value, InterpreterError>;

impl Interpreter {
    pub fn new() -> Interpreter {
        let env = Rc::new(Environment::new(None));

        let interpreter = Interpreter {
            environment: Rc::clone(&env),
            global: Rc::clone(&env),
        };

        // Native function definitions
        interpreter.environment.define(
            &Token {
                ttype: TokenType::IDENTIFIER,
                lexeme: "clock".to_string(),
                literal: None,
                line: 0,
            },
            &Value::Callable {
                callable: Callable::NativeFunction {
                    arity: 0,
                    call: {
                        |_interpreter, _arguments| {
                            let start = SystemTime::now();
                            let since_the_epoch = start
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards");

                            Ok(Value::Double {
                                value: since_the_epoch.as_millis() as f64,
                            })
                        }
                    },
                    value: "<native fn>".to_owned(),
                },
            },
        );

        interpreter
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements.into_iter() {
            match self.execute(statement) {
                Ok(_) => (),
                Err(err) => {
                    println!("Runtime Error caught at `interpret`: {:?}", err);
                    break;
                }
            }
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> StatementResult {
        stmt.accept(self)
    }

    pub fn evaluate(&mut self, expr: &Expr) -> ExpressionResult {
        expr.accept(self)
    }

    pub fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Boolean { value } => value.clone(),
            Value::Double { value: _ } => true,
            Value::String { value: _ } => true,
            Value::Nil => false,
            Value::Callable { callable: _ } => true,
        }
    }

    pub fn is_equal(&self, left: &Value, right: &Value) -> Value {
        Value::Boolean {
            value: Self::values_are_equal(left, right),
        }
    }

    pub fn is_not_equal(&self, left: &Value, right: &Value) -> Value {
        Value::Boolean {
            value: !Self::values_are_equal(left, right),
        }
    }

    fn values_are_equal(left: &Value, right: &Value) -> bool {
        left == right
    }

    // TODO Does this need to return a Return?
    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Environment,
    ) -> StatementResult {
        // Create a new env that refers to the current env
        // Replace the current env with the new env
        // Process the statements
        // Reset the env back
        // //
        // let parent_env = self.environment.take();
        let prev = Rc::clone(&self.environment);
        self.environment = environment.into();

        for statement in statements.into_iter() {
            // TODO Do we need to break out here to return?
            match self.execute(&statement) {
                Ok(_) => (),
                Err(err) => {
                    // reset environment - TODO Confirm if needed
                    self.environment = prev;
                    return Err(err);
                }
            }
        }

        // TODO - I can't remember what I meant by the following comment, maybe it can be removed.
        // TODO - this needs to reset to the previous env, not the enclosing of the current env.
        self.environment = prev;
        Ok(())
    }
}

impl ExprVisitor<ExpressionResult> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &crate::expression::Expr) -> ExpressionResult {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = match self.evaluate(left) {
                    Ok(val) => val,
                    Err(runtime_error) => return Err(runtime_error),
                };

                let right_val = match self.evaluate(right) {
                    Ok(val) => val,
                    Err(runtime_error) => return Err(runtime_error),
                };

                match &operator.ttype {
                    TokenType::EQUAL_EQUAL => return Ok(self.is_equal(&left_val, &right_val)),
                    TokenType::BANG_EQUAL => return Ok(self.is_not_equal(&left_val, &right_val)),
                    _ => (), // do nothing here, evalue the operator based on the left type below
                }

                match &left_val {
                    Value::Double { value: left_value } => match &right_val {
                        Value::Double { value: right_value } => match operator.ttype {
                            TokenType::MINUS => Ok(Value::Double {
                                value: left_value - right_value,
                            }),
                            TokenType::PLUS => Ok(Value::Double {
                                value: left_value + right_value,
                            }),
                            TokenType::SLASH => Ok(Value::Double {
                                value: left_value / right_value,
                            }),
                            TokenType::STAR => Ok(Value::Double {
                                value: left_value * right_value,
                            }),
                            TokenType::GREATER => Ok(Value::Boolean {
                                value: left_value > right_value,
                            }),
                            TokenType::GREATER_EQUAL => Ok(Value::Boolean {
                                value: left_value >= right_value,
                            }),
                            TokenType::LESS => Ok(Value::Boolean {
                                value: left_value < right_value,
                            }),
                            TokenType::LESS_EQUAL => Ok(Value::Boolean {
                                value: left_value <= right_value,
                            }),
                            op => Err(InterpreterError::RuntimeError {
                                operator: op,
                                error: "Cannot perform this operation on a number".to_string(),
                            }),
                        },
                        Value::Boolean { value: _ } => Err(InterpreterError::RuntimeError {
                            operator: operator.ttype,
                            error: "Cannot perform this with a number and boolean".to_string(),
                        }),
                        Value::String { value: _ } => Err(InterpreterError::RuntimeError {
                            operator: operator.ttype,
                            error: "Cannot perform this with a number and a string".to_string(),
                        }),
                        Value::Nil => Err(InterpreterError::RuntimeError {
                            operator: operator.ttype,
                            error: "Cannot perform this with a number and nil".to_string(),
                        }),
                        // TODO - Maybe this is a bug??
                        Value::Callable { callable: _ } => Err(InterpreterError::RuntimeError {
                            operator: operator.ttype,
                            error: "Cannot perform this with a number and Callable".to_string(),
                        }),
                    },
                    Value::String { value: left_value } => match operator.ttype {
                        crate::token::TokenType::PLUS => Ok(Value::String {
                            value: left_value.to_string() + &right_val.to_string(),
                        }),
                        op => Err(InterpreterError::RuntimeError {
                            operator: op,
                            error: "Cannot perform this operation on a string".to_string(),
                        }),
                    },
                    _ => match operator.ttype {
                        op => Err(InterpreterError::RuntimeError {
                            operator: op,
                            error: "Cannot perform this operation on this type".to_string(),
                        }),
                    },
                }
            }
            _ => panic!("NOT A BINARY EXPRESSION"),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &crate::expression::Expr) -> ExpressionResult {
        match expr {
            Expr::Grouping { expression } => self.evaluate(&expression),
            _ => panic!("Nope!"),
        }
    }

    fn visit_literal_expr(&mut self, expr: &crate::expression::Expr) -> ExpressionResult {
        match expr {
            Expr::Literal { value } => Ok(value.clone()),
            _ => panic!("Nope!"),
        }
    }

    fn visit_unary_expr(&mut self, expr: &crate::expression::Expr) -> ExpressionResult {
        match expr {
            Expr::Unary { operator, right } => {
                let right_val = match self.evaluate(right) {
                    Ok(val) => val,
                    Err(runtime_error) => return Err(runtime_error),
                };

                match operator.ttype {
                    crate::token::TokenType::MINUS => match right_val {
                        Value::Double { value } => Ok(Value::Double { value: -value }),
                        // We could handle strings here
                        _ => panic!("Nope!"),
                    },
                    crate::token::TokenType::BANG => Ok(Value::Boolean {
                        value: !self.is_truthy(&right_val),
                    }),
                    _ => panic!("Nope!"),
                }
            }
            _ => panic!("Nope!"),
        }
    }

    fn visit_variable_expr(&mut self, expr: &Expr) -> ExpressionResult {
        match expr {
            Expr::Variable { name } => self.environment.get(name.clone()),
            _ => panic!("Nope!"),
        }
    }

    fn visit_assign_expr(&mut self, expr: &Expr) -> ExpressionResult {
        match expr {
            Expr::Assign { name, value } => {
                let value = self.evaluate(value);

                match value {
                    Ok(expression_value) => self.environment.assign(name, &expression_value),
                    Err(err) => Err(err),
                }
            }
            _ => panic!("Nope!"),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Expr) -> ExpressionResult {
        if let Expr::Logical {
            left,
            operator,
            right,
        } = expr
        {
            let left_result = self.evaluate(left);

            match left_result {
                Ok(left) => {
                    if operator.ttype == TokenType::OR {
                        if self.is_truthy(&left) {
                            return Ok(left);
                        }
                    } else {
                        if !self.is_truthy(&left) {
                            return Ok(left);
                        }
                    }

                    self.evaluate(right)
                }
                Err(err) => Err(err),
            }
        } else {
            panic!("Nope!")
        }
    }

    fn visit_call_expr(&mut self, expr: &Expr) -> ExpressionResult {
        if let Expr::Call {
            callee,
            paren,
            arguments,
        } = expr
        {
            let callee_res = self.evaluate(&callee);
            let mut func_arguments = vec![];

            for arg in arguments.into_iter() {
                match self.evaluate(arg) {
                    Ok(arg_value) => func_arguments.push(arg_value),
                    Err(err) => return Err(err),
                }
            }

            if let Ok(Value::Callable { callable }) = callee_res {
                if func_arguments.len() == callable.arity() as usize {
                    callable.call(self, &func_arguments)
                } else {
                    return Err(InterpreterError::RuntimeError {
                        operator: paren.ttype,
                        // TODO Interpolate this error correctly
                        error: "Expected x arguments, but got y".to_owned(),
                    });
                }
            } else {
                return Err(InterpreterError::RuntimeError {
                    operator: paren.ttype,
                    error: "Can only call functions and classes.".to_owned(),
                });
            }
        } else {
            panic!("Nope!")
        }
    }
}

impl StmtVisitor<StatementResult> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &crate::expression::Stmt) -> StatementResult {
        // println!("Visiting expression statement");
        match stmt {
            Stmt::Expression { expr } => {
                // TODO statements should raise errors
                match self.evaluate(expr) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            }
            _ => panic!("Nope!"),
        }
    }

    fn visit_print_stmt(&mut self, stmt: &crate::expression::Stmt) -> StatementResult {
        match stmt {
            Stmt::Print { expr } => match self.evaluate(expr) {
                Ok(value) => {
                    println!("{}", value);
                    Ok(())
                }
                Err(err) => Err(err),
            },
            _ => panic!("Nope!"),
        }
    }

    fn visit_variable_stmt(&mut self, stmt: &Stmt) -> StatementResult {
        match stmt {
            Stmt::Var { name, initializer } => {
                // TODO statements should raise errors

                match initializer {
                    Some(initializer_expression) => match self.evaluate(initializer_expression) {
                        Ok(value) => {
                            self.environment.define(&name, &value);
                            Ok(())
                        }
                        Err(err) => Err(err),
                    },
                    None => {
                        self.environment.define(&name, &Value::Nil);
                        Ok(())
                    }
                }
            }
            _ => panic!("Nope!"),
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) -> StatementResult {
        match stmt {
            Stmt::Block { statements } => {
                let env = Environment::new(Some(Rc::clone(&self.environment)));
                self.execute_block(statements, env)
            }
            _ => panic!("Nope!"),
        }
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) -> StatementResult {
        if let Stmt::If {
            condition,
            then_branch,
            else_branch,
        } = stmt
        {
            let value = self.evaluate(condition).unwrap();
            if self.is_truthy(&value) {
                self.execute(&then_branch)
            } else if let Some(else_stmt) = else_branch {
                self.execute(else_stmt)
            } else {
                Ok(())
            }
        } else {
            panic!("Nope")
        }
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) -> StatementResult {
        if let Stmt::While { condition, body } = stmt {
            loop {
                // TODO replace unwrap with match
                let condition_result = self.evaluate(condition);

                match condition_result {
                    Ok(val) => {
                        if !self.is_truthy(&val) {
                            break;
                        }
                        // TODO Execute should return runtime errors if it breaks
                        let exec_result = self.execute(&body);

                        match exec_result {
                            Ok(_) => (),
                            Err(err) => return Err(err),
                        }
                    }
                    Err(err) => return Err(err),
                }
            }
            Ok(())
        } else {
            panic!("Nope")
        }
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) -> StatementResult {
        if let Stmt::Function {
            name,
            params: _,
            body: _,
        } = stmt
        {
            self.environment.define(
                name,
                &Value::Callable {
                    callable: Callable::Function {
                        declaration: Box::new(stmt.clone()),
                        closure: Rc::clone(&self.environment),
                    },
                },
            );

            Ok(())
        } else {
            panic!("Nope")
        }
    }

    // TODO This should return a return with an enclosing value
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> StatementResult {
        if let Stmt::Return { keyword: _, value } = stmt {
            // self.environment.define(
            //     name,
            //     &Value::Callable {
            //         callable: Callable::Function {
            //             declaration: Box::new(stmt.clone()),
            //         },
            //     },
            // )

            let value = self.evaluate(value);

            match value {
                Ok(value) => Err(InterpreterError::Return { value }),
                Err(err) => Err(err),
            }
        } else {
            panic!("Nope")
        }
    }
}
