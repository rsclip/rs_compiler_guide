//! This module implements Semantic Analysis on an existing AST.
//! 
//! Errors:
//! - Scope checking                Idents must be declared before use
//! - Type checking                 Return types, etc.
//! - Variable redeclaration         Variables cannot be redeclared
//! - Function redeclaration        Functions cannot be redeclared
//! - Control flow checks           Return statements, etc. breaks cannot be outside loops
//! - Missing main function
//! 
//! Warnings:
//! - Dead code (unused anything)
//! - Unreachable code

use super::symbols::SymbolTable;
use anyhow::{Error, Result};
use crate::ast::*;

pub fn analyse<'a>(ast: &'a AST) -> Vec<Error> {
    let program = &ast.program;

    let mut global_table = SymbolTable::new();
    let mut errors = Vec::new();

    // recognise all functions
    for item in &program.items {
        match item {
            Item::FunctionDecl(f) => {
                let name = &f.ident;
                if global_table.contains(name) {
                    // error
                } else {
                    global_table.insert(name.clone(), f.clone());
                }
            }
        }
    }

    errors
}