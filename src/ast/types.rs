use crate::ast::*;

#[derive(Debug)]
pub enum Type {
    Primitive(PrimitiveType),
}

#[derive(Debug)]
pub enum PrimitiveType {
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
        match self {
            PrimitiveType::Int => "int".to_string(),
            PrimitiveType::Float => "float".to_string(),
            PrimitiveType::Bool => "bool".to_string(),
        }
    }
}
