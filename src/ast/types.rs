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

#[derive(Debug, Clone, PartialEq)]
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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print(0))
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Primitive(p1), Type::Primitive(p2)) => p1 == p2,
        }
    }
}

impl PartialEq for PrimitiveType {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
