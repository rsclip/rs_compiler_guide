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

### Error Reporting
We'll use `codespan-reporting` to design prettier error messages. This library allows us to highlight the source code where the error occurred, and provide a helpful message to the user. Plus, it's incredibly useful!

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

## Error Diagnostics
It's important to display the errors properly, so that the user can see exactly where the error occurred, why it occurred, and other useful information to help them debug the issue.

Using `codespan-reporting` we can generate a `Diagnostic` for each error, and then render the diagnostics to the terminal. The `Diagnostic` will contain the error message, the source code, and the span of the error.

Let's implement a function to generate a `Diagnostic` for each `LangError` variant.

### Diagnostics
Before we jump into making the function, it's important to review the [documentation](https://docs.rs/codespan-reporting/0.11.1/codespan_reporting/) (as always), and the [example](https://crates.io/crates/codespan-reporting) can help too.

Here's a simple uncolored error message we'll be able to generate later:
```
error: Unexpected character: `.`                    // Message
  ┌─ test:1:12                                      // File:Line:Column
  │
1 │ if name == 335.333.2 { return true; }           // Source from span
  │            ^^^^^^^^^ Unexpected character: `.`  // A label with a message
```
Here the literal uses two decimal points (which is invalid), and the error message is displayed with the source code and the span of the error. I've labelled what each part of the error message is.

We can determine that a `Diagnostic` error can be created by `Diagnostic::error()`. From here, we can add a message, and add as many labels as we want. We can add a span too, which is the range of the error in the source code.

### File systems
In `codespan-reporting`, we have a simple file system  `SimpleFiles<Name, Source>` we'll use. This essentially allows you to load a file into memory (which you get an associated file ID), and then you can use the file ID to generate diagnostics.

### Generating Diagnostics
Let's implement a function to generate a `Diagnostic` for each `LangError` variant.

```rust,ignore
impl LangError {
    /// Produces a diagnostic for the error
    pub fn diagnostic(&self, file_id: usize) -> Diagnostic<usize> {
        // Generate the base diagnostic with a main message
        let mut diagnostic = Diagnostic::error().with_message(self.to_string());

        // Add a label
        match self {
            LangError::UnexpectedCharacter(ch, span) => {
                diagnostic =
                    diagnostic.with_labels(vec![Label::primary(file_id, span.start..span.end)
                        .with_message(format!("Unexpected character: `{}`", ch))]);
            },
            // ...
        }

        diagnostic
    }
}
```

You can implement the rest of the variants in the `match` statement. It would be ideal for the message to briefly describe the overall error, and labels specifying exactly what went wrong.

## Error Reporting
To streamline the error process, we can create our own `ErrorReporter` struct that will handle the file system, and report all the diagnostics at the end.

```rust,ignore
pub struct ErrorReporter<'a> {
    /// Store the files in memory
    files: &'a SimpleFiles<String, String>,
    /// Store where to write the diagnostics
    writer: StandardStream,
    /// Store the configuration for the terminal
    config: codespan_reporting::term::Config,
}
```

You can determine how it can be constructed, but the key concept for us is that the immutable reference `&'a SimpleFiles<String, String>` implies we do not add any files.

Next, we can create a `report` method for a single error. We'll simply want to generate a `Diagnostic` and then render it to the terminal (or another output).

```rust,ignore
impl<'a> ErrorReporter<'a> {
    pub fn report(&self, file_id: usize, error: &anyhow::Error) -> anyhow::Result<()> {
        // Try to downcast the error to a LangError
        let diagnostic = if let Some(lang_error) = error.downcast_ref::<LangError>() {
            lang_error.diagnostic(file_id)
        } else {
            // If it fails somehow, just create a generic error
            Diagnostic::error().with_message(error.to_string())
        };

        // Render the diagnostic to the terminal
        codespan_reporting::term::emit(
            &mut self.writer.lock(),
            &self.config,
            self.files,
            &diagnostic,
        )?;

        Ok(())
    }
}
```

To make life easier, we can introduce a `report_all` method to report all the errors at once. This will be useful when we want to report all the errors at the end of the compilation process.

```rust,ignore
impl<'a> ErrorReporter<'a> {
    // ...

    pub fn report_all(&self, errors: Vec<(usize, &anyhow::Error)>) -> anyhow::Result<()> {
        for (file_id, error) in errors {
            self.report(file_id, error)?;
        }

        Ok(())
    }
}
```

# Resources
- [codespan-reporting](https://docs.rs/codespan-reporting/0.11.1/codespan_reporting/)
- [thiserror](https://docs.rs/thiserror/1.0.24/thiserror/)
- [codespan-reporting example](https://crates.io/crates/codespan-reporting)
- [SimpleFiles](https://docs.rs/codespan-reporting/0.11.1/codespan_reporting/files/struct.SimpleFiles.html)
- [Diagnostic](https://docs.rs/codespan-reporting/0.11.1/codespan_reporting/diagnostic/struct.Diagnostic.html)
- [Label](https://docs.rs/codespan-reporting/0.11.1/codespan_reporting/diagnostic/struct.Label.html)
- [Error Recovery](https://en.wikipedia.org/wiki/Error_recovery)
- [Panic vs Recover in Go](https://go.dev/blog/defer-panic-and-recover)