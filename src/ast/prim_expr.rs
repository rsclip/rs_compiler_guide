use crate::{ast::*, token::Span};

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