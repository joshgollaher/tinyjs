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
            } => {
                Literal::Array(elements.iter().map(|el| self.do_expression(*el.clone()).into() ).collect())
            },
            Expression::Assignment {
                target,
                value
            } => {
                match *target {
                    Expression::Identifier(name) => {
                        let res = self.do_expression(*value);
                        self.scope.set(name, res.clone());
                        res
                    },
                    Expression::Index {
                        target,
                        index
                    } => {
                        match *target {
                            Expression::Identifier(name) => {
                                let res = self.do_expression(*value);
                                let arr = self.scope.get(name.clone()).expect(format!("Unknown identifier '{}'", name.clone()).as_str()).clone();
                                let mut arr = match arr {
                                    Literal::Array(arr) => arr,
                                    _ => panic!("Expected array, got {:?}", arr)
                                };
                                let index = self.do_expression(*index).clone();
                                let index = match index {
                                    Literal::Number(index) => index as usize,
                                    _ => panic!("Expected number, got {:?}", index)
                                };

                                arr[index] = res.into();
                                self.scope.set(name.clone(), Literal::Array(arr.clone()));
                                Literal::Array(arr)
                            },
                            _ => panic!("Expected identifier, got {:?}", target)
                        }
                    },
                    Expression::Property {
                        target,
                        name
                    } => {
                        match *target {
                            Expression::Identifier(obj_name) => {
                                let res = self.do_expression(*value);
                                let obj = self.scope.get(obj_name.clone()).expect(format!("Unknown identifier '{}'", obj_name.clone()).as_str()).clone();
                                let mut obj = match obj {
                                    Literal::Object(obj) => obj,
                                    _ => panic!("Expected object, got {:?}", obj)
                                };

                                for i in 0..obj.len() {
                                    if obj[i].0 == name {
                                        obj[i] = (name.clone(), res.clone().into());
                                        self.scope.set(obj_name.clone(), Literal::Object(obj.clone()));
                                        return Literal::Object(obj);
                                    }
                                }

                                // Not found, add it
                                obj.push((name.clone(), res.clone().into()));
                                self.scope.set(obj_name.clone(), Literal::Object(obj.clone()));
                                res
                            }
                            _ => panic!("Expected identifier, got {:?}", target)
                        }
                    },
                    _ => panic!("Expected identifier, got {:?}", target)
                }
            },
            Expression::FunctionCall {
                name,
                args
            } => {
                todo!()
            },
            Expression::Index {
                target,
                index
            } => {
                let index = self.do_expression(*index);
                let target = match *target {
                    Expression::Identifier(name) => self.scope.get(name.clone()).expect(format!("Unknown identifier '{}'", name.clone()).as_str()).clone(),
                    _ => panic!("Expected identifier, got {:?}", target)
                };
                let index = match index {
                    Literal::Number(index) => index as usize,
                    _ => panic!("Expected number, got {:?}", index)
                };

                let arr = match target {
                    Literal::Array(arr) => arr,
                    _ => panic!("Expected array, got {:?}", target)
                };

                if index >= arr.len() || index < 0 {
                    panic!("Index out of bounds: {index}");
                }
                *arr[index].clone()
            },
            Expression::Object {
                properties
            } => {
                Literal::Object(properties.into_iter().map(|(name, val)| {
                    (name, self.do_expression(*val).into())
                }).collect())
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
                self.scope.set(name, Literal::Function {
                    args,
                    body
                });
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
                todo!();
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