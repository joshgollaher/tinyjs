use crate::parser::parser::Parser;
use crate::token::Token;

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    BinaryAnd,
    BinaryOr,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Null,
    Boolean(bool),
    Undefined,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Assignment {
        name: String,
        value: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    Return(Box<Expression>),
    If {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    While {
        condition: Box<Expression>,
        body: Box<Statement>,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Box<Expression>>,
        update: Option<Box<Statement>>,
        body: Box<Statement>,
    },
    Function {
        name: String,
        args: Vec<String>,
        body: Box<Statement>,
    },
    Scope {
        statements: Vec<Statement>,
    },
}

#[derive(Debug)]
pub struct AST {
    pub statements: Vec<Statement>,
}

impl AST {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        let mut parser = Parser::new(tokens);
        parser.parse()
    }
}
