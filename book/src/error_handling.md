# Error Handling
Before we dive into developing the compiler, it's important to have some foundations for how we will handle errors.

## Introduction
### Error Types
There are three types of errors that we will handle in our compiler:
- **Lexical Errors**: These errors occur when the lexer encounters an invalid token.
- **Syntax Errors**: These errors occur when the parser encounters an invalid syntax.
- **Semantic Errors**: These errors occur when the compiler encounters an invalid semantic.

We'll cover Semantic Errors later on when we discuss Semantic Analysis.

### Error Recovery
When an error is encountered, the compiler should attempt to recover from the error and continue parsing the input. This is important because it allows the compiler to report multiple errors in a single run, rather than stopping at the first error.

We can either **panic** or **recover** from an error. When we panic, we stop parsing and report the error. When we recover, we attempt to continue parsing the input. It's better to recover from an error, as it allows us to report multiple errors in a single run; imagine fixing an error one, recompiling, and then finding another error repeatedly. Not fun.

The errors should be **propagated** (i.e., passed up the call stack) to the top-level function, where they can be reported to the user.


## Error Handling in Rust
Let's setup the main `enum LangError` representing the different errors we might encounter at any stage in the process.

```rust,ignore
use crate::token::{Span, TokenKind};

#[derive(Debug)]
pub enum LangError {
    // An unexpected character was found
    UnexpectedCharacter(String, Span),

    // A string was not terminated
    UnterminatedString(Span),

    // Was expecting a particular token, found another
    ExpectedToken {
        expected: TokenKind,
        found: TokenKind,
        span: Span,
    },

    // Was expecting any of the tokens, found another
    ExpectedAnyToken {
        expected: Vec<TokenKind>,
        found: TokenKind,
        span: Span,
    },

    // Unexpected EOF
    UnexpectedEOF(Span),

    // Invalid literal
    InvalidLiteral(String, Span),
}
```

Straightforward, and it's clear to see it separated in two categories for now: Lexical and Syntax errors. We can always add more as we go along.

However, we could also like to use `thiserror` to derive the `Error` trait for our `LangError` enum. This will allow us to use the `?` operator to propagate errors up the call stack.

Let's implement:
```rust,ignore
use crate::token::{Span, TokenKind};
use thiserror::Error; // <<<<<<<< Make sure you use this!

#[derive(Debug, Error)]
pub enum LangError {
    // An unexpected character was found
    #[error("Unexpected character: `{0}`")]
    UnexpectedCharacter(String, Span),

    // A string was not terminated
    #[error("Unterminated string")]
    UnterminatedString(Span),

    // Was expecting a particular token, found another
    #[error("Expected token: `{expected}`, found: `{found}`")]
    ExpectedToken {
        expected: TokenKind,
        found: TokenKind,
        span: Span,
    },

    // Was expecting any of the tokens, found another
    #[error(
        "Expected any of the tokens: `{}`, found: `{found}`",
        LangError::any_tokens_display(expected)
    )]
    ExpectedAnyToken {
        expected: Vec<TokenKind>,
        found: TokenKind,
        span: Span,
    },

    // Unexpected EOF
    #[error("Unexpected EOF")]
    UnexpectedEOF(Span),

    // Invalid literal
    #[error("Invalid literal: `{0}`")]
    InvalidLiteral(String, Span),
}
```
Simply put, we add `#[error("...")]` to each variant, and we can use `{}` to interpolate values. We also add a method `any_tokens_display` to the `LangError` enum to display the expected tokens in a more readable format.

```rust,ignore
impl LangError {
    fn any_tokens_display(expected: &Vec<TokenKind>) -> String {
        expected
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
```
