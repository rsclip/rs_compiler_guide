# Error Handling with `ariadne`

> ✅ This is the error crate we will be using.
>
> Too confusing? Try [codespan-reporting](./error_handling_with_codespan_reporting.md) instead.

We'll use `ariadne` to design prettier error messages. This library allows us to highlight the source code where the error occurred, and provide a helpful message to the user. It's extremely strong and versatile, as well as able to generate beautiful error messages.

Take a moment to review [ariadne's documentation](https://docs.rs/ariadne/*/ariadne/), and the [example](https://crates.io/crates/ariadne) can help tremendously. As of writing, the documentation can be initially difficult to understand, but the example is very helpful.

## File System
We'll opt for making our file system for `ariadne` to use. This will allow us to load a file into memory, and then use the file ID to generate diagnostics.

Let's look into the structure we need to create for the file system.

### `SourceFile`
We need to store our source files somewhere. Let's create a `SourceFile` struct to hold the file's name and the source code. We'll define some simple functions for creating them too.

We will use `ariadne::Source` to make sure it's compatible with the library; we can easily create it from source code by `Source::from(source_code: String)`.

```rust,ignore
use ariadne::Source;

/// Represents a source file
pub struct SourceFile {
    pub id: String,
    pub source: Source,
}

impl SourceFile {
    /// Create a new `SourceFile`
    pub fn new(id: String, contents: String) -> Self {
        Self {
            id,
            source: Source::from(contents),
        }
    }

    /// Create a new `SourceFile` from a string
    /// Name is set to "src"
    pub fn from_code(contents: String) -> Self {
        Self {
            id: String::new(),
            source: Source::from(contents),
        }
    }
}
```

Great, we can create a source file from a string, and we can also create a source file from a file name and its contents.

### `Files`
Here, we'll create a `struct` to manage all these `SourceFile`s. We'll be able to add files, get files, and get the source code of a file.

There are some other features we might find useful:
- `get_file` to get a file by its name
- `get_source` to get the source code of a file by its name
- `add_file` to add a file to the collection

In `files.rs`:
```rust,ignore
use std::collections::HashMap;

/// Represents a collection of source files
pub struct Files {
    files: HashMap<String, SourceFile>,
}

impl Files {
    /// Create a new `Files`
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Add a file to the collection
    pub fn add_file(&mut self, id: String, contents: String) {
        let file = SourceFile::new(id, contents);
        self.files.insert(file.id.clone(), file);
    }

    /// Get a file from the collection
    fn get_file(&self, path: &str) -> Option<&SourceFile> {
        self.files.get(path)
    }
}
```

This "filesystem" is essentially what `ariadne` considers a **Cache**. This cache essentially is a store of source files (dependant on the file ID), and we can use it to generate diagnostics.

Because of the way we designed our struct, implementing the [`Cache` trait](https://docs.rs/ariadne/*/ariadne/trait.Cache.html) is simple. We can use the `get` method to get a file by its ID, and the `get_source` method to get the source code of a file by its ID.

```rust,ignore
impl ariadne::Cache<String> for Files {
    type Storage = String;

    fn fetch(
        &mut self,
        id: &String,
    ) -> Result<&Source<Self::Storage>, Box<dyn std::fmt::Debug + '_>> {
        let file = self.get_file(id).unwrap();
        Ok(&file.source)
    }

    fn display<'a>(&self, id: &'a String) -> Option<Box<dyn fmt::Display + 'a>> {
        let file = self.get_file(id)?;
        Some(Box::new(file.id.clone()))
    }
}
```

The `Storage` type declares exactly what the ID is (we refer to files by `String`s here). The `fetch` method is used to get the source code of a file by its ID, and the `display` method is used to display the file's name.

In the `fetch` method, we simply call the function we defined earlier to get the file by its ID, and then return the source code. In the `display` method, we get the file by its ID, and then return the file's name.

## Working with spans
`ariadne` really wants us to use spans which it can use, through the [`ariadne::Span` trait](https://docs.rs/ariadne/latest/ariadne/trait.Span.html). This requires us to actually have the file ID of the span, which (un)fortunately our current `Span` lacks. We would prefer not to store the file ID in the `Span` itself -- storing it for each token is memory-inefficient.

Instead, we can determine that the only time we would need to access the `Span` in a reporting manner is, of course, when we need to report an error. Thus, we can create a new `struct` which is generated on the fly, which includes the needed information.

<details>
<summary>Why do we need to store the file ID?</summary>
We'd need to implement the trait later on, which means we need to store the file ID in the struct itself. If you can think of a way to retrieve the file ID without storing it, that would be interesting.
</details>

```rust,ignore
/// Ariadne-compatible span
/// To be generated on-the-fly.
#[derive(Debug, Clone, PartialEq)]
pub struct ReportableSpan {
    pub file: String,
    pub start: usize,
    pub end: usize,
}

impl ReportableSpan {
    pub fn new(file: String, span: &Span) -> Self {
        Self {
            file,
            start: span.start,
            end: span.end,
        }
    }
}
```

This `struct` is essentially a wrapper around the `Span` struct, but it includes the file ID. We can create it from a `Span` and a file ID, and then use it to generate diagnostics.

Before it's ready for use, we need to implement `ariadne::Span` to it:

```rust,ignore
impl ariadne::Span for ReportableSpan {
    type SourceId = String;

    fn source(&self) -> &Self::SourceId {
        &self.file
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }
}
```

## Generating Diagnostics
Here's an example error message we'll be able to generate later:

```plaintext
Error:
   ╭─[tests/test:1:12]                                      // File:Line:Column
   │
 5 │     if name == 335.333.2 { return true; }              // Source from span
   │                ───────┬─
   │                       ╰──── Unexpected character: `.`  // A label with a message
───╯
```

Pretty, right? We can build this using `Report::build`!

You can build an error with a `ReportKind` (i.e. `ReportKind::Error`), the file ID, and the span. After that, you can add whatever you like, such as labels with their own spans and messages and colors.

Let's implement a function to generate a diagnostic across any `LangError` variant.

In `errors.rs`:
```rust,ignore
use ariadne::{Color, Label, Report, ReportKind};
use crate::token::{ReportableSpan, Span};

impl LangError {
    /// Produces a diagnostic for the error
    pub fn diagnostic(&self, file: String) -> Report<ReportableSpan> {
        // Get the span of the error
        let span = self.span(file);

        // This is pretty much the error message
        let label = Label::new(span.clone())
            .with_message(self.to_string())
            .with_color(Color::Red);

        // Create an `ReportKind::Error` report with the message
        Report::build(ReportKind::Error, self.to_string(), span.start)
            .with_label(label)
            .finish()
    }

    /// Get the span of the error
    pub fn span(&self, file: String) -> ReportableSpan {
        ReportableSpan::new(
            file,
            match self {
                LangError::UnexpectedCharacter(_, span) => span,
                LangError::UnterminatedString(span) => span,
                LangError::ExpectedToken { span, .. } => span,
                LangError::ExpectedAnyToken { span, .. } => span,
                LangError::UnexpectedEOF(span) => span,
                LangError::InvalidLiteral(_, span) => span,
            },
        )
    }
}
```

We also included a little helper function to extract the span out of errors, since because of a silly design decision, the position of the span in the error is not consistent. This might not be needed if you have a better design!

Note that `self.to_string()` is thanks to `thiserror`'s implementation using the strings we made above each variant.

## Error Reporting
To streamline the error process, we can create our own `ErrorReporter` struct that will handle the file system, and report all the diagnostics at the end.

It'll have a **mutable reference** to the file system (necessary for diagnostic printing).

```rust,ignore
pub struct ErrorReporter<'a> {
    files: &'a mut Files,
}
```

We can create a `report` method for a single error. We'll simply want to generate a `Diagnostic` and then render it to the terminal (or another output).

```rust,ignore
impl<'a> ErrorReporter<'a> {
    pub fn report(&mut self, file: String, error: &anyhow::Error) -> Result<()> {
        let diagnostic = match error.downcast_ref::<LangError>() {
            Some(e) => e.diagnostic(file),
            None => {
                return Err(anyhow!("Unknown error: {}", error));
            }
        };

        match diagnostic.eprint(&mut self.files) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("Failed to print diagnostic: {}", e)),
        }
    }
}
```

This is moderately complex. We first need to downcast the error to a `LangError`, and then we can generate a diagnostic from it. After that, we can print the diagnostic to the terminal. We make sure to catch any possible errors which may arise.

# Resources
- [ariadne documentation](https://docs.rs/ariadne/*/ariadne/)
- [ariadne Span trait](https://docs.rs/ariadne/latest/ariadne/trait.Span.html)
- [ariadne Cache trait](https://docs.rs/ariadne/*/ariadne/trait.Cache.html)