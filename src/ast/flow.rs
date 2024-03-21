use crate::{ast::*, token::Span};

#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    VariableDecl(VariableDecl),
    Flow(FlowStatement),
    Return(Option<Box<Expression>>),
}

#[derive(Debug)]
pub struct VariableDecl {
    pub ident: Ident,
    pub ty: Type,
    pub expression: Expression,
    pub span: Span,
}

#[derive(Debug)]
pub struct FlowStatement {
    pub condition: Expression,
    pub if_block: Block,
    pub else_block: Option<Block>,
    pub span: Span,
}

impl PrettyPrint for Statement {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Statement::Expression(e) => e.pretty_print(indent),
            Statement::VariableDecl(v) => v.pretty_print(indent),
            Statement::Flow(f) => f.pretty_print(indent),
            Statement::Return(e) => format!(
                "{:indent$}Return\n{}",
                "",
                e.as_ref()
                    .map(|e| e.pretty_print(indent + 1))
                    .unwrap_or_else(|| "".to_string()),
                indent = indent * 4
            ),
        }
    }
}

impl PrettyPrint for VariableDecl {
    // format: "VariableDecl ident: ty =\n" + expression
    fn pretty_print(&self, indent: usize) -> String {
        format!(
            "{:indent$}VariableDecl {}:{} =\n{}",
            "",
            self.ident.ident,
            self.ty.pretty_print(0),
            self.expression.pretty_print(indent + 1),
            indent = indent * 4
        )
    }
}

impl PrettyPrint for FlowStatement {
    // format: "FlowStatement\ncond=\n" + condition + "if=\n" + if_block + "else=\n" + else_block
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = format!("{:indent$}FlowStatement\n", "", indent = indent * 4);
        s.push_str(&self.condition.pretty_print(indent + 1));
        s.push_str(&self.if_block.pretty_print(indent + 1));
        if let Some(else_block) = &self.else_block {
            s.push_str(&else_block.pretty_print(indent + 1));
        }
        s
    }
}
