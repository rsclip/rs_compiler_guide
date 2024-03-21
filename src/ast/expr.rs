use crate::errors::SemanticError;
use crate::semantic_analysis::{Analysis, SymbolTable};
use crate::{ast::*, token::Span};
use anyhow::{anyhow, Error, Result};

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

impl Analysis for Block {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        for statement in &self.statements {
            errors.extend(statement.analyze(table));
        }

        errors
    }
}

impl Block {
    /// Explore for all return statements
    /// Return:
    /// 1. The return values
    /// 2. Whether or not it guarantees a return regardless of the path
    pub fn get_return_stmts(&self) -> (Vec<&Statement>, bool) {
        let mut return_stms = Vec::new();
        let mut guaranteed_return = false;

        for statement in &self.statements {
            match statement {
                Statement::Return(ret) => {
                    return_stms.push(statement);
                    guaranteed_return = true;
                    break;
                }
                Statement::Flow(flow) => {
                    let (returns, guaranteed) = flow.if_block.get_return_stmts();
                    return_stms.extend(returns);
                    guaranteed_return &= guaranteed;

                    if let Some(else_block) = &flow.else_block {
                        let (returns, guaranteed) = else_block.get_return_stmts();
                        return_stms.extend(returns);
                        guaranteed_return &= guaranteed;
                    }
                }
                _ => {}
            }
        }

        (return_stms, guaranteed_return)
    }
}

impl Analysis for Expression {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        match self {
            Expression::Primary(p) => p.analyze(table),
            Expression::Unary(u) => u.analyze(table),
            Expression::Binary(b) => b.analyze(table),
        }
    }
}

impl Expression {
    pub fn get_type(&self, table: &SymbolTable) -> Result<Type> {
        match self {
            Expression::Primary(p) => p.get_type(table),
            Expression::Unary(u) => u.get_type(table),
            Expression::Binary(b) => b.get_type(table),
        }
    }
}
