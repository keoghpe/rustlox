use std::collections::HashMap;

use crate::{
    expression::{Expr, ExprVisitor, Stmt, StmtVisitor},
    interpreter::Interpreter,
    token::Token,
};

// Macro to simplify match statements with panic on non-match
macro_rules! match_or_panic {
    ($value:expr, $pattern:pat => $body:block) => {
        match $value {
            $pattern => $body,
            _ => panic!(),
        }
    };
}

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_statements(&mut self, statements: &Vec<Stmt>) {
        for statement in statements.iter() {
            self.resolve_statement(statement);
        }
    }

    fn resolve_statement(&mut self, statement: &Stmt) {
        statement.accept(self)
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_string(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_string(), true);
        }
    }

    fn resolve_expression(&mut self, expr: &Expr) {}

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            let scope = &self.scopes[i];
            if let Some(_) = scope.get(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
            }
        }
    }

    fn resolve_function(&mut self, function: &Stmt) {
        match_or_panic!(function, Stmt::Function { name, params, body } => {
            self.begin_scope();

            for param in params.into_iter() {
                self.declare(name);
                self.define(name);
            }

            self.resolve_statements(body);
            self.end_scope();
        });
    }
}

impl ExprVisitor<()> for Resolver {
    fn visit_assign_expr(&mut self, expr: &Expr) {
        match_or_panic!(expr, Expr::Assign { name, value } => {
            self.resolve_expression(value);
            self.resolve_local(expr, name);
        });
    }

    fn visit_binary_expr(&mut self, expr: &Expr) {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Expr) {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &Expr) {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &Expr) {
        todo!()
    }

    fn visit_variable_expr(&mut self, expr: &Expr) {
        match_or_panic!(expr, Expr::Variable { name } => {
            if let Some(scope) = self.scopes.last() {
                if let Some(value) = scope.get(&name.lexeme) {
                    if *value == false {
                        // error
                        panic!("You can't do that - crashing");
                    }
                }
            }

            self.resolve_local(expr, name);
        });
    }

    fn visit_logical_expr(&mut self, expr: &Expr) {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &Expr) {
        todo!()
    }
}

impl StmtVisitor<()> for Resolver {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) {
        match_or_panic!(stmt, Stmt::Expression { expr } => {
            self.resolve_expression(expr.as_ref());
        });
    }

    fn visit_print_stmt(&mut self, stmt: &Stmt) {}

    fn visit_variable_stmt(&mut self, stmt: &Stmt) {
        match_or_panic!(stmt, Stmt::Var { name, initializer } => {
            self.declare(name);
            match initializer {
                Some(init) => self.resolve_expression(init),
                None => (),
            }
            self.define(name);
        });
    }

    fn visit_block_stmt(&mut self, stmt: &Stmt) {
        match_or_panic!(stmt, Stmt::Block { statements } => {
            self.begin_scope();
            self.resolve_statements(statements);
            self.end_scope();
        });
    }

    fn visit_if_stmt(&mut self, stmt: &Stmt) {
        match_or_panic!(stmt, Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            self.resolve_expression(condition);
            self.resolve_statement(then_branch.as_ref());
            if let Some(else_stmt) = else_branch {
                self.resolve_statement(else_stmt.as_ref());
            }
        });
    }

    fn visit_while_stmt(&mut self, stmt: &Stmt) {
        todo!()
    }

    fn visit_function_stmt(&mut self, stmt: &Stmt) {
        match_or_panic!(stmt, Stmt::Function { name, params, body } => {
            self.declare(name);
            self.define(name);
            self.resolve_function(stmt);
        });
    }

    fn visit_return_stmt(&mut self, stmt: &Stmt) {
        todo!()
    }
}
