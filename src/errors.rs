//! Contains the error types used in the library while parsing and lexing.
//! Used with `anyhow` to provide context to errors.

use crate::ast::{Ident, Type};
use crate::files::Files;
use crate::token::{ReportableSpan, Span, TokenKind};
use anyhow::{anyhow, Result};
use ariadne::{Color, Label, Report, ReportKind};
use thiserror::Error;

/// Main colour preset
pub const PRIM_COLOR: Color = Color::Red;
pub const SEC_COLOR: Color = Color::Cyan;
pub const TERT_COLOR: Color = Color::Yellow;

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
#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Missing `main` function")]
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
    NonBooleanCondition { found_type: Type, found_span: Span },

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
    },
}

#[derive(Debug, Error)]
pub enum Warning {
    #[error("Unused variable")]
    UnusedVariable(Ident, Span),

    #[error("Unused function")]
    UnusedFunction(Ident, Span),

    #[error("Unreachable code")]
    UnreachableCode(Span),
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
            .with_color(PRIM_COLOR);

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

        let mut builder = Report::build(ReportKind::Error, file, span.start)
            .with_message(self.to_string())
            .with_labels(labels);

        if let Some(help) = self.help() {
            builder = builder.with_help(help);
        }

        builder.finish()
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
                SemanticError::IncompatibleReturnType { found_span, .. } => found_span,
                SemanticError::ReturnNotGuaranteed(span) => span,
                SemanticError::TypesDoNotMatch { found_span, .. } => found_span,
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
                    .with_message("No main function declared")
                    .with_color(PRIM_COLOR)]
            }
            SemanticError::FunctionAlreadyDeclared(ref name, ref span, ref existing) => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("Tried to declare {name} here"))
                        .with_color(PRIM_COLOR),
                    Label::new(ReportableSpan::new(file.clone(), existing))
                        .with_message("Already declared here")
                        .with_color(SEC_COLOR),
                ]
            }
            SemanticError::VariableAlreadyDeclared(ref name, ref span, ref existing) => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("Variable `{}` already declared", name))
                        .with_color(PRIM_COLOR),
                    Label::new(ReportableSpan::new(file.clone(), existing))
                        .with_message("First declared here")
                        .with_color(SEC_COLOR),
                ]
            }
            SemanticError::MainMustReturnInt(ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message("Does not return an integer literal")
                    .with_color(PRIM_COLOR)]
            }
            SemanticError::MissingReturnStatement(ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message("Missing return statement")
                    .with_color(PRIM_COLOR)]
            }
            SemanticError::IncompatibleReturnType {
                expected_type,
                expected_span,
                found_span,
                found_type,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), found_span))
                        .with_message(format!("found {found_type} instead of {expected_type}"))
                        .with_color(PRIM_COLOR),
                    Label::new(ReportableSpan::new(file.clone(), expected_span))
                        .with_message(format!("expected {expected_type} return type"))
                        .with_color(SEC_COLOR),
                ]
            }
            SemanticError::ReturnNotGuaranteed(ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message("Return not guaranteed in all branches")
                    .with_color(PRIM_COLOR)]
            }
            SemanticError::TypesDoNotMatch {
                expected_type,
                expected_span,
                found_type,
                found_span,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), found_span))
                        .with_message(format!(
                            "found type {found_type} instead of {expected_type}"
                        ))
                        .with_color(PRIM_COLOR),
                    Label::new(ReportableSpan::new(file.clone(), expected_span))
                        .with_message(format!("expected type {expected_type}"))
                        .with_color(SEC_COLOR),
                ]
            }
            SemanticError::NonBooleanCondition {
                found_type,
                found_span,
            } => {
                vec![Label::new(ReportableSpan::new(file.clone(), found_span))
                    .with_message(format!("evaluates to {found_type}"))
                    .with_color(PRIM_COLOR)]
            }
            SemanticError::FunctionNotDeclared(ref name, ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message(format!("Function `{}` has not been declared yet", name))
                    .with_color(PRIM_COLOR)]
            }
            SemanticError::VariableNotDeclared(ref name, ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message(format!("Variable `{}` has not been declared yet", name))
                    .with_color(PRIM_COLOR)]
            }
            SemanticError::ArgumentCountMismatch {
                expected,
                found,
                call_span,
                decl_span,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), call_span))
                        .with_message(format!("got {found} arguments"))
                        .with_color(PRIM_COLOR),
                    Label::new(ReportableSpan::new(file.clone(), decl_span))
                        .with_message(format!("expected {expected} arguments"))
                        .with_color(SEC_COLOR),
                ]
            }
            SemanticError::UnsupportedUnaryOperation {
                operator,
                operand_type,
                span,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message("unsupported unary operation for {operator}")
                        .with_color(PRIM_COLOR),
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("can't apply {operator} to {operand_type}"))
                        .with_color(SEC_COLOR),
                ]
            }
            SemanticError::UnsupportedBinaryOperation {
                operator,
                operand_type,
                span,
            } => {
                vec![
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message("unsupported binary operation for {operator}")
                        .with_color(PRIM_COLOR),
                    Label::new(ReportableSpan::new(file.clone(), span))
                        .with_message(format!("can't apply {operator} to {operand_type}"))
                        .with_color(SEC_COLOR),
                ]
            }
        }
    }

    /// Optional help text
    fn help(&self) -> Option<String> {
        match self {
            SemanticError::MissingMainFunction(_) => {
                Some("consider declaring a main function".to_string())
            }
            SemanticError::MissingReturnStatement(_) => {
                Some("consider adding a reachable return statement".to_string())
            }
            SemanticError::ReturnNotGuaranteed(_) => {
                Some("make sure all possible paths return a value".to_string())
            }
            _ => None,
        }
    }
}

