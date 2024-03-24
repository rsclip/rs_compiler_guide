# Implementing a Semantic Analyzer

Now that we have a Symbol Table struct, we'll be able to implement a semantic analyzer. This process is pretty difficult and can be challenging to debug, but try not to get discouraged! It's a very rewarding process.

## Overview
Our parser was quite limited in retrospect; it would terminate and fail to perform error recovery. This is intended, as it's much more simple to implement a parser that fails on the first error. However, this is not the case for a semantic analyzer. We want to collect as many errors as possible and report them all at once.

Let's implement some sort of analysis algorithm which will aggregate all errors found in the program. We'll start by creating a new module `analysis` in `src/semantic_analysis/` if you haven't already.

## `Analysis` trait

Again, instead of diving in, let's outline exactly how we want the process to look like. We could have functions (e.g. `analyze_fn`, `analyze_block`, etc.) that take in the Symbol Table and the AST node to analyze. This way, we can easily traverse the AST and check for errors.

In order to make the code cleaner, however, it would be more idiomatic to introduce a new **trait** that would allow us to call `analyze` on any AST node. This way, we can easily call `analyze` on any node and have the node analyze itself.

Let's declare the trait in `src/semantic_analysis/traits.rs`:
    
```rust,ignore
use super::symbols::SymbolTable;

use anyhow::Error;

/// Trait for Semantic Analysis
///
/// When implementing, you are expected to analyse
/// every part of the node, aggregating and returning
/// *all* errors found.
pub trait Analysis {
    /// Analyze the node
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error>;
}
```

We take in the current symbol table to represent the current scope. We'll also return a vector of `Error`s, which will contain all errors found in the node.

## The main analysis function

Before we implement `Analysis` on each AST node, let's make an interface for us to perform analysis on an entire program.

Within this function, we'll first create a new `SymbolTable` to represent the global scope. We can then call `analyze` on the AST itself, passing in the global scope.

In `src/semantic_analysis/analysis.rs`:
```rust,ignore
use super::symbols::SymbolTable;
use super::traits::Analysis;
use crate::ast::*;
use crate::errors::SemanticError;
use crate::token::Span;
use anyhow::{anyhow, Error};

pub fn analyse<'a>(ast: &'a AST) -> Vec<Error> {
    let program = &ast.program;

    let mut global_table = SymbolTable::new();
    let mut errors = Vec::new();

    // let AST analyse itself
    errors.extend(ast.analyze(&mut global_table));

    errors
}
```

There is a fundamental flaw in this design; it does not analyze the functions at all, so the symbol table initializes as empty. We can implement this either in the AST node, or we can do that here.

Since it is a little different than how our typical `impl Analaysis for ...` would look like, let's implement it here.

We'll recognise all functions by iterating through all `Item`s in the program, registering them within the symbol table.

The presence of the `main` function is also required, so we'll flag an error if it is not found. We also would like this to return `int`, which would represent the exit code of the program, akin to C++.

Let's implement this:

```rust,ignore
    // ...
    let mut errors = Vec::new();

    // recognise all functions
    let mut main_node: Option<&FunctionDecl> = None;

    for item in &program.items {
        match item {
            Item::FunctionDecl(f) => {
                if f.ident.ident == "main" {
                    main_node = Some(f);
                }

                if let Err(e) = global_table.add_fn(f) {
                    errors.push(e);
                }
            }
        }
    }

    if main_node.is_none() {
        errors.push(anyhow!(SemanticError::MissingMainFunction(Span::default())));
    } else {
        // ensure return type is int
        let ret_ty = main_node.unwrap().ty.clone();
        match ret_ty {
            Type::Primitive(ty) => {
                if ty.kind != PrimitiveKind::Int {
                    errors.push(anyhow!(SemanticError::MainMustReturnInt(ty.span.clone())));
                }
            },
        }
    }

    // let AST analyse itself
    errors.extend(ast.analyze(&mut global_table));

    // ...
```

## Implementing our `Analysis` trait
Here comes the difficult and long part, we picked this hell for ourselves.

### Implementing `Analysis` for `AST` and `Item`
This is thankfully pretty simple, we just need to go through each item in `AST` and call `analyze` on them. For each item, we need to match what kind of item it is and call `analyze` on it.

