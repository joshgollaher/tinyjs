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
    Expression(Expression),
}

#[derive(Debug)]
pub struct AST {
    pub statements: Vec<Statement>,
}

impl AST {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Self { statements: vec![] }
    }
}