impl Warning {
    pub fn diagnostic(&self, file: String) -> Report<ReportableSpan> {
        let span = self.first_span(&file);

        let labels = self.get_labels(&file);

        let mut builder = Report::build(ReportKind::Warning, file, span.start)
            .with_message(self.to_string())
            .with_labels(labels);

        if let Some(help) = self.help() {
            builder = builder.with_help(help);
        }

        builder.finish()
    }

    fn help(&self) -> Option<String> {
        match self {
            Warning::UnusedVariable(ident, _) => Some(format!("if intended, prefix with an underscore: `_{ident}`")),
            _ => None,
        }
    }

    /// First span of error
    pub fn first_span(&self, file: &String) -> ReportableSpan {
        ReportableSpan::new(
            file.clone(),
            match self {
                Warning::UnusedVariable(_, span) => span,
                Warning::UnusedFunction(_, span) => span,
                Warning::UnreachableCode(span) => span,
            },
        )
    }

    fn get_labels(&self, file: &String) -> Vec<Label<ReportableSpan>> {
        match self {
            Warning::UnusedVariable(ref name, ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message(format!("{name} is never used"))
                    .with_color(TERT_COLOR)]
            }
            Warning::UnusedFunction(ref name, ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message(format!("{name} is never called"))
                    .with_color(TERT_COLOR)]
            }
            Warning::UnreachableCode(ref span) => {
                vec![Label::new(ReportableSpan::new(file.clone(), span))
                    .with_message("code is unreachable")
                    .with_color(TERT_COLOR)]
            }
        }
    }
}

impl<'a> ErrorReporter<'a> {
    pub fn new(files: &'a mut Files) -> Self {
        Self { files }
    }

    pub fn report(&mut self, file: String, error: &anyhow::Error) -> Result<()> {
        let diagnostic = if let Some(e) = error.downcast_ref::<LangError>() {
            e.diagnostic(file)
        } else if let Some(e) = error.downcast_ref::<SemanticError>() {
            e.diagnostic(file)
        } else if let Some(e) = error.downcast_ref::<Warning>() {
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
