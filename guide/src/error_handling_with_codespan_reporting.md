# Error Handling with `codespan-reporting`

> ⚠️ We will not be using this crate, but it's a simpler alternative to `ariadne`.

We'll use `codespan-reporting` to design prettier error messages. This library allows us to highlight the source code where the error occurred, and provide a helpful message to the user. Plus, it's incredibly useful!

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