use crate::ast::*;

#[derive(Debug)]
pub enum UnaryExpression {
    Negation(Box<Expression>),
    Not(Box<Expression>),
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
}

impl PrettyPrint for UnaryExpression {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            UnaryExpression::Negation(e) => format!(
                "{:indent$}Negation\n{}\n",
                "",
                e.pretty_print(indent + 1),
                indent = indent * 4
            ),
            UnaryExpression::Not(e) => format!(
                "{:indent$}Not\n{}\n",
                "",
                e.pretty_print(indent + 1),
                indent = indent * 4
            ),
        }
    }
}

impl PrettyPrint for BinaryExpression {
    // format: "BinaryExpression\nop=\n" + op + "lhs=\n" + lhs + "rhs=\n" + rhs
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = format!("{:indent$}BinaryExpression\n", "", indent = indent * 4);
        s.push_str(&format!(
            "{:indent$}op={}\n",
            "",
            self.op.pretty_print(indent + 1),
            indent = indent * 4
        ));
        s.push_str(&self.lhs.pretty_print(indent + 1));
        s.push_str(&self.rhs.pretty_print(indent + 1));
        s
    }
}