```rust,ignore
impl Analysis for AST {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        for item in &self.program.items {
            errors.extend(item.analyze(table));
        }

        errors
    }
}

impl Analysis for Item {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        match self {
            Item::FunctionDecl(f) => f.analyze(table),
        }
    }
}
```

As tempting as it may be to implement `Analysis` for `FunctionDecl` next, let's continue going in order.

## Implementing `Analysis` for `Block`
When analysing a `Block`, it will introduce a new **scope**. To make things simple, when we analyze a function it will introduce a scope for the parameters, and the block will introduce its own scope.

Within a block, we'd simply analyze each statement for errors.

```rust,ignore
impl Analysis for Block {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();
        let mut new_table = SymbolTable::child(table);

        for statement in &self.statements {
            debug!("Analyzing statement: {statement:?}");
            errors.extend(statement.analyze(&mut new_table));
        }

        errors
    }
}
```

This analyzes each statement within the block. There's also two other problems we would like to solve:
12. If the block is a function body, we need a way to
    1. See if the return for the block is guaranteed or not, taking into account all possible paths
    2. Get all the types for these return expressions
2. A block may have **dead code** (unreachable code), and **unused variables**

We'll ignore determining whether conditions are guaranteed or not.

> **Performance Warning**
> 
> This is a very naive implementation, and will be very slow. Try to introduce caching and memoization to speed up the process.

We'll solve the first problem by introducing a method; `get_return_stmts`. This will check if the block is guaranteed to return, and get all the return types.

The process will follow:
1. Iterate through all the statements. If the statement is:
    - **Return**
        1. Mark as guaranteed return, and early return
        2. Add expression type
    - **Variable Declaration**
        1. Add variable to the symbol table (may be referenced later)
    - **Flow statement** (`if`)
        1. Get the return statement expressions, and whether the block is independently guaranteed to return
        2. Add expression types
        3. `guaranteed_return *= if_block_guaranteed_return` (if block is guaranteed to return, then the entire block is guaranteed to return)
        4. If there is an `else` block:
            1. Do the same process as above
            2. If both blocks are guaranteed, then the entire block is guaranteed. Set the early return flag to true.
2. Return the statement types, and whether the block is guaranteed to return

Let's implement this helper method:

```rust,ignore
impl Block {
    pub fn get_return_stmts(&self, table: &mut SymbolTable) -> (Vec<Type>, bool) {
        let mut return_stmts_types = Vec::new();
        let mut guaranteed_return = true;
        let mut tmp_my_table = SymbolTable::child(table);
        let mut early_return = false;

        for statement in &self.statements {
            match statement {
                Statement::Return(stmt) => {
                    if let Some(expr) = stmt {
                        if let Ok(ty) = expr.get_type(&tmp_my_table) {
                            return_stmts_types.push(ty.clone());
                        }
                    }
                    guaranteed_return = true;
                    early_return = true;
                    break;
                }
                Statement::VariableDecl(v) => {
                    // recover error, add your own debugging stuff
                    if let Err(e) = tmp_my_table.add_var(v) {}
                }
                Statement::Flow(flow) => {
                    let (returns, if_guaranteed) = flow.if_block.get_return_stmts(&mut tmp_my_table);

                    return_stmts_types.extend(returns);
                    guaranteed_return &= if_guaranteed;

                    if let Some(else_block) = &flow.else_block {
                        let (returns, else_guaranteed) = else_block.get_return_stmts(&mut tmp_my_table);

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
```

Next, we need to introduce a method to check for dead code and unused variables: `check_dead_unreachable`.

Here, we'll want to check for dead and unreachable code, aggregating the errors and warnings we'll find. We would also like to check for which variables are declared.

The process is as follows:
1. Set the `early_return` flag to `false`. We need this to check for unreachable code.
2. Iterate through all the statements. If the statement is:
    - **Return**
        1. If `early_return` is true, then skip the statement. It will never execute.
        2. Check whether it's an early return
        3. Set used variables in expression as used
    - **Variable Declaration**
        1. If `early_return` is true, then skip the statement. It will never execute.
        2. Add the variable to the symbol table. Ignore if error.
        3. Set used variables in expression as used
    - **Flow statement** (`if`)
        1. If `early_return` is true, then skip the statement. It will never execute.
        2. Get whether the block is guaranteed to return
        3. Set used variables in block as used
        4. If there's an `else` block:
            1. Get whether the block is guaranteed to return
            2. Set used variables in block as used
            4. If both blocks are guaranteed to return, then set `early_return` to true
