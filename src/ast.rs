//! Represents the AST of the language,
//! in accordance with the grammar defined in `grammar.bnf`.
#![allow(dead_code)]

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

#[derive(Debug)]
pub enum Type {
    Primitive(PrimitiveType),
}

#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    VariableDecl(VariableDecl),
    Flow(FlowStatement),
}

#[derive(Debug)]
pub enum Expression {
    Primary(PrimaryExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub ident: String,
    pub parameters: Vec<Parameter>,
    pub ty: Type,
    pub block: Block,
}

#[derive(Debug)]
pub struct Parameter {
    pub ident: String,
    pub ty: Type,
}

#[derive(Debug)]
pub struct VariableDecl {
    pub ident: String,
    pub ty: Type,
    pub expression: Expression,
}

#[derive(Debug)]
pub struct FlowStatement {
    pub condition: Expression,
    pub if_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub ty: Type,
}

#[derive(Debug)]
pub enum PrimitiveType {
    Int,
    Float,
    Bool,
}

#[derive(Debug)]
pub enum PrimaryExpression {
    Literal(Literal),
    Ident(String),
    Parenthesized(Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Bool(bool),
}

#[derive(Debug)]
pub enum UnaryExpression {
    Negation(Box<Expression>),
    Not(Box<Expression>),
}

#[derive(Debug)]
pub struct BinaryExpression {
    lhs: Box<Expression>,
    op: Operator,
    rhs: Box<Expression>,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
}

pub trait PrettyPrint {
    fn pretty_print(&self, indent: usize) -> String;
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

impl PrettyPrint for FunctionDecl {
    // format: FuncDecl ident(parameters) -> ty { block }
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = format!(
            "{:indent$}FuncDecl {}({}) -> {} {{\n",
            "",
            self.ident,
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
        format!("{}: {}", self.ident, self.ty.pretty_print(0))
    }
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

impl PrettyPrint for Block {
    // "Block\n" + statements
    fn pretty_print(&self, indent: usize) -> String {
        let mut s = format!("{:indent$}Block\n", "", indent = indent * 4);
        for statement in &self.statements {
            s.push_str(&statement.pretty_print(indent + 1));
        }
        s
    }
}

impl PrettyPrint for Statement {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Statement::Expression(e) => e.pretty_print(indent),
            Statement::VariableDecl(v) => v.pretty_print(indent),
            Statement::Flow(f) => f.pretty_print(indent),
        }
    }
}

impl PrettyPrint for VariableDecl {
    // format: "VariableDecl ident: ty =\n" + expression
    fn pretty_print(&self, indent: usize) -> String {
        format!(
            "{:indent$}VariableDecl {}:{} =\n{}",
            "",
            self.ident,
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

impl PrettyPrint for Expression {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            Expression::Primary(p) => p.pretty_print(indent),
            Expression::Unary(u) => u.pretty_print(indent),
            Expression::Binary(b) => b.pretty_print(indent),
        }
    }
}

impl PrettyPrint for PrimaryExpression {
    fn pretty_print(&self, indent: usize) -> String {
        match self {
            PrimaryExpression::Literal(l) => l.pretty_print(indent),
            PrimaryExpression::Ident(i) => {
                format!("{:indent$}Ident {}\n", "", i, indent = indent * 4)
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
                i,
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
        match self {
            Literal::Int(i) => format!("{:indent$}Int {}\n", "", i, indent = indent * 4),
            Literal::Float(f) => format!("{:indent$}Float {}\n", "", f, indent = indent * 4),
            Literal::Bool(b) => format!("{:indent$}Bool {}\n", "", b, indent = indent * 4),
        }
    }
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

impl PrettyPrint for Operator {
    fn pretty_print(&self, _indent: usize) -> String {
        match self {
            Operator::Add => "Add".to_string(),
            Operator::Subtract => "Subtract".to_string(),
            Operator::Multiply => "Multiply".to_string(),
            Operator::Divide => "Divide".to_string(),
            Operator::Modulus => "Modulus".to_string(),
            Operator::Equal => "Equal".to_string(),
            Operator::NotEqual => "NotEqual".to_string(),
            Operator::LessThan => "LessThan".to_string(),
            Operator::GreaterThan => "GreaterThan".to_string(),
            Operator::LessThanOrEqual => "LessThanOrEqual".to_string(),
            Operator::GreaterThanOrEqual => "GreaterThanOrEqual".to_string(),
            Operator::And => "And".to_string(),
            Operator::Or => "Or".to_string(),
        }
    }
}
impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print(0))
    }
}