use crate::ast::*;

pub fn pretty_print_ast(ast: &AST) {
    println!("{}", ast.pretty_print(0));
}

#[derive(Debug)]
pub struct AST {
    pub program: Program,
    pub file_id: usize,
}

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    FunctionDecl(FunctionDecl),
}

impl PrettyPrint for AST {
    // no extra indents for the AST
    fn pretty_print(&self, indent: usize) -> String {
        self.program.pretty_print(indent)
    }
}

impl PrettyPrint for Program {
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = String::from("Program\n");
        for item in &self.items {
            s.push_str(&item.pretty_print(indent + 1));
        }
        s
    }
}

impl PrettyPrint for Item {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Item::FunctionDecl(f) => f.pretty_print(indent),
        }
    }
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print(0))
    }
}
