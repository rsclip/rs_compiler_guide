# Implementing a Parser
So far, we have a lexer and a way to represent tokens. Next, we need to implement the parser.

We'll be using [**Recursive Descent Parsing** (RDP)](./recursive_descent_parsing.md) to implement the parser.

## The `Parser` struct
Let's define a struct for our parser. It'll own the stream of `Token`s, and it should keep track of the position in the stream.

> **Note**
>
> Instead of owning the `Vec<Token>`, we could use a reference or better, an iterator. However, for simplicity, we'll use `Vec<Token>`.

### Safe memory management
We need to be careful in designing here, since while the stream of tokens may be immutable, the position is *not*. Let's implement a basic `struct` and see why it may be a problem:

```rust,ignore
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    // ...

    // Expect a token of a certain type
    fn expect(&mut self, kind: TokenKind) -> Result<&Token> {
        // Get the current token
        let current: &Token = self.current()?;      // Immutable borrow

        // Move to the next token
        self.position += 1;                         // Mutable borrow

        if current.kind == kind {
            Ok(current)
        } else {
            Err("Unexpected token")
        }
    }
}
```
**Assume the other functions are implemented*

Here, we have a problem. We're trying to borrow `self` mutably and immutably at the same time (as labelled in the code). This isn't allowed, since Rust's borrow checker ensures that:

1. There are no mutable references when there are immutable references
2. There are no references when there are mutable references

To solve this problem, we can use `RefCell` to allow for interior mutability. This will allow us to borrow `self` mutably and immutably at the same time. It works by checking the borrow rules at runtime, rather than compile time.

Let's redefine `Parser` using `RefCell`:

```rust,ignore
use std::cell::RefCell;

pub struct Parser {
    tokens: Vec<Token>,
    pos: RefCell<usize>,
}
```

### Helper methods
Before we develop the methods for each non-terminal, we should introduce some helper functions to reduce the boilerplate code we write.

#### Position
Whilst we may not use this method when parsing, the helper methods we'll introduce later on will need to get the position we're currently at. Since it's somewhat of a hassle to get the position from the `RefCell`, we'll define a helper method for it:

```rust,ignore
impl Parser {
    /// Get position
    fn pos(&self) -> usize {
        *self.pos.borrow()
    }
}
```

Easy peasy. Now, we can use `self.pos()` to get the current position.

We may also want to advance, and it's prettier to use functions so:
    
```rust,ignore
impl Parser {
    /// Advance the parser by one token
    fn advance(&self) {
        *self.pos.borrow_mut() += 1;
    }
}
```

#### Token access
Throughout parsing, we'll continually want to get the current token we're parsing. There is a possibility the `pos` may be **out of bounds**, so we'll opt for returning `Option<&Token)` -- which, coincidentally, is the return value for `Vec::get`.

```rust,ignore
impl Parser {
    /// Get the current token
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos())
    }
}
```

This is a little messy since we'll need to handle the case where we reached the end of the stream. We can use some helpful features of `Option` to make this cleaner. This way, we will return `Result<&Token>` instead, which we can use the `?` operator to handle.

```rust,ignore
impl Parser {
    /// Get current token
    /// Raises an error if at EOF
    fn current_or_eof(&self) -> Result<&Token> {
        self.current() // get the current token
            .filter(|token| token.kind != TokenKind::Eof) // remove it if it's EOF
            .ok_or_else(|| anyhow!(self.eof_error())) // raise an error if none
    }
}
```

There is a lot going on here, so let's break it down:
1. `self.current()` gets the current token
2. `filter` removes the token if it's EOF
3. `ok_or_else` raises an error if there are no tokens left

We can also create a helper method `eof_error()` to generate the error message:

```rust,ignore
impl Parser {
    /// Get the last token and return an EOF error
    fn eof_error(&self) -> LangError {
        let token = self.tokens.last().unwrap();
        LangError::UnexpectedEOF(token.span.clone())
    }
}
```

Nothing special, all simple.

#### Expecting a token
We're often going to need to expect a certain token, simply meaning we need to guarantee that the token we're currently parsing is exactly a certain kind. The process is essentially:
1. Get the current token and advance
2. Check if it's the expected token

