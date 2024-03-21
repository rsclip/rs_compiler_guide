//! Defines the symbol table and its related functions.

use crate::ast::*;
use crate::errors::SemanticError;
use crate::token::Span;

use anyhow::{anyhow, Result};

use std::collections::HashMap;

/// Represents a symbol table
#[derive(Debug)]
pub struct SymbolTable<'a> {
    /// Table for variables
    pub variables: HashMap<Ident, VarSymbol>,
    /// Table for functions
    pub functions: HashMap<Ident, FuncSymbol>,
    /// Parent
    pub parent: Option<Box<&'a SymbolTable<'a>>>,
}

/// Represents a variable symbol
#[derive(Debug)]
pub struct VarSymbol {
    /// The type of the variable
    pub ty: Type,
    /// Full span
    pub span: Span,
}

/// Represents a function symbol
#[derive(Debug)]
pub struct FuncSymbol {
    /// Parameter types
    pub params: Vec<Type>,
    /// Return type
    pub ret_ty: Type,
    /// Full
    pub span: Span,
    /// Span of the identifier
    pub ident_span: Span,
    /// Signature span
    pub sig_span: Span,
}

impl<'a> SymbolTable<'a> {
    /// Creates a new symbol table
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }
    }

    /// Creates a new symbol table with a parent
    pub fn child(parent: &'a SymbolTable) -> Self {
        SymbolTable {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Inserts a variable symbol into the table
    pub fn add_var(&mut self, var: &VariableDecl) -> Result<()> {
        if let Some(existing) = self.variables.get(&var.ident) {
            return Err(anyhow!(SemanticError::VariableAlreadyDeclared(
                var.ident.clone(),
                var.span.clone(),
                existing.span.clone()
            )));
        } else {
            self.variables.insert(
                var.ident.clone(),
                VarSymbol {
                    ty: var.ty.clone(),
                    span: var.span.clone(),
                },
            );
        }

        Ok(())
    }

    /// Insert a parameter symbol into the table
    pub fn add_param(&mut self, param: &Parameter) -> Result<()> {
        if let Some(existing) = self.variables.get(&param.ident) {
            return Err(anyhow!(SemanticError::VariableAlreadyDeclared(
                param.ident.clone(),
                param.span.clone(),
                existing.span.clone()
            )));
        } else {
            self.variables.insert(
                param.ident.clone(),
                VarSymbol {
                    ty: param.ty.clone(),
                    span: param.span.clone(),
                },
            );
        }

        Ok(())
    }

    /// Inserts a function symbol into the table
    pub fn add_fn(&mut self, func: &FunctionDecl) -> Result<()> {
        let params = func.parameters.iter().map(|p| p.ty.clone()).collect();
        let ret_ty = func.ty.clone();

        if let Some(existing) = self.functions.get(&func.ident) {
            return Err(anyhow!(SemanticError::FunctionAlreadyDeclared(
                func.ident.clone(),
                func.span.clone(),
                existing.span.clone()
            )));
        } else {
            self.functions.insert(
                func.ident.clone(),
                FuncSymbol {
                    params,
                    ret_ty,
                    span: func.span.clone(),
                    ident_span: func.ident.span.clone(),
                    sig_span: Span::combine(&func.ident.span, &func.ty.span()),
                },
            );
        }

        Ok(())
    }

    /// Looks up a variable symbol in the table
    pub fn get_var(&self, name: &Ident) -> Option<&VarSymbol> {
        self.variables.get(name)
    }

    /// Looks up a function symbol in the table
    pub fn get_fn(&self, name: &Ident) -> Option<&FuncSymbol> {
        self.functions.get(name)
    }
}
