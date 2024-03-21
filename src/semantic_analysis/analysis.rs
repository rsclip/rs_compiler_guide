//! This module implements Semantic Analysis on an existing AST.
//!
//! Errors:
//! - Scope checking                Idents must be declared before use
//! - Type checking                 Return types, etc.
//! - Variable redeclaration         Variables cannot be redeclared
//! - Function redeclaration        Functions cannot be redeclared
//! - Control flow checks           Return statements, etc. breaks cannot be outside loops
//! - Missing main function
//!
//! Warnings:
//! - Dead code (unused anything)
//! - Unreachable code

use super::symbols::SymbolTable;
use super::traits::Analysis;
use crate::ast::*;
use crate::errors::SemanticError;
use crate::token::Span;
use anyhow::{anyhow, Error};

pub fn analyse<'a>(ast: &'a AST) -> Vec<Error> {
    let program = &ast.program;

    let mut global_table = SymbolTable::new();
    let mut errors = Vec::new();

    // recognise all functions
    let mut main_node: Option<&FunctionDecl> = None;

    for item in &program.items {
        match item {
            Item::FunctionDecl(f) => {
                if f.ident.ident == "main" {
                    main_node = Some(f);
                }

                if let Err(e) = global_table.add_fn(f) {
                    errors.push(e);
                }
            }
        }
    }

    if main_node.is_none() {
        errors.push(anyhow!(SemanticError::MissingMainFunction(Span::default())));
    } else {
        // ensure return type is int
        let ret_ty = main_node.unwrap().ty.clone();
        match ret_ty {
            Type::Primitive(ty) => {
                if ty.kind != PrimitiveKind::Int {
                    errors.push(anyhow!(SemanticError::MainMustReturnInt(ty.span.clone())));
                }
            },
        }
    }

    // let AST analyse itself
    errors.extend(ast.analyze(&mut global_table));

    errors
}

/// These tests are inexact
/// Errors may be unrelated but it will still pass
#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    fn quick_parse(input: &str) -> AST {
        let lexer = crate::lexer::Lexer::new(input);
        let (tokens, err) = crate::lexer::consume_lexer(lexer);

        assert!(err.is_empty());

        let mut parser = crate::parser::Parser::new(tokens);
        match parser.parse("test".to_string()) {
            Ok(ast) => ast,
            Err(e) => {
                quick_errors(&vec![e], input);
                panic!("Parser error");
            }
        }
    }

    /// Print errors
    fn quick_errors(errors: &Vec<Error>, src: &str) {
        let mut files = crate::files::Files::new();
        files.add_file("test".to_string(), src.to_string());

        let mut reporter = crate::errors::ErrorReporter::new(&mut files);
        for err in errors {
            reporter.report("test".to_string(), &err).unwrap();
        }
    }

    #[test]
    fn test_missing_main() {
        let src = "fn foo() -> int {return 1;}";
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0].to_string(), "Missing `main` function");
    }

    #[test]
    fn test_main_found() {
        let src = "fn main() -> int {return 0;}";
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_duplicate_main() {
        let src = "fn main() -> int {return 0;}\n\nfn main() -> int {return 1;}";
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0].to_string(), "Function `main` already declared");
    }

    #[test]
    fn test_duplicate_var() {
        let src = "fn main() -> int { let x: int = 5; let x: int = 10; return y; }";
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0].to_string(), "Variable `x` already declared");
    }

    #[test]
    fn test_no_return() {
        let src = "fn main() -> int {}";
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0].to_string(), "Missing return statement");
    }

    #[test]
    fn test_if() {
        let src = "fn main() -> int {if 1 == 1 { return 5; }}";
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert!(errors.len() > 0);
    }

    #[test]
    fn undeclared_var() {
        let src = "fn main() -> int { let x:int = 3; return x; }";
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);
    }

    #[test]
    fn incompatible_return_type() {
        let src = "fn main() -> int {\n\treturn 5;\n}\nfn foo() -> bool {\n\treturn 5;\n}";
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0].to_string(), "Incompatible return type");
    }

    #[test]
    fn incompatible_return_type_var() {
        let src = "fn main() -> int {\n\treturn 5;\n}\nfn foo() -> bool {\n\tlet x: int = 5;\n\treturn x;\n}";
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0].to_string(), "Incompatible return type");
    }

    #[test]
    fn return_unguaranteed() {
        // should fail since there is no return statement if condition is false
        let src = r#"fn main() -> int {
            if 1 == 1 {
                return 5;
            }
        }"#;
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0].to_string(), "Return not guaranteed in all branches");
    }

    #[test]
    fn return_unguaranteed_2() {
        // should pass since all paths have a return statement
        let src = r#"fn main() -> int {
            if 1 == 1 {
                return 5;
            } else {
                return 6;
            }
        }"#;
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn return_unguaranteed_3() {
        // should fail since there is no return statement if condition is false
        let src = r#"fn main() -> int {
            if 1 == 1 {
                return 5;
            } else {
            }
        }"#;
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0].to_string(), "Return not guaranteed in all branches");
    }

    #[test]
    fn return_unguaranteed_4() {
        // should pass
        let src = r#"fn main() -> int {
            if 1 == 1 {
                return 5;
            }

            return 6;
        }"#;
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn return_unguaranteed_5() {
        // should pass
        let src = r#"fn main() -> int {
            if 1 == 1 {
                if 2 == 2 {
                    return 5;
                } else {
                    return 6;
                }
            } else {
                return 7;
            }
        }"#;
        let ast = quick_parse(src);
        let errors = analyse(&ast);

        quick_errors(&errors, src);

        assert_eq!(errors.len(), 0);
    }
}
