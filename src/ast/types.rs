use crate::{ast::*, token::Span};

#[derive(Debug, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
}

#[derive(Debug, Clone)]
pub struct PrimitiveType {
    pub kind: PrimitiveKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum PrimitiveKind {
    Int,
    Float,
    Bool,
}

impl PrettyPrint for Type {
    fn pretty_print(&self, _indent: usize) -> String {
        match self {
            Type::Primitive(p) => p.pretty_print(0),
        }
    }
}

impl PrettyPrint for PrimitiveType {
    fn pretty_print(&self, _indent: usize) -> String {
        match self.kind {
            PrimitiveKind::Int => "int".to_string(),
            PrimitiveKind::Float => "float".to_string(),
            PrimitiveKind::Bool => "bool".to_string(),
        }
    }
}

impl ASTSpan for Type {
    fn span(&self) -> Span {
        match self {
            Type::Primitive(p) => p.span.clone(),
        }
    }
}