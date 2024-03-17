//! Contains the error types used in the library while parsing and lexing.
//! Used with `anyhow` to provide context to errors.
//! 
//! Should migrate to `ariadne` crate for better error handling and reporting.

use crate::token::{Span, TokenKind};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use thiserror::Error;

/// The error type for the library
#[derive(Debug, Error)]
pub enum LangError {
    // Lexer errors
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

pub struct ErrorReporter<'a> {
    files: &'a SimpleFiles<String, String>,
    writer: StandardStream,
    config: codespan_reporting::term::Config,
}

impl LangError {
    /// Produces a diagnostic for the error
    pub fn diagnostic(&self, file_id: usize) -> Diagnostic<usize> {
        let mut diagnostic = Diagnostic::error().with_message(self.to_string());

        match self {
            LangError::UnexpectedCharacter(ch, span) => {
                diagnostic =
                    diagnostic.with_labels(vec![Label::primary(file_id, span.start..span.end)
                        .with_message(format!("Unexpected character: `{}`", ch))]);
            }
            LangError::UnterminatedString(span) => {
                diagnostic =
                    diagnostic.with_labels(vec![Label::primary(file_id, span.start..span.end)
                        .with_message("Unterminated string")]);
            }
            LangError::ExpectedToken {
                expected,
                found,
                span,
            } => {
                diagnostic =
                    diagnostic.with_labels(vec![Label::primary(file_id, span.start..span.end)
                        .with_message(format!("Expected: `{}`, found: `{}`", expected, found))]);
            }
            LangError::ExpectedAnyToken {
                expected,
                found,
                span,
            } => {
                diagnostic =
                    diagnostic.with_labels(vec![Label::primary(file_id, span.start..span.end)
                        .with_message(format!(
                            "Expected any of: `{}`, found: `{}`",
                            LangError::any_tokens_display(expected),
                            found
                        ))]);
            }
            LangError::UnexpectedEOF(span) => {
                diagnostic =
                    diagnostic.with_labels(vec![Label::primary(file_id, span.start..span.end)
                        .with_message("Unexpected EOF")]);
            }
            LangError::InvalidLiteral(_, span) => {
                diagnostic =
                    diagnostic.with_labels(vec![Label::primary(file_id, span.start..span.end)
                        .with_message("Invalid literal")]);
            }
        };

        diagnostic
    }

    fn any_tokens_display(tokens: &[TokenKind]) -> String {
        let mut s = String::new();
        for (i, token) in tokens.iter().enumerate() {
            s.push_str(&format!("{:?}", token));
            if i < tokens.len() - 1 {
                s.push_str(", ");
            }
        }
        s
    }
}

impl<'a> ErrorReporter<'a> {
    pub fn new(files: &'a SimpleFiles<String, String>) -> Self {
        Self {
            files,
            writer: StandardStream::stderr(ColorChoice::Always),
            config: codespan_reporting::term::Config::default(),
        }
    }

    pub fn report(&self, file_id: usize, error: &anyhow::Error) -> anyhow::Result<()> {
        let diagnostic = if let Some(lang_error) = error.downcast_ref::<LangError>() {
            lang_error.diagnostic(file_id)
        } else {
            Diagnostic::error().with_message(error.to_string())
        };

        codespan_reporting::term::emit(
            &mut self.writer.lock(),
            &self.config,
            self.files,
            &diagnostic,
        )?;

        Ok(())
    }

    pub fn report_all(&self, errors: Vec<(usize, &anyhow::Error)>) -> anyhow::Result<()> {
        for (file_id, error) in errors {
            self.report(file_id, error)?;
        }
        Ok(())
    }
}