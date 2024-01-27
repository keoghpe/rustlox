use core::panic;

use crate::{
    expression::{Expr, ExprVisitor},
    token::{Token, TokenType, Value},
};

pub struct Interpreter {}

pub struct RuntimeError {
    // TODO Replace operator with token so we can print the line number in the error
    operator: TokenType,
    error: String,
}

impl RuntimeError {
    fn to_string(&self) -> String {
        format!("Error: {} ({})", self.error, self.operator)
    }
}

impl Interpreter {
    pub fn interpret(&self, expr: &Expr) -> String {
        match self.evaluate(expr) {
            Ok(value) => value.to_string(),
            Err(runtime_error) => runtime_error.to_string(),
        }
    }

    pub fn evaluate(&self, expr: &Expr) -> Result<Value, RuntimeError> {
        expr.accept(self)
    }

    pub fn is_truthy(&self, val: Value) -> bool {
        match val {
            Value::Boolean { value } => value,
            Value::Double { value: _ } => true,
            Value::String { value: _ } => true,
            Value::Nil => false,
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
}

impl ExprVisitor<Result<Value, RuntimeError>> for Interpreter {
    fn visit_binary_expr(&self, expr: &crate::expression::Expr) -> Result<Value, RuntimeError> {
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
                            op => Err(RuntimeError {
                                operator: op,
                                error: "Cannot perform this operation on a number".to_string(),
                            }),
                        },
                        Value::Boolean { value } => Err(RuntimeError {
                            operator: operator.ttype,
                            error: "Cannot perform this with a number and boolean".to_string(),
                        }),
                        Value::String { value } => Err(RuntimeError {
                            operator: operator.ttype,
                            error: "Cannot perform this with a number and a string".to_string(),
                        }),
                        Value::Nil => Err(RuntimeError {
                            operator: operator.ttype,
                            error: "Cannot perform this with a number and nil".to_string(),
                        }),
                    },
                    Value::String { value: left_value } => match operator.ttype {
                        crate::token::TokenType::PLUS => Ok(Value::String {
                            value: left_value.to_string() + &right_val.to_string(),
                        }),
                        op => Err(RuntimeError {
                            operator: op,
                            error: "Cannot perform this operation on a string".to_string(),
                        }),
                    },
                    _ => match operator.ttype {
                        op => Err(RuntimeError {
                            operator: op,
                            error: "Cannot perform this operation on this type".to_string(),
                        }),
                    },
                }
            }
            _ => panic!("NOT A BINARY EXPRESSION"),
        }
    }

    fn visit_grouping_expr(&self, expr: &crate::expression::Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Grouping { expression } => self.evaluate(&expression),
            _ => panic!("Nope!"),
        }
    }

    fn visit_literal_expr(&self, expr: &crate::expression::Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(value.clone()),
            _ => panic!("Nope!"),
        }
    }

    fn visit_unary_expr(&self, expr: &crate::expression::Expr) -> Result<Value, RuntimeError> {
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
                        value: !self.is_truthy(right_val),
                    }),
                    _ => panic!("Nope!"),
                }
            }
            _ => panic!("Nope!"),
        }
    }
}
