# Lexical Analysis

In this chapter, we'll cover the theory and implement a working lexer.

## Introduction
The primary purpose of lexical analysis is to simplify the subsequent phases of the compiler (the parser!) by providing an unstructured categorized representation of the source code as a stream of **tokens**. This can be done in a few ways.

Regular expressions are patterns used to define the syntax of tokens in a programming language, used by tokenizers/lexers to recognize tokens. This guide will not use regular expressions. This is mainly because it's inefficient (lots of backtracking), is very deterministic, and is difficult to debug and control. I don't like regex either.

Finite automata, including deterministic finite automata (DFAs) and non-deterministic finite automata (NFAs), serve as theoretical models for defining and implementing lexical analyzers.

We will go into more detail in later sub-sections. For now, let's get started!

## Our Language
Before we start, we should get a clear idea of what our language should look like. We'll keep it simple, with the following features:
- Types (integer and boolean), i.e. `int`, `bool`
- Variables, i.e. `let x: int = 5;`
- Arithmetic operations (`+`, `-`, `*`, `/`), i.e. `let x: int = 5 + 3;`
- Parentheses, i.e. `let x: int = (5 + 3) * 2;`
- If statements, i.e. `if (x > 5) { ... } else { ... }`

We would like it to be **strongly typed**, so we can't do things like `let x: int = true;`. We'll also have a few reserved keywords, such as `let`, `if`, `else`, `int`, `bool`. Each function should have a return type, and we'll have a few built-in functions.

Let's create a simple example code snippet for our language:
```rust
// Declare a function `main()`, which returns an integer (exit code)
fn main() -> int {
    let x: int = 5;
    let y: int = 3;
    let z: int = x + y;
    if (z > 5) {            // Branching!
        return z;           // Return z
    } else {
        return 0;           // Return 0
    }
}
```

This is a simple program which declares two variables, adds them together, and returns the result if it's greater than 5. Otherwise, it returns 0. It follows a rust-like syntax, but it's not exactly Rust.

Anyways, let's break it down into tokens:
1. `fn`: Keyword declaring a function
2. `main`: Identifier for the function
3. `(`: Parenthesis
4. `)`: Parenthesis
5. `->`: Arrow, indicating the return type
6. `int`: Keyword for the return type
7. `{`: Curly brace
8. `let`: Keyword for declaring a variable
9. `x`: Identifier for the variable
10. `:`: Colon, indicating the type
11. `int`: Keyword for the type
12. `=`: Assignment operator
13. `5`: Integer literal
14. `;`: Semicolon, indicating the end of the statement

And so forth. This will help us understand how we can structure our tokens.

# Resources
- [Awesome FSM](https://github.com/leonardomso/awesome-fsm): A curated list of awesome finite state machine libraries, software and resources.