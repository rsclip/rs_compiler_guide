use std::collections::HashMap;

use crate::errors::Warning;
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

        errors.extend(self.check_dead_unreachable(&table).0);

        debug!("Block analysis complete, errors: {errors:?}");

        errors
    }
}

impl Block {
    /// Check for dead and unreachable code
    /// Returns errors and map of declared variables
    fn check_dead_unreachable(&self, table: &SymbolTable) -> (Vec<Error>, HashMap<Ident, bool>) {
        debug!("Checking for dead and unreachable in block: {self:?}, table: {table:?}");
        let mut errors = Vec::new();
        let mut tmp_my_table = SymbolTable::child(table);

        // map of variables declared in this scope, and whether they are used
        let mut declared_vars: HashMap<Ident, bool> = HashMap::new();

        let mut early_return = false;
        for (cur_idx, statement) in self.statements.iter().enumerate() {
            match statement {
                Statement::Return(expr) => {
                    if early_return {
                        continue;
                    }

                    early_return = cur_idx + 1 != self.statements.len();
                    let idents_used = expr.as_ref().map(|e| e.idents_used()).unwrap_or_default();

                    for ident in &idents_used {
                        if let Some(declared) = declared_vars.get_mut(&ident) {
                            *declared = true;
                        }
                    }

                    debug!("Found return statement at {cur_idx}, early_return: {early_return}, idents_used: {idents_used:?}, stmt: {statement:?}");
                }
                Statement::VariableDecl(v) => {
                    if early_return {
                        continue;
                    }

                    if let Err(e) = tmp_my_table.add_var(v) {
                        warn!("Error adding variable to table: {:?}, {:?}", statement, e);
                    }
                    declared_vars.insert(v.ident.clone(), false);

                    // expression may use the variable
                    for ident in &v.expression.idents_used() {
                        if let Some(declared) = declared_vars.get_mut(&ident) {
                            *declared = true;
                        }
                    }
                }
                Statement::Flow(flow) => {
                    if early_return {
                        continue;
                    }

                    let (_, if_guaranteed) = flow.if_block.get_return_stmts(&mut tmp_my_table);

                    // block may use a variable in this scope
                    for (ident, used) in &flow.if_block.check_dead_unreachable(&tmp_my_table).1 {
                        if let Some(declared) = declared_vars.get_mut(&ident) {
                            *declared |= *used;
                        }
                    }

                    if let Some(else_block) = &flow.else_block {
                        let (_, else_guaranteed) = else_block.get_return_stmts(&mut tmp_my_table);
                        for (ident, used) in &else_block.check_dead_unreachable(&tmp_my_table).1 {
                            if let Some(declared) = declared_vars.get_mut(&ident) {
                                *declared |= *used;
                            }
                        }

                        // in the case where both blocks have a return statement
                        // we can guarantee a return
                        if if_guaranteed && else_guaranteed {
                            early_return = true;
                            debug!("Found early return");
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        if early_return {
            // get span of unreachable code
            let span = Span::combine(
                &self
                    .statements
                    .first()
                    .map(|s| s.span())
                    .unwrap_or(self.span.clone()),
                &self
                    .statements
                    .last()
                    .map(|s| s.span())
                    .unwrap_or(self.span.clone()),
            );

            errors.push(anyhow!(Warning::UnreachableCode(span)));
        };

        for (ident, declared) in &declared_vars {
            if !declared && !ident.ident.starts_with("_") {
                warn!("Unused variable: {:?}", ident);
                errors.push(anyhow!(Warning::UnusedVariable(
                    ident.clone(),
                    ident.span.clone()
                )));
            }
        }

        debug!("Dead and unreachable check complete, errors: {errors:?}");

        (errors, declared_vars)
    }

    /// Explore for all return statements
    /// Return:
    /// 1. The return value **types**
    /// 2. Whether or not it guarantees a return regardless of the path
    ///
    /// Invalid return statements will be ignored
    pub fn get_return_stmts(&self, table: &mut SymbolTable) -> (Vec<Type>, bool) {
        let mut return_stmts_types = Vec::new();
        let mut guaranteed_return = true;
        let mut tmp_my_table = SymbolTable::child(table);
        let mut early_return = false;

        for statement in &self.statements {
            match statement {
                Statement::Return(stmt) => {
                    debug!("Getting return statement type, table: {table:?}");
                    if let Some(expr) = stmt {
                        if let Ok(ty) = expr.get_type(&tmp_my_table) {
                            return_stmts_types.push(ty.clone());
                        }
                    }
                    debug!("Found return statement: {:?}", statement);
                    guaranteed_return = true;
                    early_return = true;
                    break;
                }
                Statement::VariableDecl(v) => {
                    if let Err(e) = tmp_my_table.add_var(v) {
                        warn!("Error adding variable to table: {:?}, {:?}", statement, e);
                    }
                }
                Statement::Flow(flow) => {
                    let (returns, if_guaranteed) =
                        flow.if_block.get_return_stmts(&mut tmp_my_table);
                    debug!(
                        "If block return values (guaranteed {if_guaranteed}): {:?}",
                        &returns
                    );
                    return_stmts_types.extend(returns);
                    guaranteed_return &= if_guaranteed;

                    if let Some(else_block) = &flow.else_block {
                        let (returns, else_guaranteed) =
                            else_block.get_return_stmts(&mut tmp_my_table);
                        debug!(
                            "Else block return values (guaranteed {else_guaranteed}): {:?}",
                            &returns
                        );
                        return_stmts_types.extend(returns);
                        guaranteed_return &= else_guaranteed;

                        // in the case where both blocks have a return statement
                        // we can guarantee a return
                        if if_guaranteed && else_guaranteed {
                            early_return = true;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        (return_stmts_types, guaranteed_return && early_return)
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

    pub fn idents_used(&self) -> Vec<Ident> {
        match self {
            Expression::Primary(p) => p.idents_used(),
            Expression::Unary(u) => u.idents_used(),
            Expression::Binary(b) => b.idents_used(),
        }
    }
}