Nothing too major.

```rust,ignore
impl Parser {
    /// Expect a token and advance
    /// Does not work with EOF
    fn expect(&mut self, kind: TokenKind) -> Result<&Token> {
        let current: &Token = self.current_or_eof()?;
        self.advance();

        if current.kind == kind {
            Ok(current)
        } else {
            Err(anyhow!(LangError::ExpectedToken {
                expected: kind,
                found: current.kind.clone(),
                span: current.span.clone(),
            }))
        }
    }
}
```

## Non-terminal methods
Let's start with the start symbol \\(S\\) of our grammar: `program`.

Recall the [grammar](./context_free_grammars.md) we defined:

<details>
<summary><strong>View the grammar</strong></summary>

```ebnf
program           ::= item*

item              ::= function_decl
                    # | struct_decl
                    # | enum_decl
                    # | impl_block

function_decl     ::= "fn" IDENTIFIER "(" {parameter_list} ")" "->" type block

parameter_list    ::= parameter ("," parameter)*

parameter         ::= IDENTIFIER ":" type

type              ::= primitive_type
                    # | user_defined_type
                    # | tuple_type
                    # | reference_type
                    # | array_type
                    # | function_type

primitive_type    ::= "int" | "float" | "bool"

block             ::= "{" statement* "}"

statement         ::= expression
                    | variable_decl
                    | flow_statement
                    | return_statement


expression        ::= primary_expression
                    | unary_expression
                    | binary_expression

primary_expression ::= literal
                    | IDENTIFIER
                    | "(" expression ")"
                    | function_call

literal           ::= INT | FLOAT | BOOLEAN

unary_expression  ::= "-" expression | "!" expression #| "&" expression | "*" expression

binary_expression ::= expression OPERATOR expression

function_call     ::= IDENTIFIER "(" arguments ")"
arguments         ::= expression ("," expression)* | ε

variable_decl     ::= "let" IDENTIFIER ":" type "=" expression ";"

flow_statement    ::= "if" expression block {"else" block}

return_statement  ::= "return" {expression} ";"
```

</details>

The program section:
```ebnf
program           ::= item*
```

### `Program`
Simply means to parse 0 or more items. We can implement this as a loop that keeps parsing items until we reach the end of the file.

```rust,ignore
    /// Parse a program
    fn program(&mut self) -> Result<Program> {
        let mut items = Vec::new();
        while self.current().is_some() {
            items.push(self.item()?);
        }

        Ok(Program { items })
    }
```

### `Item`
The `item` non-terminal is where we introduce how to implement the pipe `|` in a parser. Right now, we are only parsing functions, so we'll only implement the function declaration. Regardless, we will implement it for scalability so we can add extra items later.

```rust,ignore
    /// Parse a single item
    fn item(&mut self) -> Result<Item> {
        let current = self.current_or_eof()?;

        let item = match current.kind {
            TokenKind::Fn => Item::FunctionDecl(self.function()?),
            _ => {
                return Err(anyhow!(LangError::ExpectedAnyToken {
                    expected: vec![TokenKind::Fn,],
                    found: current.kind.clone(),
                    span: current.span.clone(),
                }))
            }
        };

        Ok(item)
    }
```

### `Function` and `Parameter`
The `function` non-terminal grammar is as follows:
```ebnf
function_decl     ::= "fn" IDENTIFIER "(" {parameter_list} ")" "->" type block
```
There's a few new things here, main one being accumulating information throughout the process. The process is still straightforward, we follow the grammar:
1. Expect `fn`
2. Expect an `ident`, and store it
3. Expect `(`, and collect parameters
    1. Get and store `ident`
    2. Expect `:`
    3. Get and store `type`
    4. Repeat until `)`
4. Expect `)`
5. Expect `->`
6. Get and store `type`
7. Parse and store `block`

Let's implement it step by step. Let's start with the function signature, not including the body (`block`).

