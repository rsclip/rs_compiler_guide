//! Contains the error types used in the library while parsing and lexing.
//! Used with `anyhow` to provide context to errors.

use crate::files::Files;
use crate::token::{ReportableSpan, Span, TokenKind};
use anyhow::{anyhow, Result};
use ariadne::{Color, Label, Report, ReportKind};
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
    #[error("Expected token: {expected}, found: `{found}`")]
    ExpectedToken {
        expected: TokenKind,
        found: TokenKind,
        span: Span,
    },
    // Was expecting any of the tokens, found another
    #[error(
        "Expected any of the tokens: {}, found: `{found}`",
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
    files: &'a mut Files,
}

impl LangError {
    /// Produces a diagnostic for the error
    pub fn diagnostic(&self, file: String) -> Report<ReportableSpan> {
        let span = self.span(file);

        let label = Label::new(span.clone())
            .with_message(self.to_string())
            .with_color(Color::Red);

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

    fn any_tokens_display(tokens: &[TokenKind]) -> String {
        let mut s = String::new();
        for (i, token) in tokens.iter().enumerate() {
            s.push_str(&format!("`{}`", token));
            if i < tokens.len() - 1 {
                s.push_str(", ");
            }
        }
        s
    }
}

impl<'a> ErrorReporter<'a> {
    pub fn new(files: &'a mut Files) -> Self {
        Self { files }
    }

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