3. If there's an early return:
    1. Raise a warning for the section of dead code following the return statement
4. For each undeclared variable, raise an error
5. Return all errors/warnings, and the variables declared

Let's implement this helper method:

```rust,ignore
impl Block {
    fn check_dead_unreachable(&self, table: &SymbolTable) -> (Vec<Error>, HashMap<Ident, bool>) {
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
                }
                Statement::VariableDecl(v) => {
                    if early_return {
                        continue;
                    }

                    if let Err(e) = tmp_my_table.add_var(v) {}
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
                errors.push(anyhow!(Warning::UnusedVariable(
                    ident.clone(),
                    ident.span.clone()
                )));
            }
        }

        (errors, declared_vars)
    }
}
```

Be sure to add this to the `Analysis` implementation:
    
```rust,ignore
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
```

We do **not** check for guaranteed return here, since we don't have enough context of the block (it may be a flow block, not a function body). The function body will handle this.

## Implementing `Analysis` for `Expression`
Since this is an `enum`, we can call `analyze` on each variant.

```rust,ignore
impl Analysis for Expression {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        match self {
            Expression::Primary(p) => p.analyze(table),
            Expression::Unary(u) => u.analyze(table),
            Expression::Binary(b) => b.analyze(table),
        }
    }
}
```

However, we have some methods we may need to use.

We called `get_type` to evaluate what the expression may be. We also called `idents_used` to get all the identifiers used in the expression. We'll need to implement these methods.

```rust,ignore
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
```

## Implementing `Analysis` for `Statement`
This is similar to `Expression`, we'll call `analyze` on each variant. The return statements variant contains a reference to an expression, so we'll need to call `analyze` on that.

```rust,ignore
impl Analysis for Statement {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        match self {
            Statement::Expression(e) => e.analyze(table),
            Statement::VariableDecl(v) => v.analyze(table),
            Statement::Flow(f) => f.analyze(table),
            Statement::Return(e) => e.as_ref().map_or_else(|| vec![], |e| e.analyze(table)),
        }
    }
}
```

## Implementing `Analysis` for `VariableDecl`
When analyzing a variable declaration, we'd get given the current scope. We would like to declare the current variable to the symbol table. We'd also like to check whether the expression is the same as the variable type.

Let's implement:
```rust,ignore
impl Analysis for VariableDecl {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        // add variable to symbol table
        match table.add_var(&self) {
            Ok(_) => (),
            Err(e) => errors.push(e),
        };

        // check if expression type matches variable type
        match self.expression.get_type(table) {
            Ok(ty) => {
                if ty != self.ty {
                    errors.push(anyhow!(SemanticError::TypesDoNotMatch {
                        expected_type: self.ty.clone(),
                        expected_span: self.ty.span(),
                        found_type: ty,
                        found_span: self.expression.span(),
                    }));
                }
            }
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        errors
    }
}
```

## Implementing `Analysis` for `FlowStatement`
To analyze the flow statement, we'd need to check a few things. The structure is a little complex, so let's look at a typical flow statement and identify what we need to determine:

```rust,ignore
if a == 1 {
    return 1;
} else {
    return 2;
}
```

1. We need to check the condition expression. This should be a boolean.
2. We need to check the if block. Pretty simple.
3. If there's an else block, we need to check that too.

Let's implement this:
```rust,ignore

impl Analysis for FlowStatement {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        // check if condition is a boolean
        match self.condition.get_type(table) {
            Ok(ty) => {
                let bool_type = Type::Primitive(PrimitiveType {
                    kind: PrimitiveKind::Bool,
                    span: Span::default(),
                });

                if ty != bool_type {
                    errors.push(anyhow!(SemanticError::NonBooleanCondition {
                        found_type: ty,
                        found_span: self.condition.span(),
                    }));
                }
            }
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        errors.extend(self.if_block.analyze(table));

        if let Some(else_block) = &self.else_block {
            errors.extend(else_block.analyze(table));
        }

        errors
    }
}
```

## Implementing `Analysis` for `FunctionDecl`
When analyzing a function declaration, we'd need to do a few things:

1. Make sure all return statements match the function type
2. Make sure the function is guaranteed to return
3. Analyze each parameter
4. Analyze the block

