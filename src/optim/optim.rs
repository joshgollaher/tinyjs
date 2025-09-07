use std::collections::HashMap;
use crate::parser::{Expression, Literal, Statement, AST};

#[derive(Clone)]
enum ConstVal {
    StringLiteral(String),
    Number(f64),
    Boolean(bool),
}

impl ConstVal {
    pub fn into_literal(self) -> Literal {
        match self {
            ConstVal::StringLiteral(s) => Literal::String(s),
            ConstVal::Number(n) => Literal::Number(n),
            ConstVal::Boolean(b) => Literal::Boolean(b),
        }
    }

    pub fn into_expression(self) -> Expression {
        Expression::Literal(self.into_literal())
    }
}

pub struct Optimizer {
    ast: AST,
    constants: Vec<HashMap<String, ConstVal>>,
}

impl Optimizer {
    pub(crate) fn new(ast: AST) -> Self {
        Self {
            ast,
            constants: vec![HashMap::new()],
        }
    }

    fn mark_constant(&mut self, name: String, value: ConstVal) {
        self.constants.last_mut().unwrap().insert(name, value);
    }

    fn get_constant(&self, name: &str) -> Option<ConstVal> {
        for scope in self.constants.iter().cloned().rev() {
            if scope.contains_key(name) {
                return Some(scope.get(name).unwrap().clone());
            }
        }

        None
    }

    fn enter(&mut self) {
        self.constants.push(HashMap::new());
    }

    fn exit(&mut self) {
        self.constants.pop();
    }

    fn constant_value_propagation(&mut self) {
        let mut stmts = self.ast.statements.clone();
        stmts = stmts.into_iter().map(|stmt| self.propagate_statement(stmt)).collect();

        self.ast.statements = stmts;
    }

    fn propagate_expression(&mut self, expr: Expression) -> Expression {
        match expr {
            Expression::Literal(l) => Expression::Literal(l),
            Expression::Identifier(id) => {
                if let Some(saved_const) = self.get_constant(id.as_str()) {
                    saved_const.into_expression()
                } else {
                    Expression::Identifier(id)
                }
            }
            Expression::Object { properties } => {
                let properties = properties.into_iter().map(|(k, v)| (k, self.propagate_expression(*v).into())).collect();

                Expression::Object { properties }
            },
            Expression::Array { elements } => {
                let elements = elements.into_iter().map(|el| self.propagate_expression(*el).into()).collect();

                Expression::Array { elements }
            }
            Expression::BinaryOp { left, op, right } => {
                Expression::BinaryOp { left: self.propagate_expression(*left).into(), op, right: self.propagate_expression(*right).into() }
            },
            Expression::UnaryOp { op, expr } => {
                Expression::UnaryOp { op, expr: self.propagate_expression(*expr).into() }
            },
            e @ Expression::FunctionCall { .. } => e,
            Expression::Assignment { target, value } => {
                Expression::Assignment { target, value: self.propagate_expression(*value).into() }
            },
            e @ Expression::Index { .. } => e,
            e @ Expression::Property { .. } => e,
        }
    }

    fn propagate_statement(&mut self, stmt: Statement) -> Statement {
        match stmt {
            Statement::Expression(ex) => {
                Statement::Expression(self.propagate_expression(*ex).into()).into()
            },
            Statement::Return(ex) => {
                let ret = Statement::Return(self.propagate_expression(*ex).into()).into();

                ret
            },
            Statement::If { condition, consequence, alternative } => {
                let condition = self.propagate_expression(*condition);
                let consequence = self.propagate_statement(*consequence);
                let alternative = alternative.map(|alt| self.propagate_statement(*alt.clone()).into());

                Statement::If { condition: condition.into(), consequence: consequence.into(), alternative }
            },
            Statement::While { condition, body } => {
                let condition = self.propagate_expression(*condition);
                let body = self.propagate_statement(*body).into();

                Statement::While { condition: condition.into(), body }
            },
            Statement::For { init, condition, update, body } => {
                let init = init.map(|init| self.propagate_statement(*init.clone()).into());
                let condition = condition.map(|condition| self.propagate_expression(*condition.clone()).into());
                let update = update.map(|update| self.propagate_expression(*update.clone()).into());
                let body = self.propagate_statement(*body).into();

                Statement::For { init, condition, update, body }
            },
            Statement::Function { name, args, body } => {
                Statement::Function { name, args, body: self.propagate_statement(*body).into() }
            },
            Statement::Scope { statements } => {
                self.enter();
                let statements = statements.into_iter().map(|stmt| self.propagate_statement(stmt)).collect();
                self.exit();

                Statement::Scope { statements }.into()
            },
            Statement::Let { name, value } => {
                let expr = self.propagate_expression(*value);
                match expr.clone() {
                    Expression::Literal(l) => {
                        match l {
                            Literal::Number(n) => {
                                self.mark_constant(name.clone(), ConstVal::Number(n));
                            }
                            Literal::String(s) => {
                                self.mark_constant(name.clone(), ConstVal::StringLiteral(s));
                            }
                            Literal::Boolean(b) => {
                                self.mark_constant(name.clone(), ConstVal::Boolean(b));
                            }
                            _ => {}
                        }
                    },
                    _ => {}
                };

                Statement::Let { name, value: expr.into() }
            }
        }
    }

    fn constant_folding(&mut self) {

    }

    fn tree_shaking(&mut self) {

    }

    fn loop_unrolling(&mut self) {
        // Conditions
        // - Loop bounds and increment are known AOT
        // - No variables inside body, we're not doing substitution.
    }

    pub fn optimize(&mut self) -> AST {
        // Propagate then fold, otherwise we might miss some opportunities.
        self.constant_value_propagation();
        self.constant_folding();

        self.tree_shaking();
        self.loop_unrolling();

        self.ast.clone()
    }
}