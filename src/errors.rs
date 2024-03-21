//! Contains the error types used in the library while parsing and lexing.
//! Used with `anyhow` to provide context to errors.

use crate::ast::{Ident, Type};
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

/// Errors for Semantic Analysis
#[derive(Debug, Error, HelpMessage)]
pub enum SemanticError {
    #[error("Missing `main` function")]
    #[help = "Consider declaring a main function"]
    MissingMainFunction(Span),

    /// 2 spans for function declaration, and the existing declaration
    #[error("Function `{0}` already declared")]
    FunctionAlreadyDeclared(Ident, Span, Span),

    #[error("Variable `{0}` already declared")]
    VariableAlreadyDeclared(Ident, Span, Span),

    #[error("`main` must return an integer")]
    MainMustReturnInt(Span),

    #[error("Missing return statement")]
    MissingReturnStatement(Span),

    /// Span for the return statement, and the expected return type
    #[error("Incompatible return type")]
    IncompatibleReturnType {
        expected_type: Type,
        expected_span: Span,
        found_type: Type,
        found_span: Span,
    },

    #[error("Return not guaranteed in all branches")]
    ReturnNotGuaranteed(Span),

    #[error("Types do not match")]
    TypesDoNotMatch {
        expected_type: Type,
        expected_span: Span,
        found_type: Type,
        found_span: Span,
    },

    #[error("Condition must be a boolean")]
    NonBooleanCondition {
        found_type: Type,
        found_span: Span,
    },

    #[error("Function `{0}` has not been declared yet")]
    FunctionNotDeclared(Ident, Span),

    #[error("Variable `{0}` has not been declared yet")]
    VariableNotDeclared(Ident, Span),

    #[error("Argument count mismatch")]
    ArgumentCountMismatch {
        expected: usize,
        found: usize,
        call_span: Span, // fn call
        decl_span: Span, // fn decl
    },

    // unsupported unary operator
    #[error("Unsupported unary operation")]
    UnsupportedUnaryOperation {
        operator: String,
        operand_type: Type,
        span: Span,
    },

    #[error("Unsupported binary operation")]
    UnsupportedBinaryOperation {
        operator: String,
        operand_type: Type,
        span: Span,
    }
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

impl SemanticError {
    pub fn diagnostic(&self, file: String) -> Report<ReportableSpan> {
        let span = self.first_span(&file);

        let labels = self.get_labels(&file);

        Report::build(ReportKind::Error, file, span.start)
            .with_message(self.to_string())
            .with_labels(labels)
            .finish()
    }

    /// First span of error
    pub fn first_span(&self, file: &String) -> ReportableSpan {
        ReportableSpan::new(
            file.clone(),
            match self {
                SemanticError::MissingMainFunction(span) => span,
                SemanticError::FunctionAlreadyDeclared(_, span, _) => span,
                SemanticError::VariableAlreadyDeclared(_, span, _) => span,
                SemanticError::MainMustReturnInt(span) => span,
                SemanticError::MissingReturnStatement(span) => span,
                SemanticError::IncompatibleReturnType {
                    found_span,
                    ..
                } => found_span,
                SemanticError::ReturnNotGuaranteed(span) => span,
                SemanticError::TypesDoNotMatch {
                    found_span,
                    ..
                } => found_span,
                SemanticError::NonBooleanCondition { found_span, .. } => found_span,
                SemanticError::FunctionNotDeclared(_, span) => span,
                SemanticError::VariableNotDeclared(_, span) => span,
                SemanticError::ArgumentCountMismatch { call_span, .. } => call_span,
                SemanticError::UnsupportedUnaryOperation { span, .. } => span,
                SemanticError::UnsupportedBinaryOperation { span, .. } => span,
            },
        )
    }

