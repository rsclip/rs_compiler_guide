use crate::errors::SemanticError;
use crate::semantic_analysis::{Analysis, SymbolTable};
use crate::{ast::*, token::Span};
use anyhow::{anyhow, Error};
use log::{debug, warn};

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
        debug!("Analyzing return statements for function: {:?}", self.ident.ident);
        let mut errors = Vec::new();

        // multiple return statements
        debug!("Getting return statements for function: {:?}", self.ident.ident);
        let (return_values, guaranteed_return) = self.block.get_return_stmts(table);
        debug!("Return values: {:?}, guaranteed return: {:?}", return_values, guaranteed_return);

        if !guaranteed_return {
            warn!("Function does not have a guaranteed return statement: {:?}", self.ident.ident);
            errors.push(anyhow!(SemanticError::MissingReturnStatement(
                self.span.clone()
            )));
        } else {
            for ty in return_values {
                if ty != self.ty {
                    warn!("Incompatible return type for function: {:?}", self.ident.ident);
                    errors.push(anyhow!(SemanticError::IncompatibleReturnType {
                        expected_type: self.ty.clone(),
                        expected_span: self.ty.span(),
                        found_type: ty.clone(),
                        found_span: ty.span(),
                    }));
                }
            }
        }

        debug!("Return statement analysis errors: {:?}", errors);

        errors
    }

    /// Analyze parameters, and build a new symbol table for the next block
    fn analyze_parameters(&self, new_table: &mut SymbolTable) -> Vec<Error> {
        debug!("Analyzing parameters for function: {:?}", self.ident.ident);
        let mut errors = Vec::new();

        for param in &self.parameters {
            if let Err(e) = new_table.add_param(param) {
                errors.push(e);
            }
        }

        debug!("Parameter analysis errors: {:?}", errors);

        errors
    }
}

impl Analysis for FunctionDecl {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        debug!("Analyzing function declaration: {:?}", self.ident.ident);
        let mut errors = Vec::new();

        errors.extend(self.analyze_return_stmt(table));

        let mut new_table = SymbolTable::child(table);
        let param_errors = self.analyze_parameters(&mut new_table);
        errors.extend(param_errors);

        // analyze the block
        errors.extend(self.block.analyze(&mut new_table));

        debug!("Function declaration analysis errors: {:?}", errors);

        errors
    }
}
