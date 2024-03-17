//! Represents the AST of the language,
//! in accordance with the grammar defined in `grammar.bnf`.
#![allow(dead_code)]

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
pub enum BinaryExpression {
    Operation(Box<Expression>, Operator, Box<Expression>),
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
