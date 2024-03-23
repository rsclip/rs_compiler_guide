# Implementing a Symbol Table

Here, we'll implement a Symbol Table for our language.

## Symbol Entries

First, let's determine what exactly the entries will be in our symbol table. Since we'll implement type-checking and such, we'll need to store more than just the name of the symbol. Here's a list of what we'll store in each entry:
- `name`: The name of the symbol.
    - This is stored in the symbol table anyways
- `span`: The span of the symbol in the source code.
- `type`: The type of the symbol.

However, we do not want to allow people to be able to call variables as if they were a variable. Instead, we'll introduce **two** hashtables, as well as two symbol entry types: one for variables, one for functions.

A Variable Symbol would contain the type of the variable, as well as the span (as usual). Let's quickly define `VarSymbol`:

```rust,ignore
use crate::ast::*;
use crate::token::Span;

/// Represents a variable symbol
#[derive(Debug)]
pub struct VarSymbol {
    /// The type of the variable
    pub ty: Type,
    /// Full span
    pub span: Span,
}
```

Function symbols are a little more complex. We'll need to store the return type and span, as well as the parameters. Let's define `FuncSymbol`:

```rust,ignore
use crate::ast::*;
use crate::token::Span;

/// Represents a function symbol
#[derive(Debug)]
pub struct FuncSymbol {
    /// The return type of the function
    pub return_ty: Type,
    /// The parameters of the function
    pub params: Vec<(String, Type)>,
    /// Full span
    pub span: Span,
}
```

There is a little problem; the `span` field would represent the entire function declaration (including the block!). We would rather have different spans for each part of the function. For now, let's add 2 more for the identifier, and the signature (ident, params, types):

```rust,ignore
use crate::ast::*;
use crate::token::Span;

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
```

## Symbol Table
Now that we have the symbol entries, we can store the symbols in a symbol table. We'll use two hashmaps: one for variables, one for functions. We'll also use **nested scopes**, so we'll keep a reference (on the heap) to the parent scope, if any.

Let's define the `SymbolTable` struct:

```rust,ignore
use crate::ast::*;
use crate::token::Span;
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
```

Great! Let's implement some methods for the `struct` now. We'll start with methods to add and get symbols.

### Adding and Retrieving Symbols

Before we do this, let's prime on the process of adding a variable symbol to the table:

1. Retrieve a reference to the variable declaration AST node
2. Check if the variable is already declared in the current scope
    - If it is, return an error and terminate
3. Add the variable to the table
    1. Create a variable symbol from the variable declaration
    2. Insert the symbol into the table

For variable retrival, the process is less straightforward:

1. Retrieve the identifier of the variable
2. Search for the variable in the current scope
    - If it is found, return the symbol and terminate
3. If the variable is not found, search in the parent scope
    - If it is found, return the symbol and terminate
4. If the variable is not found in the current or parent scope, return **nothing**

We want to avoid returning an error if the variable isn't found; there may be contexts where it's not required to have a variable declared.

```rust,ignore
use anyhow::{anyhow, Result};
use crate::errors::SemanticError;

impl<'a> SymbolTable<'a> {
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

    /// Looks up a variable symbol in the table
    pub fn get_var(&self, name: &Ident) -> Option<&VarSymbol> {
        match self.variables.get(name) {
            Some(v) => Some(v),
            None => match &self.parent {
                Some(p) => p.get_var(name),
                None => None,
            },
        }
    }
}
```

### Adding and Retrieving Functions
Functions follow an identical to variables, although you may want to handle overloading functions here. I don't like that concept, so we won't be implementing that here.
    
```rust,ignore
impl<'a> SymbolTable<'a> {
    /// Inserts a function symbol into the table
    pub fn add_fn(&mut self, func: &FunctionDecl) -> Result<()> {
        let params = func.parameters.iter().map(|p| p.ty.clone()).collect();
        let ret_ty = func.ty.clone();

        if let Some(existing) = self.functions.get(&func.ident) {
            warn!("Function already declared: {}", func.ident.ident);
            return Err(anyhow!(SemanticError::FunctionAlreadyDeclared(
                func.ident.clone(),
                func.span.clone(),
                existing.span.clone()
            )));
        } else {
            debug!("Adding function: {}", func.ident.ident);
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

    /// Looks up a function symbol in the table
    pub fn get_fn(&self, name: &Ident) -> Option<&FuncSymbol> {
        match self.functions.get(name) {
            Some(f) => Some(f),
            None => match &self.parent {
                Some(p) => p.get_fn(name),
                None => None,
            },
        }
    }
}
```

## Conclusion
Now we have a symbol table to represent each scope, and the symbols within the scopes. We can now use this symbol table to implement type-checking and other semantic analysis.

# Resources
- [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [Memory Stack and Heap](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html)