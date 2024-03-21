use crate::errors::SemanticError;
use crate::semantic_analysis::{Analysis, SymbolTable};
use crate::{ast::*, token::Span};
use anyhow::{anyhow, Error, Result};
use log::{debug, warn};

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
        debug!("Analyzing block: {self:?}, table: {table:?}");
        let mut errors = Vec::new();
        let mut new_table = SymbolTable::child(table);

        for statement in &self.statements {
            debug!("Analyzing statement: {statement:?}");
            errors.extend(statement.analyze(&mut new_table));
        }

        debug!("Block analysis complete, errors: {errors:?}");

        errors
    }
}

impl Block {
    /// Explore for all return statements
    /// Return:
    /// 1. The return value **types**
    /// 2. Whether or not it guarantees a return regardless of the path
    /// 
    /// Invalid return statements will be ignored
    pub fn get_return_stmts(&self, table: &mut SymbolTable) -> (Vec<Type>, bool) {
        let mut return_stmts_types = Vec::new();
        let mut guaranteed_return = false;
        let mut tmp_my_table = SymbolTable::child(table);

        for statement in &self.statements {
            match statement {
                Statement::Return(stmt) => {
                    if let Some(expr) = stmt {
                        if let Ok(ty) = expr.get_type(&tmp_my_table) {
                            return_stmts_types.push(ty.clone());
                        }
                    }
                    guaranteed_return = true;
                    break;
                }
                Statement::VariableDecl(v) => {
                    if let Err(e) = tmp_my_table.add_var(v) {
                        warn!("Error adding variable to table: {:?}, {:?}", statement, e);
                    }
                },
                Statement::Flow(flow) => {
                    let (returns, guaranteed) = flow.if_block.get_return_stmts(&mut tmp_my_table);
                    return_stmts_types.extend(returns);
                    guaranteed_return &= guaranteed;

                    if let Some(else_block) = &flow.else_block {
                        let (returns, guaranteed) = else_block.get_return_stmts(&mut tmp_my_table);
                        return_stmts_types.extend(returns);
                        guaranteed_return &= guaranteed;
                    }
                }
                _ => {}
            }
        }

        (return_stmts_types, guaranteed_return)
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