Let's implement the basic analysis function:
```rust,ignore
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
```

We used a few functions we haven't made yet. Let's implement them.

The `analyze_return_stmt` function will simply analyze the first two criteria mentioned:
1. Make sure all return statements match the function type
2. Make sure the function is guaranteed to return

This would look at the block, and get the return statements. If there are no return statements, or the return statements do not match the function type, we'll raise an error.

Thankfully, we defined the `get_return_stmts` function in `Block` earlier; we can use that to our advantage.

```rust,ignore
impl FunctionDecl {
    fn analyze_return_stmt(&self, table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        // multiple return statements
        let (return_values, guaranteed_return) = self.block.get_return_stmts(table);

        if !guaranteed_return {
            if return_values.len() == 0 {
                errors.push(anyhow!(SemanticError::MissingReturnStatement(
                    self.span.clone()
                )));
            } else {
                errors.push(anyhow!(SemanticError::ReturnNotGuaranteed(
                    self.span.clone()
                )));
            }
        } else {
            for ty in return_values {
                if ty != self.ty {
                    errors.push(anyhow!(SemanticError::IncompatibleReturnType {
                        expected_type: self.ty.clone(),
                        expected_span: self.ty.span(),
                        found_type: ty.clone(),
                        found_span: ty.span(),
                    }));
                }
            }
        }

        errors
    }
}
```

Next, we need to implement the `analyze_parameters` function.

We'll receive the current parameter block, and we'll add each parameter to the symbol table. There's no need to check for duplicate parameters, as the symbol table will handle that.

```rust,ignore
impl FunctionDecl {
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
```

Very simple!

## Implementing `Analysis` for `UnaryExpression`
Let's do this before Binary Expressions, since they involve a little extra work.

