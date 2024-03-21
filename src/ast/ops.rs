use crate::{ast::*, token::Span};

#[derive(Debug)]
pub struct BinaryOperator {
    pub kind: BinaryOperatorKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum BinaryOperatorKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

#[derive(Debug)]
pub struct UnaryOperator {
    pub kind: UnaryOperatorKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum UnaryOperatorKind {
    Negate,
    Not,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        self.kind.precedence()
    }
}

impl BinaryOperatorKind {
    /// Get the precedence of the operator
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperatorKind::Or => 1,
            BinaryOperatorKind::And => 2,
            BinaryOperatorKind::Equal | BinaryOperatorKind::NotEqual => 3,
            BinaryOperatorKind::LessThan
            | BinaryOperatorKind::GreaterThan
            | BinaryOperatorKind::LessThanOrEqual
            | BinaryOperatorKind::GreaterThanOrEqual => 4,
            BinaryOperatorKind::Add | BinaryOperatorKind::Subtract => 5,
            BinaryOperatorKind::Multiply
            | BinaryOperatorKind::Divide
            | BinaryOperatorKind::Modulus => 6,
        }
    }
}

impl PrettyPrint for UnaryOperatorKind {
    fn pretty_print(&self, _indent: usize) -> String {
        match self {
            UnaryOperatorKind::Negate => "Negate ".to_string(),
            UnaryOperatorKind::Not => "Not ".to_string(),
        }
    }
}

impl PrettyPrint for BinaryOperator {
    fn pretty_print(&self, _indent: usize) -> String {
        match self.kind {
            BinaryOperatorKind::Add => "Add".to_string(),
            BinaryOperatorKind::Subtract => "Subtract".to_string(),
            BinaryOperatorKind::Multiply => "Multiply".to_string(),
            BinaryOperatorKind::Divide => "Divide".to_string(),
            BinaryOperatorKind::Modulus => "Modulus".to_string(),
            BinaryOperatorKind::Equal => "Equal".to_string(),
            BinaryOperatorKind::NotEqual => "NotEqual".to_string(),
            BinaryOperatorKind::LessThan => "LessThan".to_string(),
            BinaryOperatorKind::GreaterThan => "GreaterThan".to_string(),
            BinaryOperatorKind::LessThanOrEqual => "LessThanOrEqual".to_string(),
            BinaryOperatorKind::GreaterThanOrEqual => "GreaterThanOrEqual".to_string(),
            BinaryOperatorKind::And => "And".to_string(),
            BinaryOperatorKind::Or => "Or".to_string(),
        }
    }
}
