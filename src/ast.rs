//! Represents the AST of the language,
//! in accordance with the grammar defined in `grammar.bnf`.

pub enum Program {
    Item(Vec<Item>),
}

pub enum Item {
    FunctionDecl(FunctionDecl),
}

pub enum Type {
    Primitive(PrimitiveType),
}

pub enum Statement {
    Expression(Box<Expression>),
    VariableDecl(VariableDecl),
    Flow(FlowStatement),
}

pub enum Expression {
    Primary(PrimaryExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

pub struct FunctionDecl {
    identifier: String,
    parameters: Vec<Parameter>,
    return_type: Type,
    block: Block,
}

pub struct Parameter {
    identifier: String,
    param_type: Type,
}

pub struct VariableDecl {
    identifier: String,
    var_type: Type,
    expression: Expression,
}

pub struct FlowStatement {
    condition: Expression,
    if_block: Block,
    else_block: Option<Block>,
}

pub struct Block {
    statements: Vec<Statement>,
}

pub struct FunctionType {
    parameters: Vec<Type>,
    return_type: Type,
}

pub enum PrimitiveType {
    Int,
    Float,
    Bool,
}

pub enum PrimaryExpression {
    Literal(Literal),
    Identifier(String),
    Parenthesized(Box<Expression>),
    FunctionCall(String, Vec<Expression>),
}

pub enum Literal {
    Int(i32),
    Float(f32),
    Boolean(bool),
}

pub enum UnaryExpression {
    Negation(Box<Expression>),
    Not(Box<Expression>),
}

pub enum BinaryExpression {
    Operation(Box<Expression>, Operator, Box<Expression>),
}

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
