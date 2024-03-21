use crate::{ast::*, token::Span};

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: Ident,
    pub parameters: Vec<Parameter>,
    pub ty: Type,
    pub block: Block,
    pub span: Span,
}

#[derive(Debug)]
pub struct Parameter {
    pub ident: Ident,
    pub ty: Type,
    pub span: Span,
}

impl PrettyPrint for FunctionDecl {
    // format: FuncDecl ident(parameters) -> ty { block }
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = format!(
            "{:indent$}FuncDecl {}({}) -> {} {{\n",
            "",
            self.ident.ident,
            self.parameters
                .iter()
                .map(|p| p.pretty_print(indent))
                .collect::<Vec<String>>()
                .join(", "),
            self.ty.pretty_print(indent),
            indent = indent * 4
        );
        s.push_str(&self.block.pretty_print(indent + 1));
        s.push_str(&format!("{:indent$}}}\n", "", indent = indent * 4));
        s
    }
}

impl PrettyPrint for Parameter {
    // format: ident: ty
    fn pretty_print(&self, _indent: usize) -> String {
        format!("{}: {}", self.ident.ident, self.ty.pretty_print(0))
    }
}
