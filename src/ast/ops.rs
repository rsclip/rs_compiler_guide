use crate::ast::*;

#[derive(Debug)]
pub enum BinaryOperator {
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

pub enum UnaryOperator {
    Negate,
    Not,
}

impl BinaryOperator {
    /// Get the precedence of the operator
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Or => 1,
            BinaryOperator::And => 2,
            BinaryOperator::Equal | BinaryOperator::NotEqual => 3,
            BinaryOperator::LessThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::LessThanOrEqual
            | BinaryOperator::GreaterThanOrEqual => 4,
            BinaryOperator::Add | BinaryOperator::Subtract => 5,
            BinaryOperator::Multiply | BinaryOperator::Divide | BinaryOperator::Modulus => 6,
        }
    }
}

impl PrettyPrint for UnaryOperator {
    fn pretty_print(&self, _indent: usize) -> String {
        match self {
            UnaryOperator::Negate => "Negate ".to_string(),
            UnaryOperator::Not => "Not ".to_string(),
        }
    }
}

impl PrettyPrint for BinaryOperator {
    fn pretty_print(&self, _indent: usize) -> String {
        match self {
            BinaryOperator::Add => "Add".to_string(),
            BinaryOperator::Subtract => "Subtract".to_string(),
            BinaryOperator::Multiply => "Multiply".to_string(),
            BinaryOperator::Divide => "Divide".to_string(),
            BinaryOperator::Modulus => "Modulus".to_string(),
            BinaryOperator::Equal => "Equal".to_string(),
            BinaryOperator::NotEqual => "NotEqual".to_string(),
            BinaryOperator::LessThan => "LessThan".to_string(),
            BinaryOperator::GreaterThan => "GreaterThan".to_string(),
            BinaryOperator::LessThanOrEqual => "LessThanOrEqual".to_string(),
            BinaryOperator::GreaterThanOrEqual => "GreaterThanOrEqual".to_string(),
            BinaryOperator::And => "And".to_string(),
            BinaryOperator::Or => "Or".to_string(),
        }
    }
}
