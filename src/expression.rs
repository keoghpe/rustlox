use crate::token::Token;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: String,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    fn accept<A>(&self, visitor: &dyn ExprVisitor<A>) -> A {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(self),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(self),
            Expr::Literal { value } => visitor.visit_literal_expr(self),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(self),
        }
    }
}

trait ExprVisitor<A> {
    fn visit_binary_expr(&self, expr: &Expr) -> A;
    fn visit_grouping_expr(&self, expr: &Expr) -> A;
    fn visit_literal_expr(&self, expr: &Expr) -> A;
    fn visit_unary_expr(&self, expr: &Expr) -> A;
}

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: String, expr1: &Expr, expr2: Option<&Expr>) -> String {
        let mut string = ("(".to_string() + &name + " ").to_owned() + &expr1.accept(self);

        match expr2 {
            Some(expr) => string = string + " " + &expr.accept(self),
            None => (),
        }

        string = string + ")";

        string
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(operator.lexeme.to_string(), &*left, Some(&*right)),
            _ => panic!("Nope!"),
        }
    }

    fn visit_grouping_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Grouping { expression } => {
                self.parenthesize("group".to_owned(), &*expression, None)
            }
            _ => panic!("Nope!"),
        }
    }

    fn visit_literal_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal { value } => value.to_string(),
            _ => panic!("Nope!"),
        }
    }

    fn visit_unary_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Unary { operator, right } => {
                self.parenthesize(operator.lexeme.to_string(), &*right, None)
            }
            _ => panic!("Nope!"),
        }
    }
}
