use crate::{
    interpreter::Return,
    token::{Token, Value},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
}

impl Expr {
    pub fn accept<A>(&self, visitor: &mut dyn ExprVisitor<A>) -> A {
        match self {
            Expr::Assign { name: _, value: _ } => visitor.visit_assign_expr(self),
            Expr::Binary {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_binary_expr(self),
            Expr::Grouping { expression: _ } => visitor.visit_grouping_expr(self),
            Expr::Literal { value: _ } => visitor.visit_literal_expr(self),
            Expr::Unary {
                operator: _,
                right: _,
            } => visitor.visit_unary_expr(self),
            // TODO replace with Macro?
            Expr::Variable { name: _ } => visitor.visit_variable_expr(self),
            Expr::Logical {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_logical_expr(self),
            Expr::Call {
                callee: _,
                paren: _,
                arguments: _,
            } => visitor.visit_call_expr(self),
        }
    }
}

pub trait ExprVisitor<A> {
    // TODO replace with Macro?
    fn visit_assign_expr(&mut self, expr: &Expr) -> A;
    fn visit_binary_expr(&mut self, expr: &Expr) -> A;
    fn visit_grouping_expr(&mut self, expr: &Expr) -> A;
    fn visit_literal_expr(&mut self, expr: &Expr) -> A;
    fn visit_unary_expr(&mut self, expr: &Expr) -> A;
    fn visit_variable_expr(&mut self, expr: &Expr) -> A;
    fn visit_logical_expr(&mut self, expr: &Expr) -> A;
    fn visit_call_expr(&mut self, expr: &Expr) -> A;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expr: Box<Expr>,
    },
    Print {
        expr: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Return {
        keyword: Token,
        value: Box<Expr>,
    },
}

impl Stmt {
    pub fn accept<A>(&self, visitor: &mut dyn StmtVisitor<A>) -> A {
        match self {
            Stmt::Expression { expr: _ } => visitor.visit_expression_stmt(self),
            Stmt::Print { expr: _ } => visitor.visit_print_stmt(self),
            Stmt::Var {
                name: _,
                initializer: _,
            } => visitor.visit_variable_stmt(self),
            Stmt::Block { statements: _ } => visitor.visit_block_stmt(self),
            Stmt::If {
                condition: _,
                then_branch: _,
                else_branch: _,
            } => visitor.visit_if_stmt(self),
            Stmt::While {
                condition: _,
                body: _,
            } => visitor.visit_while_stmt(self),
            Stmt::Function {
                name: _,
                params: _,
                body: _,
            } => visitor.visit_function_stmt(self),
            Stmt::Return { keyword, value } => visitor.visit_return_stmt(self),
        }
    }
}

pub trait StmtVisitor<A> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> A;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> A;
    fn visit_variable_stmt(&mut self, stmt: &Stmt) -> A;
    fn visit_block_stmt(&mut self, stmt: &Stmt) -> A;
    fn visit_if_stmt(&mut self, stmt: &Stmt) -> A;
    fn visit_while_stmt(&mut self, stmt: &Stmt) -> A;
    fn visit_function_stmt(&mut self, stmt: &Stmt) -> A;
    fn visit_return_stmt(&mut self, stmt: &Stmt) -> A;
}

// pub struct AstPrinter {}

// impl AstPrinter {
//     // pub fn print(&self, expr: &Expr) -> String {
//     //     expr.accept(self)
//     // }

//     fn parenthesize(&mut self, name: String, expr1: &Expr, expr2: Option<&Expr>) -> String {
//         let mut string = ("(".to_string() + &name + " ").to_owned() + &expr1.accept(self);

//         match expr2 {
//             Some(expr) => string = string + " " + &expr.accept(self),
//             None => (),
//         }

//         string = string + ")";

//         string
//     }
// }

// impl ExprVisitor<String> for AstPrinter {
//     fn visit_binary_expr(&self, expr: &Expr) -> String {
//         match expr {
//             Expr::Binary {
//                 left,
//                 operator,
//                 right,
//             } => self.parenthesize(operator.lexeme.to_string(), &*left, Some(&*right)),
//             _ => panic!("Nope!"),
//         }
//     }

//     fn visit_grouping_expr(&self, expr: &Expr) -> String {
//         match expr {
//             Expr::Grouping { expression } => {
//                 self.parenthesize("group".to_owned(), &*expression, None)
//             }
//             _ => panic!("Nope!"),
//         }
//     }

//     fn visit_literal_expr(&self, expr: &Expr) -> String {
//         match expr {
//             Expr::Literal { value } => value.to_string(),
//             _ => panic!("Nope!"),
//         }
//     }

//     fn visit_unary_expr(&self, expr: &Expr) -> String {
//         match expr {
//             Expr::Unary { operator, right } => {
//                 self.parenthesize(operator.lexeme.to_string(), &*right, None)
//             }
//             _ => panic!("Nope!"),
//         }
//     }

//     fn visit_variable_expr(&self, expr: &Expr) -> String {
//         match expr {
//             Expr::Variable { name: _ } => {
//                 todo!()
//             }
//             _ => panic!("Nope!"),
//         }
//     }

//     fn visit_assign_expr(&self, _expr: &Expr) -> String {
//         todo!()
//     }

//     fn visit_logical_expr(&self, _expr: &Expr) -> String {
//         todo!()
//     }

//     fn visit_call_expr(&mut self, expr: &Expr) -> String {
//         todo!()
//     }
// }