```rust,ignore
    fn function(&mut self) -> Result<FunctionDecl> {
        // "fn"
        self.expect(TokenKind::Fn)?;

        // IDENTIFIER
        let ident = self.ident()?;

        // "("
        self.expect(TokenKind::LParen)?;

        // parameter list
        let mut params: Vec<Parameter> = Vec::new();
        while self.current_or_eof()?.kind != TokenKind::RParen {
            let param = self.parameter()?;
            params.push(param);

            if self.current_or_eof()?.kind == TokenKind::Comma {
                self.advance();
            }
        }

        // ")"
        self.expect(TokenKind::RParen)?;

        // "->"
        self.expect(TokenKind::Arrow)?;

        // type
        let ty = self.type_()?;

        unimplemented!()
    }
```

Here, we defined a few new functions:
- `ident`
    - Although tokenization makes this simple, we like to reduce the boilerplate code we write
- `parameter`
- `type_`
    - Same as above

Finally, we should parse the block, and return the function. No, we will not implement the block parsing inside the `function` method. `block` is a non-terminal which exists in the grammar, so it will have its own method.

The reason we didn't create a method for `parameter_list` despite it being a non-terminal is for simplicity: the only dependant is `function`, so we can just implement it inside `function`. It's still perfectly fine (and probably better) to implement it as a separate method.

Let's add the block parsing and return the function:

```rust,ignore
    fn function(&mut self) -> Result<FunctionDecl> {
        // ...

        // block
        let block = self.block()?;

        Ok(FunctionDecl {
            ident,
            parameters: params,
            ty,
            block,
        })
    }
```

The function signature isn't fully complete yet, we need to implement parsing the parameters too. Let's do that now.

```rust,ignore
    fn parameter(&mut self) -> Result<Parameter> {
        // Expect and store ident
        let ident = self.ident()?;

        // ":"
        self.expect(TokenKind::Colon)?;

        // Get and store type
        let ty = self.type_()?;
        Ok(Parameter { ident, ty })
    }
```

### Primitives, types, etc.
We've reached a point where there are quite a lot of errors on the screen though. Let's cut down on how many errors we see by implementing the `type`, and `ident` methods now.

#### Types
Types are simple, we can just match and return a token which are valid types. We can put the primitive types here for now:

```rust,ignore
    fn type_(&mut self) -> Result<Type> {
        match self.current_or_eof()?.kind {
            TokenKind::Int => {
                self.advance();
                Ok(Type::Primitive(PrimitiveType::Int))
            }
            TokenKind::Bool => {
                self.advance();
                Ok(Type::Primitive(PrimitiveType::Bool))
            }
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![TokenKind::Int, TokenKind::Bool,],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }
```

#### `ident`
Identifier is very very straight-forward. Thanks to our tokenization, we don't need any complex rules since the identifier tokens are already recognised by the lexer. This is simply a function to reduce boilerplate by giving a consistent error message:

```rust,ignore
    fn ident(&mut self) -> Result<Ident> {
        match self.current_or_eof()?.kind {
            TokenKind::Ident(ref ident) => {
                let ident = ident.clone();
                self.advance();
                Ok(Ident { ident })
            }
            _ => Err(anyhow!(LangError::ExpectedToken {
                expected: TokenKind::Ident("".to_string()),
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }
```

### Blocks
Recall the `block` non-terminal production rule:
```ebnf
block             ::= "{" statement* "}"
```

This is pretty simple, we just need to parse 0 or more statements. We can implement this as a loop that keeps parsing statements until we reach the end of the block.

```rust,ignore
    fn block(&mut self) -> Result<Block> {
        // "{"
        self.expect(TokenKind::LBrace)?;

        let mut statements = Vec::new();
        while self.current_or_eof()?.kind != TokenKind::RBrace {
            statements.push(self.statement()?);
        }

        // "}"
        self.expect(TokenKind::RBrace)?;

        Ok(Block { statements })
    }
```

### Statements (+ `variable_decl`, `flow_statement`, `return_statement`)
Now we're entering the territory of parsing expressions, something a lot of people struggle with.

```ebnf
statement         ::= expression
                    | variable_decl ";"
                    | return_statement ";"
                    | flow_statement ";"
```