When analyzing a unary expression, we'd need to check a few things:
1. The operator must be valid for the type (i.e. we can't call `!` on an integer)
2. The expression must be valid

Let's quickly write the implementation for not (`!`). We would only like the expression to be a boolean.

```rust,ignore
impl Analysis for UnaryExpression {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        match &self.kind {
            UnaryExpressionKind::Negation(_) => self.analyze_negation(table),
            UnaryExpressionKind::Not(_) => self.analyze_not(table),
        }
    }
}

impl UnaryExpression {
    fn analyze_not(&self, table: &SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        let expr: &Box<Expression> = match &self.kind {
            UnaryExpressionKind::Not(e) => e,
            _ => unreachable!(),
        };

        // check if unsupported type
        let expr_type = match expr.get_type(table) {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        match expr_type {
            Type::Primitive(ref prim_ty) => match prim_ty.kind {
                PrimitiveKind::Bool => (),
                _ => errors.push(anyhow!(SemanticError::UnsupportedUnaryOperation {
                    operator: "Not".to_string(),
                    operand_type: expr_type.clone(),
                    span: self.span.clone(),
                })),
            },
        }

        errors
    }
}
```

Let's do the same for negation (`-`). We'd only want the expression to be an integer or float strictly. This follows the same code structure as the `!` operator.

```rust,ignore
impl UnaryExpression {
    fn analyze_negation(&self, table: &SymbolTable) -> Vec<Error> {
        // ...

        match expr_type {
            Type::Primitive(ref prim_ty) => match prim_ty.kind {
                PrimitiveKind::Int | PrimitiveKind::Float => (),
                _ => errors.push(anyhow!(SemanticError::UnsupportedUnaryOperation {
                    operator: "Negation".to_string(),
                    operand_type: expr_type.clone(),
                    span: self.span.clone(),
                })),
            },
        }

        errors
    }
}
```

Recall that we declared `get_type` and `idents_used` for the expression, which would propagate onto this struct. We'll pretty much redirect them back to the expression, which would call the same function on the inner expression.

```rust,ignore
impl UnaryExpression {
    pub fn get_type(&self, table: &SymbolTable) -> Result<Type> {
        let expr = match &self.kind {
            UnaryExpressionKind::Negation(e) => e,
            UnaryExpressionKind::Not(e) => e,
        };

        expr.get_type(table)
    }

    pub fn idents_used(&self) -> Vec<Ident> {
        match &self.kind {
            UnaryExpressionKind::Negation(e) => e.idents_used(),
            UnaryExpressionKind::Not(e) => e.idents_used(),
        }
    }
}
```

## Implementing `Analysis` for `BinaryExpression`
A binary expression involves two operands. We'd need to check whether both the left hand size `lhs` and right hand size `lhs` are the same type (we do not want implicit type coercion here). We'd also like to make sure they're valid types to be added together.

Nothing, too extravagant, let's implement:
```rust,ignore
impl Analysis for BinaryExpression {
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error> {
        // validate both sides are same type
        let mut errors = Vec::new();

        let lhs_type = match self.lhs.get_type(table) {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        let rhs_type = match self.rhs.get_type(table) {
            Ok(t) => t,
            Err(e) => {
                errors.push(e);
                return errors;
            }
        };

        if lhs_type != rhs_type {
            errors.push(anyhow!(SemanticError::TypesDoNotMatch {
                expected_type: lhs_type.clone(),
                expected_span: self.lhs.span(),
                found_type: rhs_type.clone(),
                found_span: self.rhs.span(),
            }));
        };

        // check if either side is not a valid type
        if !self.is_valid_type(&lhs_type) {
            errors.push(anyhow!(SemanticError::UnsupportedBinaryOperation {
                operator: self.op.kind.to_string(),
                operand_type: lhs_type,
                span: self.span.clone(),
            }));
        }

        if !self.is_valid_type(&rhs_type) {
            errors.push(anyhow!(SemanticError::UnsupportedBinaryOperation {
                operator: self.op.kind.to_string(),
                operand_type: rhs_type,
                span: self.span.clone(),
            }));
        }

        errors
    }
}

impl BinaryExpression {
    fn is_valid_type(&self, ty: &Type) -> bool {
        match ty {
            Type::Primitive(ref prim_ty) => match prim_ty.kind {
                PrimitiveKind::Int | PrimitiveKind::Float | PrimitiveKind::Bool => true,
            },
        }
    }
}
```

Similar to unary expressions, we'll need to implement `get_type` and `idents_used` for binary expressions. We'll redirect these functions to the inner expressions.

```rust,ignore
impl BinaryExpression {
    pub fn get_type(&self, table: &SymbolTable) -> Result<Type> {
        let lhs_type = self.lhs.get_type(table)?;
        let rhs_type = self.rhs.get_type(table)?;

        if lhs_type != rhs_type {
            return Err(anyhow!(SemanticError::TypesDoNotMatch {
                expected_type: lhs_type,
                expected_span: self.lhs.span(),
                found_type: rhs_type,
                found_span: self.rhs.span(),
            }));
        }

        // if comparison, return bool
        match self.op.kind {
            BinaryOperatorKind::Equal
            | BinaryOperatorKind::NotEqual
            | BinaryOperatorKind::LessThan
            | BinaryOperatorKind::LessThanOrEqual
            | BinaryOperatorKind::GreaterThan
            | BinaryOperatorKind::GreaterThanOrEqual => Ok(Type::Primitive(PrimitiveType {
                kind: PrimitiveKind::Bool,
                span: self.span.clone(),
            })),
            _ => Ok(lhs_type),
        }
    }

    pub fn idents_used(&self) -> Vec<Ident> {
        let mut idents = self.lhs.idents_used();
        idents.extend(self.rhs.idents_used());
        idents
    }
}
```

## Implementing `Analysis` for `PrimaryExpression`
Since this is an enum, we simply call `analyze` on each variant. However, the function call variant is a little more complex. We'll define and create `analyze_fn_call` for this.

```rust,ignore
impl Analysis for PrimaryExpression {
    fn analyze(&self, _table: &mut SymbolTable) -> Vec<Error> {
        match self {
            PrimaryExpression::Literal(_) => vec![],
            PrimaryExpression::Ident(_) => vec![],
            PrimaryExpression::Parenthesized(p) => p.analyze(_table),
            PrimaryExpression::FunctionCall(_, _) => self.analyze_fn_call(_table),
        }
    }
}
```

When analyzing a function call, we need to ensure that:
1. The function is declared
2. The number of arguments passed match
3. The types of the arguments match the function parameters

This is pretty much the process of the entire function, so we can just implement. We need to unwrap the primary expression, look for the function in the symbol table, and check the arguments.

```rust,ignore
impl PrimaryExpression {
    fn analyze_fn_call(&self, table: &mut SymbolTable) -> Vec<Error> {
        let mut errors = Vec::new();

        let (ident, args) = match self {
            PrimaryExpression::FunctionCall(i, a) => (i, a),
            _ => unreachable!("analyze_fn_call called on non-FunctionCall variant"),
        };

        // can we find the function in the symbol table?
        if let Some(func) = table.get_fn(&ident) {
            // check if the number of arguments match
            if func.params.len() != args.len() {
                // get the span of arguments supplied
                let call_span = Span::combine(
                    &args.first().map(|a| a.span()).unwrap_or(ident.span.clone()),
                    &args.last().map(|a| a.span()).unwrap_or(ident.span.clone()),
                );

                errors.push(anyhow!(SemanticError::ArgumentCountMismatch {
                    expected: func.params.len(),
                    found: args.len(),
                    call_span,
                    decl_span: func.sig_span.clone(),
                }));
            } else {
                // check if the types of the arguments match
                for (param_ty, arg) in func.params.iter().zip(args) {
                    let arg_ty: Type = match arg.get_type(table) {
                        Ok(ty) => ty,
                        Err(e) => {
                            errors.push(e);
                            continue;
                        }
                    };

                    if *param_ty != arg_ty {
                        errors.push(anyhow!(SemanticError::TypesDoNotMatch {
                            expected_type: param_ty.clone(),
                            expected_span: param_ty.span(),
                            found_type: arg_ty,
                            found_span: arg.span(),
                        }));
                    }
                }
            }
        } else {
            errors.push(anyhow!(SemanticError::FunctionNotDeclared(
                ident.clone(),
                ident.span.clone()
            )));
        }

        errors
    }
}
```

Since `Expression` propagated its `get_type` and `idents_used` functions through to us, we need to implement this here. Since this is a primary expression, we'll actually handle it this time.

It depends on what kind of primary expression it is, however:
- **Literal**: Call the `get_type` function on the literal
- **Ident**: Get the type of the identifier from the symbol table
- **Parenthesized**: Call the `get_type` function on the expression
- **FunctionCall**: Return the function return type, if it exists

```rust,ignore
impl PrimaryExpression {
    pub fn get_type(&self, table: &SymbolTable) -> Result<Type> {
        match self {
            PrimaryExpression::Literal(l) => Ok(l.get_type()),
            PrimaryExpression::Ident(i) => {
                if let Some(var) = table.get_var(&i) {
                    Ok(var.ty.clone())
                } else {
                    Err(anyhow!(SemanticError::VariableNotDeclared(
                        i.clone(),
                        i.span.clone()
                    )))
                }
            }
            PrimaryExpression::Parenthesized(p) => p.get_type(table),
            PrimaryExpression::FunctionCall(i, _) => {
                if let Some(func) = table.get_fn(&i) {
                    Ok(func.ret_ty.clone())
                } else {
                    Err(anyhow!(SemanticError::FunctionNotDeclared(
                        i.clone(),
                        i.span.clone()
                    )))
                }
            }
        }
    }
}
```

For idents used it, too, depends on the variant:
- **Literal**: No identifiers used
- **Ident**: Return the identifier
- **Parenthesized**: Call the `idents_used` function on the expression
- **FunctionCall**: Return the identifiers used in the arguments

```rust,ignore
impl PrimaryExpression {
    pub fn idents_used(&self) -> Vec<Ident> {
        match self {
            PrimaryExpression::Literal(_) => vec![],
            PrimaryExpression::Ident(i) => vec![i.clone()],
            PrimaryExpression::Parenthesized(p) => p.idents_used(),
            PrimaryExpression::FunctionCall(_, args) => {
                args.iter().flat_map(|a| a.idents_used()).collect()
            }
        }
    }
}
```

## Testing
It's important to test your code. Let's look at some code examples and run tests on them. We'll just go through one test.

```
fn main() -> int {
    return 5;
}

fn foo() -> bool {
    return 5;
}
```

This gives us the error:
```
Error: Incompatible return type
   ╭─[test:5:9]
   │
 4 │ fn foo() -> bool {
   │             ──┬─  
   │               ╰─── expected bool return type
 5 │     return 5;
   │            ┬  
   │            ╰── found int instead of bool
───╯
```

This recognised that not only is `foo` returning an integer rather than `bool`, it knows that the `main` function exists and returns an integer. Nice.