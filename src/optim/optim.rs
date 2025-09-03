use crate::parser::AST;

pub struct Optimizer {
    ast: AST
}

impl Optimizer {
    pub(crate) fn new(ast: AST) -> Self {
        Self {
            ast
        }
    }

    pub fn optimize(&mut self) -> AST {
        self.ast.clone()
    }
}