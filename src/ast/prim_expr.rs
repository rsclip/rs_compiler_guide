use crate::errors::SemanticError;
use crate::semantic_analysis::{Analysis, SymbolTable};
use crate::{ast::*, token::Span};
use anyhow::{anyhow, Error, Result};
use log::warn;

#[derive(Debug, Clone)]
pub struct Ident {
    pub ident: String,
    pub span: Span,
}

#[derive(Debug)]
pub enum PrimaryExpression {
    Literal(Literal),
    Ident(Ident),
    Parenthesized(Box<Expression>),
    FunctionCall(Ident, Vec<Expression>),
}

#[derive(Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum LiteralKind {
    Int(i32),
    Float(f32),
    Bool(bool),
}

impl PrettyPrint for PrimaryExpression {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            PrimaryExpression::Literal(l) => l.pretty_print(indent),
            PrimaryExpression::Ident(i) => {
                format!("{:indent$}Ident {}\n", "", i.ident, indent = indent * 4)
            }
            PrimaryExpression::Parenthesized(p) => format!(
                "{:indent$}Parenthesized\n{}\n",
                "",
                p.pretty_print(indent + 1),
                indent = indent * 4
            ),
            PrimaryExpression::FunctionCall(i, args) => format!(
                "{:indent$}FunctionCall {}({})\n",
                "",
                i.ident,
                args.iter()
                    .map(|a| a.pretty_print(indent))
                    .collect::<Vec<String>>()
                    .join(", "),
                indent = indent * 4
            ),
        }
    }
}

impl PrettyPrint for Literal {
    fn pretty_print(&self, indent: usize) -> String {
        match self.kind {
            LiteralKind::Int(i) => format!("{:indent$}Int {}\n", "", i, indent = indent * 4),
            LiteralKind::Float(f) => format!("{:indent$}Float {}\n", "", f, indent = indent * 4),
            LiteralKind::Bool(b) => format!("{:indent$}Bool {}\n", "", b, indent = indent * 4),
        }
    }
}

impl PrettyPrint for Ident {
    fn pretty_print(&self, _indent: usize) -> String {
        self.ident.clone()
    }
}

impl std::hash::Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Eq for Ident {}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.ident)
    }
}

impl ASTSpan for PrimaryExpression {
    fn span(&self) -> Span {
        match self {
            PrimaryExpression::Literal(l) => l.span.clone(),
            PrimaryExpression::Ident(i) => i.span.clone(),
            PrimaryExpression::Parenthesized(p) => p.span(),
            PrimaryExpression::FunctionCall(i, _) => i.span.clone(),
        }
    }
}

impl PrimaryExpression {
    fn analyze_fn_call(&self, table: &mut SymbolTable) -> Vec<Error> {
        log::debug!("Analyzing function call: {:?}, table: {:?}", self, table);
        let mut errors = Vec::new();

        let (ident, args) = match self {
            PrimaryExpression::FunctionCall(i, a) => (i, a),
            _ => unreachable!("analyze_fn_call called on non-FunctionCall variant"),
        };

        // can we find the function in the symbol table?
        if let Some(func) = table.get_fn(&ident) {
            // check if the number of arguments match
            if func.params.len() != args.len() {
                warn!("Function call argument count mismatch: {:?}, {:?}", func, args);
                errors.push(anyhow!(SemanticError::ArgumentCountMismatch {
                    expected: func.params.len(),
                    found: args.len(),
                    call_span: ident.span.clone(),
                    decl_span: func.span.clone(),
                }));
            } else {
                // check if the types of the arguments match
                for (param_ty, arg) in func.params.iter().zip(args) {
                    log::debug!("Checking param {:?} against arg {:?}", param_ty, arg);
                    let arg_ty: Type = match arg.get_type(table) {
                        Ok(ty) => ty,
                        Err(e) => {
                            errors.push(e);
                            continue;
                        }
                    };

                    if *param_ty != arg_ty {
                        warn!("Function call argument type mismatch: {:?}, {:?}", param_ty, arg_ty);
                        errors.push(anyhow!(SemanticError::TypesDoNotMatch {
                            expected_type: param_ty.clone(),
                            expected_span: param_ty.span(),
                            found_type: arg_ty,
                            found_span: arg.span(),
                        }));
                    }
                }
            }
        } else {
            errors.push(anyhow!(SemanticError::FunctionNotDeclared(
                ident.clone(),
                ident.span.clone()
            )));
        }

        log::debug!("Function call analysis errors: {:?}", errors);

        errors
    }

    pub fn get_type(&self, table: &SymbolTable) -> Result<Type> {
        match self {
            PrimaryExpression::Literal(l) => Ok(l.get_type()),
            PrimaryExpression::Ident(i) => {
                if let Some(var) = table.get_var(&i) {
                    Ok(var.ty.clone())
                } else {
                    eprintln!("Variable not declared: {:?} for table {:#?}", i, table);
                    Err(anyhow!(SemanticError::VariableNotDeclared(i.clone(), i.span.clone())))
                }
            }
            PrimaryExpression::Parenthesized(p) => p.get_type(table),
            PrimaryExpression::FunctionCall(i, _) => {
                if let Some(func) = table.get_fn(&i) {
                    Ok(func.ret_ty.clone())
                } else {
                    Err(anyhow!(SemanticError::FunctionNotDeclared(i.clone(), i.span.clone())))
                }
            }
        }
    }
}

impl Analysis for PrimaryExpression {
    fn analyze(&self, _table: &mut SymbolTable) -> Vec<Error> {
        match self {
            PrimaryExpression::Literal(_) => vec![],
            PrimaryExpression::Ident(_) => vec![],
            PrimaryExpression::Parenthesized(p) => p.analyze(_table),
            PrimaryExpression::FunctionCall(i, args) => self.analyze_fn_call(_table),
        }
    }
}

impl Literal {
    fn get_type(&self) -> Type {
        match self.kind {
            LiteralKind::Int(_) => Type::Primitive(PrimitiveType {
                kind: PrimitiveKind::Int,
                span: self.span.clone(),
            }),
            LiteralKind::Float(_) => Type::Primitive(PrimitiveType {
                kind: PrimitiveKind::Float,
                span: self.span.clone(),
            }),
            LiteralKind::Bool(_) => Type::Primitive(PrimitiveType {
                kind: PrimitiveKind::Bool,
                span: self.span.clone(),
            }),
        }
    }
}