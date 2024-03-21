use crate::token::Span;

pub trait PrettyPrint {
    fn pretty_print(&self, indent: usize) -> String;
}

pub trait ASTSpan {
    fn span(&self) -> Span;
}