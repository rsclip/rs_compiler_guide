use crate::{ast::*, token::Span};

#[derive(Debug)]
pub enum Expression {
    Primary(PrimaryExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Expression {
    /// Get the binary operator of the expression, if any
    pub fn binary_operator(&self) -> Option<&BinaryOperator> {
        match self {
            Expression::Binary(binary_expr) => Some(&binary_expr.op),
            _ => None,
        }
    }
}

impl PrettyPrint for Block {
    // "Block\n" + statements
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = format!("{:indent$}Block\n", "", indent = indent * 4);
        for statement in &self.statements {
            s.push_str(&statement.pretty_print(indent + 1));
        }
        s
    }
}

impl PrettyPrint for Expression {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Expression::Primary(p) => p.pretty_print(indent),
            Expression::Unary(u) => u.pretty_print(indent),
            Expression::Binary(b) => b.pretty_print(indent),
        }
    }
}

impl ASTSpan for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::Primary(p) => p.span(),
            Expression::Unary(u) => u.span.clone(),
            Expression::Binary(b) => b.span.clone(),
        }
    }
}