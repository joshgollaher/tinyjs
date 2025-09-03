use crate::parser::{BinaryOperator, Expression, Literal, Statement, UnaryOperator, AST};
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

    fn do_expression(&mut self, expr: Expression) -> Literal {
        match expr {
            Expression::Identifier(name) => self.scope.get(name.clone()).expect(format!("Unknown identifier '{}'", name.clone()).as_str()).clone(),
            Expression::Literal(lit) => lit,
            Expression::BinaryOp {
                left,
                op,
                right
            } => {
                let left = self.do_expression(*left);
                let right = self.do_expression(*right);

                match op {
                    BinaryOperator::Add => { // FIXME: Support strings
                        let left = match left {
                            Literal::Number(left) => left,
                            _ => panic!("Expected number, got {:?}", left)
                        };
                        let right = match right {
                            Literal::Number(right) => right,
                            _ => panic!("Expected number, got {:?}", right)
                        };

                        Literal::Number(left + right)
                    },
                    BinaryOperator::Sub => {
                        let left = match left {
                            Literal::Number(left) => left,
                            _ => panic!("Expected number, got {:?}", left)
                        };
                        let right = match right {
                            Literal::Number(right) => right,
                            _ => panic!("Expected number, got {:?}", right)
                        };

                        Literal::Number(left - right)
                    },
                    BinaryOperator::Mul => {
                        let left = match left {
                            Literal::Number(left) => left,
                            _ => panic!("Expected number, got {:?}", left)
                        };
                        let right = match right {
                            Literal::Number(right) => right,
                            _ => panic!("Expected number, got {:?}", right)
                        };

                        Literal::Number(left * right)
                    },
                    BinaryOperator::Div => {
                        let left = match left {
                            Literal::Number(left) => left,
                            _ => panic!("Expected number, got {:?}", left)
                        };
                        let right = match right {
                            Literal::Number(right) => right,
                            _ => panic!("Expected number, got {:?}", right)
                        };

                        Literal::Number(left / right)
                    },
                    BinaryOperator::Equal => {
                        Literal::Boolean(left == right)
                    },
                    BinaryOperator::NotEqual => {
                        Literal::Boolean(left != right)
                    },
                    BinaryOperator::GreaterThan => {
                        let left = match left {
                            Literal::Number(left) => left,
                            _ => panic!("Expected number, got {:?}", left)
                        };
                        let right = match right {
                            Literal::Number(right) => right,
                            _ => panic!("Expected number, got {:?}", right)
                        };

                        Literal::Boolean(left > right)
                    },
                    BinaryOperator::GreaterThanOrEqual => {
                        let left = match left {
                            Literal::Number(left) => left,
                            _ => panic!("Expected number, got {:?}", left)
                        };
                        let right = match right {
                            Literal::Number(right) => right,
                            _ => panic!("Expected number, got {:?}", right)
                        };

                        Literal::Boolean(left >= right)
                    },
                    BinaryOperator::LessThan => {
                        let left = match left {
                            Literal::Number(left) => left,
                            _ => panic!("Expected number, got {:?}", left)
                        };
                        let right = match right {
                            Literal::Number(right) => right,
                            _ => panic!("Expected number, got {:?}", right)
                        };

                        Literal::Boolean(left < right)
                    }
                    BinaryOperator::LessThanOrEqual => {
                        let left = match left {
                            Literal::Number(left) => left,
                            _ => panic!("Expected number, got {:?}", left)
                        };
                        let right = match right {
                            Literal::Number(right) => right,
                            _ => panic!("Expected number, got {:?}", right)
                        };

                        Literal::Boolean(left <= right)
                    },
                    BinaryOperator::BinaryOr => {
                        let left = left.truthy();
                        let right = right.truthy();

                        Literal::Boolean(left || right)
                    },
                    BinaryOperator::BinaryAnd => {
                        let left = left.truthy();
                        let right = right.truthy();

                        Literal::Boolean(left && right)
                    },
                    _ => panic!("Unknown binary operator: {:?}", op)
                }
            },
            Expression::Array {
                elements
            } => {},
            Expression::Assignment {
                target,
                value
            } => {

            },
            Expression::FunctionCall {
                name,
                args
            } => {

            },
            Expression::Index {
                target,
                index
            } => {

            },
            Expression::Object {
                properties
            } => {

            },
            Expression::UnaryOp {
                op,
                expr
            } => {
                match op {
                    UnaryOperator::Negate => {
                        let expr = self.do_expression(*expr);
                        match expr {
                            Literal::Number(num) => Literal::Number(-num),
                            _ => panic!("Expected number, got {:?}", expr)
                        }
                    },
                    UnaryOperator::Not => {
                        let expr = self.do_expression(*expr);
                        Literal::Boolean(!expr.truthy())
                    }
                    _ => panic!("Unknown unary operator: {:?}", op)
                }
            },
            _ => panic!("Unknown expression: {:?}", expr)
        }
    }

    fn do_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::For {
                init,
                condition,
                update,
                body
            } => {
                // Enter scope for the for header
                self.scope.enter();
                if let Some(init) = init {
                    self.do_statement(*init)
                }

                loop {
                    if let Some(condition) = &condition {
                        if !self.do_expression(*condition.clone()).truthy() {
                            break;
                        }
                    }

                    self.do_statement(*body.clone());

                    if let Some(update) = &update {
                        self.do_expression(*update.clone());
                    }
                }

            }
            Statement::Scope {
                statements
            } => {
                self.scope.enter();
                for stmt in statements {
                    self.do_statement(stmt);
                }
                self.scope.exit()
            }
            Statement::If {
                condition,
                alternative,
                consequence,
            } => {
                if(self.do_expression(*condition).truthy()) {
                    self.do_statement(*consequence);
                } else if let Some(alternative) = alternative {
                    self.do_statement(*alternative);
                }
            }
            Statement::Function {
                name,
                args,
                body
            } => {
                todo!();
            }
            Statement::Expression(expr) => {
                self.do_expression(*expr);
            }
            Statement::Let {
                name,
                value
            } => {
                let res = self.do_expression(*value);
                self.scope.set(name, res);
            }
            Statement::Return(expr) => {

            }
            Statement::While {
                condition,
                body
            } => {
                while self.do_expression(*condition.clone()).truthy() {
                    self.do_statement(*body.clone());
                }
            }

            stmt => panic!("Unknown statement: {:?}", stmt)
        }
    }

    pub fn run(&mut self) {
        let stmts = self.ast.statements.iter().cloned().collect::<Vec<_>>();

        for stmt in stmts {
            self.do_statement(stmt);
        }
    }
}