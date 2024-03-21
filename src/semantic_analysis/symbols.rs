//! Defines the symbol table and its related functions.

use crate::ast::*;

use anyhow::{anyhow, Result};

use std::collections::HashMap;

/// Represents a symbol table
#[derive(Debug)]
pub struct SymbolTable {
    /// Table for variables
    pub variables: HashMap<Ident, VarSymbol>,
    /// Table for functions
    pub functions: HashMap<Ident, FuncSymbol>,
}

/// Represents a variable symbol
#[derive(Debug)]
pub struct VarSymbol {
    /// The type of the variable
    pub ty: Type,
}

/// Represents a function symbol
#[derive(Debug)]
pub struct FuncSymbol {
    /// Parameter types
    pub params: Vec<Type>,
    /// Return type
    pub ret_ty: Type,
}

impl SymbolTable {
    /// Creates a new symbol table
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    /// Inserts a variable symbol into the table
    pub fn add_var(&mut self, name: Ident, ty: Type) -> Result<()> {
        self.variables.insert(name, VarSymbol { ty });

        Ok(())
    }

    /// Inserts a function symbol into the table
    pub fn add_fn(&mut self, func: &FunctionDecl) -> Result<()> {
        let params = func.parameters.iter().map(|p| p.ty.clone()).collect();
        let ret_ty = func.ty.clone();
        self.functions
            .insert(func.ident.clone(), FuncSymbol { params, ret_ty });

        Ok(())
    }

    /// Looks up a variable symbol in the table
    pub fn has_var(&self, name: &Ident) -> Option<&VarSymbol> {
        self.variables.get(name)
    }

    /// Looks up a function symbol in the table
    pub fn has_fn(&self, name: &Ident) -> Option<&FuncSymbol> {
        self.functions.get(name)
    }
}
