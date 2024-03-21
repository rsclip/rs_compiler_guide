use crate::errors::SemanticError;
use crate::semantic_analysis::{Analysis, SymbolTable};
use crate::{ast::*, token::Span};
use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub parameters: Vec<Parameter>,
    pub ty: Type,
    pub block: Block,
    pub span: Span,
}

#[derive(Debug)]
pub struct Parameter {
    pub ident: Ident,
    pub ty: Type,
    pub span: Span,
}

impl PrettyPrint for FunctionDecl {
    // format: FuncDecl ident(parameters) -> ty { block }
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = format!(
            "{:indent$}FuncDecl {}({}) -> {} {{\n",
            "",
            self.ident.ident,
            self.parameters
                .iter()
                .map(|p| p.pretty_print(indent))
                .collect::<Vec<String>>()
                .join(", "),
            self.ty.pretty_print(indent),
            indent = indent * 4
        );
        s.push_str(&self.block.pretty_print(indent + 1));
        s.push_str(&format!("{:indent$}}}\n", "", indent = indent * 4));
        s
    }
}

impl PrettyPrint for Parameter {
    // format: ident: ty
    fn pretty_print(&self, _indent: usize) -> String {
        format!("{}: {}", self.ident.ident, self.ty.pretty_print(0))
    }
}

impl FunctionDecl {
    fn analyze_return_stmt(&self, table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        // multiple return statements
        let (return_values, guaranteed_return) = self.block.get_return_stmts();
        if !guaranteed_return {
            errors.push(anyhow!(SemanticError::MissingReturnStatement(
                self.span.clone()
            )));
        } else {
            for stmt in return_values {
                let expression = match stmt {
                    Statement::Return(e) => e,
                    _ => unreachable!(),
                };

                let expression = expression.as_ref().expect("Return statement without expression");

                // get type (may be an error)
                let ty: Type = match expression.get_type(table) {
                    Ok(ty) => ty,
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };

                if ty != self.ty {
                    errors.push(anyhow!(SemanticError::IncompatibleReturnType {
                        expected_type: self.ty.clone(),
                        expected_span: self.span.clone(),
                        found_type: ty,
                        found_span: stmt.span(),
                    }));
                }
            }
        }

        errors
    }

    /// Analyze parameters, and build a new symbol table for the next block
    fn analyze_parameters(&self, new_table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        for param in &self.parameters {
            if let Err(e) = new_table.add_param(param) {
                errors.push(e);
            }
        }

        errors
    }
}

impl Analysis for FunctionDecl {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        errors.extend(self.analyze_return_stmt(table));

        let mut new_table = SymbolTable::child(table);
        let param_errors = self.analyze_parameters(&mut new_table);
        errors.extend(param_errors);

        // analyze the block
        errors.extend(self.block.analyze(&mut new_table));

        errors
    }
}
