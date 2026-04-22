use std::collections::HashMap;
use crate::parser::{Expression, Literal, Statement, AST};
use crate::runtime::bytecode::{Block, Op, Operand};

struct Emitter {
    ast: AST,
    reg_index: usize,
    buffer: Vec<Block>,
    buffer_index: usize,
    identifier_reg_map: HashMap<String, usize>,
}

impl Emitter {

    fn new(ast: AST) -> Self {
        Self {
            ast,
            reg_index: 0,
            buffer: vec![ Block::empty() ],
            buffer_index: 0,
        }
    }

    fn emit(&mut self, op: Op) {
        self.buffer[self.buffer_index].push(op);
    }

    fn emit_label(&mut self) {
        self.buffer.push(Block::empty());
        self.buffer_index += 1
    }

    fn next_reg(&mut self) -> usize {
        self.reg_index += 1;
        self.reg_index
    }

    fn register_from_identifier(&self, id: String) -> usize {
        *self.identifier_reg_map.get(id.as_str()).unwrap()
    }

    fn expression(&mut self, expr: Expression) -> usize {
        match expr {
            Expression::Literal(lit) => {
                let reg = self.next_reg();

                self.emit(Op::Mov {
                    from: Operand::Constant(
                        match lit {
                            Literal::Number(n) => Value::Number(n)
                            Literal::String(s) => {}
                            Literal::Null => {}
                            Literal::Boolean(b) => {}
                            Literal::Undefined => {}
                            Literal::Array(arr) => {}
                            Literal::Object(obj) => {}
                            Literal::Function { args, body } => {}
                            Literal::NativeFunction(func) => {}
                        }
                    ),
                    to: Operand::Global(reg);
                });

                reg
            }
            Expression::Identifier(_) => {}
            Expression::Object { .. } => {}
            Expression::Array { .. } => {}
            Expression::BinaryOp { .. } => {}
            Expression::UnaryOp { .. } => {}
            Expression::FunctionCall { .. } => {}
            Expression::Assignment { .. } => {}
            Expression::Index { .. } => {}
            Expression::Property { .. } => {}
            Expression::Increment { .. } => {}
            Expression::Decrement { .. } => {}
        };

        0
    }

    fn statement(&mut self, stmt: Statement) -> Block {
        let block = Block::empty();

        match stmt {
            Statement::Expression(ex) => {
                self.expression(*ex);
            }
            Statement::Return(_) => {}
            Statement::Continue => {}
            Statement::Break => {}
            Statement::If { .. } => {}
            Statement::While { .. } => {}
            Statement::For { .. } => {}
            Statement::Function { .. } => {}
            Statement::Scope { .. } => {}
            Statement::Let { .. } => {}
        };

        block
    }

    pub fn run(self) -> Vec<Block> {
        self.buffer
    }
}