    fn get_labels(&self, file: &String) -> Vec<Label<ReportableSpan>> {
        match self {
            SemanticError::MissingMainFunction(ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message("Missing `main` function")
                    .with_color(Color::Red)]
            }
            SemanticError::FunctionAlreadyDeclared(ref name, ref span, ref existing) => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("Function `{}` already declared", name))
                        .with_color(Color::Red),
                    Label::new(ReportableSpan::new(file.clone(), existing))
                        .with_message("First declared here")
                        .with_color(Color::Yellow),
                ]
            }
            SemanticError::VariableAlreadyDeclared(ref name, ref span, ref existing) => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("Variable `{}` already declared", name))
                        .with_color(Color::Red),
                    Label::new(ReportableSpan::new(file.clone(), existing))
                        .with_message("First declared here")
                        .with_color(Color::Yellow),
                ]
            }
            SemanticError::MainMustReturnInt(ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message("`main` must return an integer")
                    .with_color(Color::Red)]
            }
            SemanticError::MissingReturnStatement(ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message("Missing return statement")
                    .with_color(Color::Red)]
            }
            SemanticError::IncompatibleReturnType {
                expected_type,
                expected_span,
                found_type,
                found_span,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), found_span))
                        .with_message("Incompatible return type")
                        .with_color(Color::Red),
                    Label::new(ReportableSpan::new(file.clone(), expected_span))
                        .with_message(format!("Expected `{}`", expected_type))
                        .with_color(Color::Yellow),
                ]
            },
            SemanticError::ReturnNotGuaranteed(ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message("Return not guaranteed in all branches")
                    .with_color(Color::Red)]
            }
            SemanticError::TypesDoNotMatch {
                expected_type,
                expected_span,
                found_type,
                found_span,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), found_span))
                        .with_message("Types do not match")
                        .with_color(Color::Red),
                    Label::new(ReportableSpan::new(file.clone(), expected_span))
                        .with_message(format!("Expected `{}`", expected_type))
                        .with_color(Color::Yellow),
                ]
            }
            SemanticError::NonBooleanCondition {
                found_type,
                found_span,
            } => {
                vec![Label::new(ReportableSpan::new(file.clone(), found_span))
                    .with_message("Condition must be a boolean")
                    .with_color(Color::Red)]
            }
            SemanticError::FunctionNotDeclared(ref name, ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message(format!("Function `{}` has not been declared yet", name))
                    .with_color(Color::Red)]
            }
            SemanticError::VariableNotDeclared(ref name, ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message(format!("Variable `{}` has not been declared yet", name))
                    .with_color(Color::Red)]
            }
            SemanticError::ArgumentCountMismatch {
                expected,
                found,
                call_span,
                decl_span,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), call_span))
                        .with_message(format!(
                            "Expected {} arguments, found {}",
                            expected, found
                        ))
                        .with_color(Color::Red),
                    Label::new(ReportableSpan::new(file.clone(), decl_span))
                        .with_message("Function declared here")
                        .with_color(Color::Yellow),
                ]
            }
            SemanticError::UnsupportedUnaryOperation {
                operator,
                operand_type,
                span,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message("Unsupported unary operation")
                        .with_color(Color::Red),
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("Operator: `{}`", operator))
                        .with_color(Color::Yellow),
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("Operand type: `{}`", operand_type))
                        .with_color(Color::Yellow),
                ]
            }
            SemanticError::UnsupportedBinaryOperation {
                operator,
                operand_type,
                span,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message("Unsupported binary operation")
                        .with_color(Color::Red),
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("Operator: `{}`", operator))
                        .with_color(Color::Yellow),
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("Operand type: `{}`", operand_type))
                        .with_color(Color::Yellow),
                ]
            }
        }
    }
}

impl<'a> ErrorReporter<'a> {
    pub fn new(files: &'a mut Files) -> Self {
        Self { files }
    }

    pub fn report(&mut self, file: String, error: &anyhow::Error) -> Result<()> {
        // try to downcast to either LangError or SemanticError
        let diagnostic = if let Some(e) = error.downcast_ref::<LangError>() {
            e.diagnostic(file)
        } else if let Some(e) = error.downcast_ref::<SemanticError>() {
            e.diagnostic(file)
        } else {
            return Err(anyhow!("Unknown error while downcasting: {}", error));
        };

        match diagnostic.eprint(&mut self.files) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("Failed to print diagnostic: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_test() {
        let err = SemanticError::MissingMainFunction(Span::default());
        assert_eq!(err.help(), "Consider declaring a main function");
    }
}