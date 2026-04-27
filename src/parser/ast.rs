use std::cell::RefCell;
use std::rc::Rc;
use crate::lexer::Token;
use crate::parser::parser::Parser;
use crate::runtime::Scope;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    BinaryAnd,
    BinaryOr,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    NotEqual,
    Mod
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    PlusEqual,
    MinusEqual,
    MulEqual,
    DivEqual,
    ModEqual,  // TODO: Implement this
}

impl AssignmentOperator {
    pub fn associated_binary_op(&self) -> BinaryOperator {
        match self {
            AssignmentOperator::PlusEqual => BinaryOperator::Add,
            AssignmentOperator::MinusEqual => BinaryOperator::Sub,
            AssignmentOperator::MulEqual => BinaryOperator::Mul,
            AssignmentOperator::DivEqual => BinaryOperator::Div,
            AssignmentOperator::ModEqual => BinaryOperator::Mod
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Clone)]
pub struct NativeFn {
    pub func: Rc<dyn Fn(Vec<Box<Literal>>) -> Box<Literal>>,
    name: String,
}

impl NativeFn {
    pub fn new(name: String, func: Rc<dyn Fn(Vec<Box<Literal>>) -> Box<Literal>>) -> Self {
        Self { func, name }
    }
}

impl std::fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Native Function [{}]", self.name)
    }
}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Null,
    Boolean(bool),
    Undefined,
    Array(Rc<RefCell<Vec<Box<Literal>>>>),
    Object(Vec<(String, Box<Literal>)>),
    Class {
        scope: Scope,
        members: Vec<(String, Box<Literal>)>,
    },
    Function {
        args: Vec<String>,
        body: Box<Statement>
    },
    Instance {
        members: Vec<(String, Box<Literal>)>,
    },
    NativeFunction(NativeFn)
}

impl Literal {
    pub(crate) fn truthy(&self) -> bool {
        match self {
            Literal::Number(n) => *n != 0. && !(*n).is_nan(),
            Literal::String(s) => s.len() > 0,
            Literal::Null => false,
            Literal::Boolean(b) => *b,
            Literal::Undefined => false,
            Literal::Array(a) => {
                !a.borrow().is_empty()
            },
            Literal::Instance { .. } => true,
            Literal::Class {..} => true,
            Literal::Object(o) => !o.is_empty(),
            Literal::Function { .. } => true,
            Literal::NativeFunction(_) => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Object {
        properties: Vec<(String, Box<Expression>)>,
    },
    Array {
        elements: Vec<Box<Expression>>,
    },
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
        callee: Box<Expression>,
        args: Vec<Box<Expression>>,
    },
    Assignment {
        target: Box<Expression>,
        value: Box<Expression>,
        op: Option<AssignmentOperator>,
    },
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
    },
    Property {
        target: Box<Expression>,
        name: String,
    },
    Increment {
        target: Box<Expression>,
    },
    Decrement {
        target: Box<Expression>,
    },
    New {
        class: String,
        args: Vec<Expression>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Box<Expression>),
    Return(Box<Expression>),
    Continue,
    Break,
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
        update: Option<Box<Expression>>,
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
    Let {
        name: String,
        value: Box<Expression>,
    },
    Class {
        name: String,
        methods: Vec<Statement>,
        scope: Scope,
    },
}

#[derive(Debug, Clone)]
pub struct AST {
    pub statements: Vec<Statement>,
}

impl AST {
    pub fn new(tokens: Vec<Token>) -> Self {
        if tokens.is_empty() {
            Self { statements: vec![] }
        } else {
            let mut parser = Parser::new(tokens);
            parser.parse()
        }
    }
}
