use core::panic;

use crate::{
    expression::{Expr, ExprVisitor},
    token::{Token, TokenType, Value},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, expr: &Expr) -> String {
        self.evaluate(expr).to_string()
    }

    pub fn evaluate(&self, expr: &Expr) -> Value {
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

impl ExprVisitor<Value> for Interpreter {
    fn visit_binary_expr(&self, expr: &crate::expression::Expr) -> Value {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left);
                let right_val = self.evaluate(right);

                match &operator.ttype {
                    TokenType::EQUAL_EQUAL => return self.is_equal(&left_val, &right_val),
                    TokenType::BANG_EQUAL => return self.is_not_equal(&left_val, &right_val),
                    _ => (), // do nothing here, evalue the operator based on the left type below
                }

                match &left_val {
                    Value::Double { value: left_value } => match &right_val {
                        Value::Double { value: right_value } => match operator.ttype {
                            TokenType::MINUS => Value::Double {
                                value: left_value - right_value,
                            },
                            TokenType::PLUS => Value::Double {
                                value: left_value + right_value,
                            },
                            TokenType::SLASH => Value::Double {
                                value: left_value / right_value,
                            },
                            TokenType::STAR => Value::Double {
                                value: left_value * right_value,
                            },
                            TokenType::GREATER => Value::Boolean {
                                value: left_value > right_value,
                            },
                            TokenType::GREATER_EQUAL => Value::Boolean {
                                value: left_value >= right_value,
                            },
                            TokenType::LESS => Value::Boolean {
                                value: left_value < right_value,
                            },
                            TokenType::LESS_EQUAL => Value::Boolean {
                                value: left_value <= right_value,
                            },
                            _ => panic!("THIS DOESNT WORK"),
                        },
                        _ => panic!("THIS DOESNT WORK"),
                    },
                    Value::String { value: left_value } => match operator.ttype {
                        crate::token::TokenType::PLUS => Value::String {
                            value: left_value.to_string() + &right_val.to_string(),
                        },
                        _ => panic!("THIS DOESNT WORK"),
                    },
                    _ => match operator.ttype {
                        _ => panic!("THIS DOESNT WORK"),
                    },
                }
            }
            _ => panic!("THIS DOESNT WORK"),
        }
    }

    fn visit_grouping_expr(&self, expr: &crate::expression::Expr) -> Value {
        match expr {
            Expr::Grouping { expression } => self.evaluate(&expression),
            _ => panic!("Nope!"),
        }
    }

    fn visit_literal_expr(&self, expr: &crate::expression::Expr) -> Value {
        match expr {
            Expr::Literal { value } => value.clone(),
            _ => panic!("Nope!"),
        }
    }

    fn visit_unary_expr(&self, expr: &crate::expression::Expr) -> Value {
        match expr {
            Expr::Unary { operator, right } => {
                let right_value = self.evaluate(right);
                match operator.ttype {
                    crate::token::TokenType::MINUS => match right_value {
                        Value::Double { value } => Value::Double { value: -value },
                        // We could handle strings here
                        _ => panic!("Nope!"),
                    },
                    crate::token::TokenType::BANG => Value::Boolean {
                        value: !self.is_truthy(right_value),
                    },
                    _ => panic!("Nope!"),
                }
            }
            _ => panic!("Nope!"),
        }
    }
}
