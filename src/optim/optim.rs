use std::collections::HashMap;
use log::trace;
use crate::parser::{BinaryOperator, Expression, Literal, Statement, UnaryOperator, AST};

#[derive(Clone)]
#[derive(Debug)]
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
                    trace!("Propagating constant: {id} = {saved_const:?}");
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

    fn fold_statement(&mut self, stmt: Statement) -> Statement {
        match stmt {
            Statement::Expression(expr) => Statement::Expression(self.fold_expression(*expr).into()),
            Statement::Return(expr) => Statement::Return(self.fold_expression(*expr).into()),
            Statement::If { condition, consequence, alternative } => Statement::If { condition: self.fold_expression(*condition).into(), consequence: self.fold_statement(*consequence).into(), alternative: alternative.map(|alt| self.fold_statement(*alt.clone()).into()) },
            Statement::While { condition, body } => Statement::While { condition: self.fold_expression(*condition).into(), body: self.fold_statement(*body).into() },
            Statement::For { init, condition, update, body } => {
                Statement::For {
                    init: init.map(|init| self.fold_statement(*init.clone()).into()),
                    condition: condition.map(|condition| self.fold_expression(*condition.clone()).into()),
                    update: update.map(|update| self.fold_expression(*update.clone()).into()),
                    body: self.fold_statement(*body).into(),
                }
            }
            Statement::Function { name, args, body } => Statement::Function { name, args, body: self.fold_statement(*body).into() },
            Statement::Scope { statements } => {
                let statements = statements.into_iter().map(|stmt| self.fold_statement(stmt)).collect();
                Statement::Scope { statements }.into()
            }
            Statement::Let { name, value } => Statement::Let { name, value: self.fold_expression(*value).into() },
        }
    }

    fn fold_expression(&mut self, expr: Expression) -> Expression {
        match expr {
            e @  Expression::Literal(..) => e,
            e @ Expression::Identifier(..) => e,
            Expression::Object { properties } => {
                Expression::Object {
                    properties: properties.into_iter().map(|(k, v)| (k, self.fold_expression(*v).into())).collect(),
                }
            },
            Expression::Array { elements } => Expression::Array { elements: elements.into_iter().map(|el| self.fold_expression(*el).into()).collect() },
            Expression::BinaryOp { left, op, right } => {
                match (*left.clone(), op.clone(), *right.clone()) {
                    (Expression::Literal(Literal::Number(l)), BinaryOperator::Add, Expression::Literal(Literal::Number(r))) => {
                        trace!("Folding {l} + {r} into {}", l + r);
                        Expression::Literal(Literal::Number(l + r))
                    },
                    (Expression::Literal(Literal::String(l)), BinaryOperator::Add, Expression::Literal(Literal::String(r))) => {
                        trace!("Folding '{l}' + '{r}' into '{}'", l.clone() + r.as_str());
                        Expression::Literal(Literal::String(l.clone() + r.as_str()))
                    },
                    (Expression::Literal(Literal::Number(l)), BinaryOperator::Sub, Expression::Literal(Literal::Number(r))) => {
                        trace!("Folding {l} - {r} into {}", l + r);
                        Expression::Literal(Literal::Number(l - r))
                    },
                    (Expression::Literal(Literal::Number(l)), BinaryOperator::Mul, Expression::Literal(Literal::Number(r))) => {
                        trace!("Folding {l} * {r} into {}", l * r);
                        Expression::Literal(Literal::Number(l * r))
                    },
                    (Expression::Literal(Literal::Number(l)), BinaryOperator::Div, Expression::Literal(Literal::Number(r))) => {
                        trace!("Folding {l} / {r} into {}", l / r);
                        Expression::Literal(Literal::Number(l / r))
                    },
                    (Expression::Literal(Literal::Number(l)), BinaryOperator::Mod, Expression::Literal(Literal::Number(r))) => {
                        trace!("Folding {l} % {r} into {}", l % r);
                        Expression::Literal(Literal::Number(l % r))
                    },
                    _ => Expression::BinaryOp { left: self.fold_expression(*left.clone()).into(), op: op.clone(), right: self.fold_expression(*right.clone()).into() },
                }
            },
            Expression::UnaryOp { op, expr } => {
                match (op.clone(), *expr.clone()) {
                    (UnaryOperator::Negate, Expression::Literal(Literal::Number(n))) => {
                        trace!("Folding -{n} into {}", -n);
                        Expression::Literal(Literal::Number(-n))
                    },
                    (UnaryOperator::Not, Expression::Literal(Literal::Boolean(b))) => {
                        trace!("Folding !{b} into {}", !b);
                        Expression::Literal(Literal::Boolean(!b))
                    },
                    _ => Expression::UnaryOp { op: op.clone(), expr: self.fold_expression(*expr.clone()).into()}
                }
            },
            Expression::FunctionCall { callee, args } => {
                Expression::FunctionCall { callee: self.fold_expression(*callee).into(), args: args.into_iter().map(|arg| self.fold_expression(*arg).into()).collect() }
            },
            Expression::Assignment { target, value } => Expression::Assignment { target, value: self.fold_expression(*value).into() },
            Expression::Index { target, index } => Expression::Index { target, index: self.fold_expression(*index).into() },
            e @ Expression::Property { .. } => e,
        }
    }

    fn constant_folding(&mut self) {
        let mut stmts = self.ast.statements.clone();
        stmts = stmts.into_iter().map(|stmt| self.fold_statement(stmt)).collect();

        self.ast.statements = stmts;
    }

    fn valid_loop_body(&self, body: &Statement) -> bool {
        true
    }

    fn unroll_statement(&mut self, stmt: Statement) -> Statement {
        match stmt {
            Statement::While { condition, body } => {
                let mut stmts = vec![];

                Statement::Scope { statements: stmts }.into()
            },
            Statement::For { init, condition, update, body } => {
                let mut stmts = vec![];

                Statement::Scope { statements: stmts }.into()
            }
            e @ Statement::Expression(_) => e,
            e @ Statement::Return(_) => e,
            Statement::If { condition, consequence, alternative } => Statement::If { condition, consequence: self.unroll_statement(*consequence).into(), alternative: alternative.map(|alt| self.unroll_statement(*alt.clone()).into()) },
            Statement::Function { name, args, body } => Statement::Function { name, args, body: self.unroll_statement(*body).into() },
            Statement::Scope { statements } => Statement::Scope { statements: statements.into_iter().map(|stmt| self.unroll_statement(stmt)).collect() },
            e @ Statement::Let { .. } => e,
        }
    }

    fn loop_unrolling(&mut self) {
        // Conditions
        // - Loop bounds and increment are known AOT
        // - No variables inside body, we're not doing substitution.

        let mut stmts = self.ast.statements.clone();
        stmts = stmts.into_iter().map(|stmt| self.unroll_statement(stmt)).collect();

        self.ast.statements = stmts;
    }

    fn tree_shaking(&mut self) {

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