For this non-terminal, the production rule will be either `expression`, `variable_decl`, or `flow_statement`. The easiest and clearest way to determine which to branch off to is simply the starting token:
- `Let` implies `variable_decl`
- `If` implies `flow_statement`
- `Return` implies `return_statement`
- Anything else implies `expression`

We can implement this as a simple match statement:

```rust,ignore
    fn statement(&mut self) -> Result<Statement> {
        match self.current_or_eof()?.kind {
            TokenKind::Let => self.variable_decl(),
            TokenKind::If => self.flow_statement(),
            TokenKind::Return => self.return_statement(),
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![TokenKind::Let,],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }
```

Let's explore each in order:

#### `variable_decl`
The `variable_decl` non-terminal grammar is as follows:
```ebnf
variable_decl     ::= "let" IDENTIFIER ":" type "=" expression ";"
```

A relatively trivial process, we just need to follow the grammar:
```rust,ignore
    fn variable_decl(&mut self) -> Result<Statement> {
        // "let"
        self.expect(TokenKind::Let)?;

        // IDENTIFIER
        let ident = self.ident()?;

        // ":"
        self.expect(TokenKind::Colon)?;

        // type
        let ty = self.type_()?;

        // "="
        self.expect(TokenKind::Equals)?;

        // expression
        let expression = self.expression()?;

        let var_decl = Statement::VariableDecl(VariableDecl {
            ident,
            ty,
            expression,
        });

        // ";"
        self.expect(TokenKind::Semicolon)?;

        Ok(var_decl)
    }
```

#### `flow_statement`
The `flow_statement` non-terminal grammar is as follows:
```ebnf
flow_statement    ::= "if" expression block {"else" block}
```

This is a simple process:
1. Expect `if`
2. Parse and store `expression`
3. Parse and store `block`
4. Branch depending on token, and store:
    - `else`: parse and store `block` -> `Some(block)`
    - Anything else: return `None`

```rust,ignore
    fn flow_statement(&mut self) -> Result<Statement> {
        // "if"
        self.expect(TokenKind::If)?;

        // condition
        let condition = self.expression()?;

        // block
        let if_block = self.block()?;

        // may be else, may be not
        let else_block = if self.current_or_eof()?.kind == TokenKind::Else {
            self.advance();
            Some(self.block()?)
        } else {
            None
        };

        let stmt = Statement::Flow(FlowStatement {
            condition,
            if_block,
            else_block,
        });

        Ok(stmt)
    }
```

#### `return_statement`
The `return_statement` non-terminal grammar is as follows:
```ebnf
return_statement  ::= "return" {expression} ";"
```

We can convert this to code quite easily:
```rust,ignore
    fn return_statement(&mut self) -> Result<Statement> {
        // "return"
        self.expect(TokenKind::Return)?;

        // expression or not
        let expression = if self.current_or_eof()?.kind != TokenKind::Semicolon {
            Some(Box::new(self.expression()?))
        } else {
            None
        };

        // ";"
        self.expect(TokenKind::Semicolon)?;

        let stmt = Statement::Return(expression);

        Ok(stmt)
    }
```

### Expressions
Prior to this, we followed straight-forward rules. Here, we'll have to implement operator precedence. This is a bit more complex, but we'll start with the simple stuff.

> **Note**
>
> Associativity will not be covered, since we don't have any right-associative operators (like exponentiation).

#### `expression`
Here's the grammar for `expression`:
```ebnf
expression        ::= primary_expression
                    | unary_expression
                    | binary_expression
```

This states that it can be one of stated three expression types. First, we'll make a simplistic approach where we calculate the left-hand side (lhs), and then the right-hand side (rhs). We'll then combine them into a `BinaryExpression` if we find an operator.

```rust,ignore
    fn expression(&mut self) -> Result<Expression> {
        let lhs = self.primary_expression()?;

        // Check if there's an operator
        if let Some(op) = self.current()?.binary_operator() {
            self.advance();
            let rhs = self.expression()?;
            Ok(Expression::Binary(BinaryExpression {
                lhs: Box::new(lhs),
                op: op.clone(),
                rhs: Box::new(rhs),
            }))
        } else {
            Ok(lhs)
        }
    }
```

