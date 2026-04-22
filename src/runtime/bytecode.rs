use std::collections::HashMap;
use crate::parser::{Literal, NativeFn, Statement, AST};

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Undefined,
    Function(usize),
    Object(HashMap<String, Box<Value>>),
    Native(NativeFn)
}

#[derive(Clone)]
pub enum Operand {
    Immediate(usize),
    Global(usize),
    Constant(Value),
}

#[derive(Clone)]
pub enum Op {
    Mov { from: Operand, to: Operand },
    Store(Operand),
    Load(Operand),

    Add { from: Operand, to: Operand },
    Sub { from: Operand, to: Operand },
    Mul { from: Operand, to: Operand },
    Div { from: Operand, to: Operand },
    Mod { from: Operand, to: Operand },

    CmpEq(Operand),
    CmpNe(Operand),
    CmpLt(Operand),
    CmpLe(Operand),
    CmpGt(Operand),
    CmpGe(Operand),

    Jmp(Operand),
    JmpIfFalse(Operand),
    Ret(Option<Operand>),
}

pub struct Block {
    ops: Vec<Op>,
}

impl Block {
    fn new(ops: Vec<Op>) -> Self {
        Self {
            ops
        }
    }

    pub(crate) fn empty() -> Self { Self::new(vec![]) }

    pub fn push(&mut self, op: Op) {
        self.ops.push(op);
    }
}

pub struct Bytecode {
    block: Block
}

// FIXME: Use SSA, use graph-coloring register allocation.
impl Bytecode {
    fn emit_statement(stmt: Statement) -> Vec<Op> {
        match stmt {
            Statement::Expression(_) => vec![],
            Statement::Return(_) => vec![],
            Statement::Continue => vec![],
            Statement::Break => vec![],
            Statement::If { .. } => vec![],
            Statement::While { .. } => vec![],
            Statement::For { .. } => vec![],
            Statement::Function { .. } => vec![],
            Statement::Class { .. } => vec![],
            Statement::Scope { statements } => {
                // Emit all statements inside scope.
                let mut ops = statements.iter().map(|s| Bytecode::emit_statement(s.clone())).collect::<Vec<_>>().iter().flatten().cloned().collect::<Vec<_>>();
                ops
            }
            Statement::Let { .. } => vec![],
        }
    }

    pub fn from_ast(ast: &AST) -> Self {
        Self {
            block: Block { ops: vec![] }
        }
    }
}