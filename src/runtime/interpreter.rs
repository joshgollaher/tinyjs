use crate::parser::AST;
use crate::runtime::scope::Scope;

pub struct Interpreter {
    scope: Scope,
    ast: AST
}

impl Interpreter {
    pub(crate) fn new(ast: AST) -> Self {
        Self {
            scope: Scope::new(),
            ast
        }
    }

    pub fn run(&mut self) {

    }
}