The problem with this approach is that it doesn't handle operator precedence (i.e. the following binary expression will take precedence over the previous one).

Instead of determining the left and right-hand side, we'll maintain a mutable `expr` variable and keep updating it as we parse the expression.

To do this, we would terminate the loop of updating `expr` when the operator being parsed is of a lower precedence than the previous expression. We need a way to determine the operator of a given expression first, if any. Let's implement a helper method in `src/ast/expr.rs`:

```rust,ignore
impl Expression {
    /// Get the binary operator of the expression, if any
    pub fn binary_operator(&self) -> Option<&BinaryOperator> {
        match self {
            Expression::Binary(binary_expr) => Some(&binary_expr.op),
            _ => None,
        }
    }
}
```

With this, we can start implementing the new algorithm. Here's the process:
1. Parse & store `expr`
2. While there is an operator, continue this loop:
    1. If the operator has a lower precedence than the previous operator, break the loop
    2. Parse the `rhs = primary()` and store it
    3. Update `expr` to a new `BinaryExpression` with `lhs`, `rhs`, and the current `op`
    4. Repeat the loop with the top condition

The new algorithm will allow us to retain information about the left-hand side expression and whether or not it should take precedence over the new right-hand side.

```rust,ignore
    fn expression(&mut self) -> Result<Expression> {
        let mut expr = self.primary()?;

        while let Some(op) = self.current_or_eof()?.kind.as_binary_operator() {
            let op_precedence = op.precedence();

            // Check if the operator precedence is higher than the previous operator
            if let Some(prev_op) = expr.binary_operator() {
                if op_precedence <= prev_op.precedence() {
                    break; // Break the loop if the current operator has lower precedence
                }
            }

            // Consume the operator token
            self.advance();

            // Parse the right-hand side expression
            let rhs = self.primary()?;

            // Parse binary expression
            expr = Expression::Binary(BinaryExpression {
                lhs: Box::new(expr),
                op: op,
                rhs: Box::new(rhs),
            });
        }

        Ok(expr)
    }
```

#### `primary_expression`
The `primary_expression` non-terminal grammar is as follows:
```ebnf
primary_expression ::= literal
                    | IDENTIFIER
                    | "(" expression ")"
                    | function_call
```

This is a straightforward `match` statement, and so we'll implement it as such:
```rust,ignore
    fn primary(&mut self) -> Result<Expression> {
        match self.current_or_eof()?.kind {
            TokenKind::IntLiteral(_) | TokenKind::BoolLiteral(_) => {
                self.literal().map(Expression::Primary)
            }
            TokenKind::Ident(_) => {
                let ident = self.ident()?;
                Ok(Expression::Primary(PrimaryExpression::Ident(ident)))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.expect(TokenKind::RParen)?;
                Ok(Expression::Primary(PrimaryExpression::Parenthesized(
                    Box::new(expr),
                )))
            }
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![
                    TokenKind::IntLiteral(0),
                    TokenKind::BoolLiteral(false),
                    TokenKind::Ident("".to_string()),
                    TokenKind::LParen,
                ],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }
```

#### Unary expressions
For simplicity sake, we'll implement the unary operators within the `expression` function.

To do this, we'll add a loop to handle unary operators. We'll parse the right-hand side expression, and then handle the unary operator. We'll then update `expr` to a new `UnaryExpression` with the current `op` and `rhs`.

Let's adjust the loop:
```rust,ignore
        while let Some(op) = self.current_or_eof()?.kind.as_binary_operator() {
            // ...

            // Parse the right-hand side expression
            let rhs = self.primary()?;

            // Handle unary operators
            while let Some(unary_op) = self.current_or_eof()?.kind.as_unary_operator() {
                // Consume the operator token
                self.advance();

                // Parse the right-hand side expression
                let rhs = self.primary()?;

                let unary_expression = match unary_op {
                    UnaryOperator::Negate => UnaryExpression::Negation(Box::new(rhs)),
                    UnaryOperator::Not => UnaryExpression::Not(Box::new(rhs)),
                };

                expr = Expression::Unary(unary_expression);
            }

            // Parse binary expression
            expr = Expression::Binary(BinaryExpression {
                lhs: Box::new(expr),
                op: op,
                rhs: Box::new(rhs),
            });
        }
```

