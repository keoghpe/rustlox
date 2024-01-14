enum Expr {
    Binary {
        left: Expr,
        operator: Token,
        right: Expr,
    },
    Grouping {
        expression: Expr,
    },
    Literal {
        value: String,
    },
    Unary {
        operator: Token,
        right: Expr,
    },
}

impl Expr {
    fn accept(&self, visitor: ExprVisitor<A>) -> A {
        match self {
            Expr::Binary => visitor.visit_binary_expr(&self),
            Expr::Grouping => visitor.visit_grouping_expr(&self),
            Expr::Literal => visitor.visit_literal_expr(&self),
            Expr::Unary => visitor.visit_unary_expr(&self),
        }
    }
}

trait ExprVisitor<A> {
    fn visit_binary_expr(expr: Expr::Binary) -> A;
    fn visit_grouping_expr(expr: Expr::Grouping) -> A;
    fn visit_literal_expr(expr: Expr::Literal) -> A;
    fn visit_unary_expr(expr: Expr::Unary) -> A;
}

struct AstPrinter {}

impl AstPrinter {
    fn print(expr: Expr) -> String {
        expr.accept(&self)
    }

    fn parenthesize(name: String, expr1: Expr, expr2: Option<Expr>) -> String {
        let mut string = "(" + name + " " + expr1.accept(self);

        match expr2 {
            Some(expr) => string = string + " " + expr.accept(self),
            None => (),
        }

        string = string + ")";

        string
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: Expr::Binary) -> String {
        self.parenthesize(expr.operator.lexeme, expr.left, Some(expr.right))
    }

    fn visit_grouping_expr(&self, expr: Expr::Grouping) -> String {
        self.parenthesize("group", expr.expression, None)
    }

    fn visit_literal_expr(&self, expr: Expr::Literal) -> String {
        expr.value
    }

    fn visit_unary_expr(&self, expr: Expr::Unary) -> String {
        self.parenthesize(expr.operator.lexeme, expr.right, None)
    }
}