#### `literal`
The `literal` non-terminal grammar is as follows:
```ebnf
literal           ::= INT | FLOAT | BOOLEAN
```

This is a simple process, we just need to match and return a token which are valid literals:
```rust,ignore
    fn literal(&mut self) -> Result<PrimaryExpression> {
        match self.current_or_eof()?.kind {
            TokenKind::IntLiteral(value) => {
                self.advance();
                Ok(PrimaryExpression::Literal(Literal::Int(value)))
            }
            TokenKind::BoolLiteral(value) => {
                self.advance();
                Ok(PrimaryExpression::Literal(Literal::Bool(value)))
            }
            _ => Err(anyhow!(LangError::ExpectedAnyToken {
                expected: vec![TokenKind::IntLiteral(0), TokenKind::BoolLiteral(false),],
                found: self.current_or_eof()?.kind.clone(),
                span: self.current_or_eof()?.span.clone(),
            })),
        }
    }
```

#### Function calls
Again, we will directly implement parsing function calls inside `primary` (primary_expressions). This is because function calls are a part of the primary expression, and so it makes sense to implement it here.

In the `TokenKind::Ident(_)` match, we'll check if the next token is an `LParen`. If it is, we'll parse the arguments and return a `FunctionCall` expression.

```rust,ignore
            TokenKind::Ident(_) => {
                let ident = self.ident()?;

                // Function call
                if self.current_or_eof()?.kind == TokenKind::LParen {
                    self.advance();
                    let args = self.expression_list()?;
                    self.expect(TokenKind::RParen)?;
                    Ok(Expression::Primary(PrimaryExpression::FunctionCall(
                        FunctionCall { ident, args },
                    )))
                } else {
                    Ok(Expression::Primary(PrimaryExpression::Ident(ident)))
                }
            }
```

Let's implement the `expression_list` method now. This is a simple process, we just need to parse 0 or more expressions. We can implement this as a loop that keeps parsing expressions until we reach the end of the list.

```rust,ignore
    fn expression_list(&mut self) -> Result<Vec<Expression>> {
        let mut args = Vec::new();
        while self.current_or_eof()?.kind != TokenKind::RParen {
            args.push(self.expression()?);
            if self.current_or_eof()?.kind == TokenKind::Comma {
                self.advance();
            }
        }
        Ok(args)
    }
```

## Testing
That's quite a lot of code. In fact, currently my `parser.rs` file is at 482 lines! It's a good idea to test the parser to ensure it's working as expected.

This section would a lot cleaner if you completed the section on [AST Optional Helper Methods](./implementing_an_ast.md#optional-helpers). You can still debug-print them if you haven't, but it's a lot more convenient to use the helper methods.

Let's run the test on this file:
```rust,ignore
fn main() -> int {
    let a: int = 5 + 2;
    let b: int = 6;
    let c: int = a + b;
    return c;
}
```

This is a test in the `main::add_compile`, so we can try `cargo test add_compile -- -nocapture` to test & view the output.

We get the following output:
```
Program
    FuncDecl main() -> int {
        Block
            VariableDecl a:int =
                BinaryExpression
                op=Add
                    Int 5
                    Int 2
            VariableDecl b:int =
                Int 6
            VariableDecl c:int =
                BinaryExpression
                op=Add
                    Ident a
                    Ident b
            Return
                Ident c
    }
```

It seems to be working pretty well! One of the typical errors is forgetting a semicolon, so let's see if it catches that too.

```rust,ignore
fn main() -> int {
    let a: int = 5 + fn 2;
    let b: int = 6;
    let c: int = a + b;
    return c;
}
```

With the same call, we get:
```plaintext
Error:
   ╭─[tests/add.pyl:2:22]
   │
 2 │     let a: int = 5 + fn 2;
   │                      ─┬
   │                       ╰── Expected any of the tokens: `IntegerLiteral`, `BooleanLiteral`, `Ident`, `(`, found: `fn`
───╯
Error: Failed to parse file
```

# Resources
- [RefCell documentation](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
- [RefCell in book](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)