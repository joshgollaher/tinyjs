use std::cell::RefCell;
use std::rc::Rc;
use crate::parser::{BinaryOperator, Expression, Literal, Statement, UnaryOperator, AST};
use crate::runtime::builtins::Builtins;
use crate::runtime::scope::Scope;

pub struct Interpreter {
    pub scope: Scope,
    builtins: Builtins,
    ast: AST
}

impl Interpreter {
    pub(crate) fn new(ast: AST) -> Self {
        Self {
            scope: Scope::new(),
            builtins: Builtins::new(),
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
                    BinaryOperator::Add => {
                        match (left, right) {
                            (Literal::Number(l), Literal::Number(r)) => Literal::Number(l + r),
                            (Literal::String(l), Literal::String(r)) => Literal::String(l + &r),
                            (Literal::String(l), Literal::Number(r)) => Literal::String(format!("{}{}", l, r)),
                            (Literal::Number(l), Literal::String(r)) => Literal::String(format!("{}{}", l, r)),
                            (l, r) => panic!("Unsupported operands for Add: {:?} and {:?}", l, r),
                        }
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
                    BinaryOperator::Mod => {
                        let left = match left {
                            Literal::Number(left) => left,
                            _ => panic!("Expected number, got {:?}", left)
                        };
                        let right = match right {
                            Literal::Number(right) => right,
                            _ => panic!("Expected number, got {:?}", right)
                        };

                        Literal::Number(left % right)
                    }
                }
            },
            Expression::Array {
                elements
            } => {
                Literal::Array(Rc::new(RefCell::new(elements.iter().map(|el| self.do_expression(*el.clone()).into() ).collect())))
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
                                let arr = match arr {
                                    Literal::Array(arr) => arr,
                                    _ => panic!("Expected array, got {:?}", arr)
                                };
                                let index = self.do_expression(*index).clone();
                                let index = match index {
                                    Literal::Number(index) => index as usize,
                                    _ => panic!("Expected number, got {:?}", index)
                                };

                                arr.borrow_mut()[index] = res.into();
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
                callee,
                args
            } => {
                let func = self.do_expression(*callee);

                match func {
                    Literal::Function {
                        args: func_args,
                        body
                    } => {
                        if func_args.len() != args.len() {
                            panic!("Expected {} arguments, got {}", func_args.len(), args.len());
                        }

                        self.scope.enter();

                        for (param_name, param_expr) in func_args.iter().zip(args.iter()) {
                            let val = self.do_expression(*param_expr.clone());
                            self.scope.set(param_name.clone(), val);
                        }

                        let ret = self.do_statement(*body);

                        self.scope.exit();

                        ret.unwrap_or(Literal::Undefined)
                    },
                    Literal::NativeFunction(f) => {
                        let args = args.into_iter().map(|arg| self.do_expression(*arg).into()).collect::<Vec<_>>();

                        *(f.func)(args)
                    }
                    _ => panic!("Expected function, got {:?}", func)
                }
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

                if index >= arr.borrow().len() {
                    panic!("Index out of bounds: {index}");
                }
                *arr.borrow()[index].clone()
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
                }
            },
            Expression::Property {
                target,
                name
            } => {
                let target = self.do_expression(*target);
                match target {
                    Literal::Object(properties) => {
                        let mut output = Literal::Undefined;
                        for (prop_name, val) in properties {
                            if *prop_name == name {
                                output = *val.clone();
                                break;
                            }
                        }

                        output
                    },
                    Literal::Array(arr) => {
                        let func = self.builtins.array_builtin(
                            Literal::Array(arr).into(),
                            name.clone()
                        );

                        *func
                    },
                    Literal::String(str) => {
                        let func = self.builtins.string_builtin(
                            Literal::String(str).into(),
                            name.clone()
                        );

                        *func
                    },
                    Literal::Number(n) => {
                        let func = self.builtins.number_builtin(
                            Literal::Number(n).into(),
                            name.clone()
                        );

                        *func
                    },
                    _ => panic!("Expected object, got {:?}", target)
                }
            }
        }
    }

    fn do_statement(&mut self, stmt: Statement) -> Option<Literal> {
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
                    self.do_statement(*init);
                };

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

                self.scope.exit();
            }
            Statement::Scope {
                statements
            } => {
                self.scope.enter();
                for stmt in statements {
                    let res = self.do_statement(stmt);
                    if res.is_some() {
                        self.scope.exit();
                        return res;
                    }
                }
                self.scope.exit()
            }
            Statement::If {
                condition,
                alternative,
                consequence,
            } => {
                if self.do_expression(*condition).truthy() {
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
                // FIXME: Right now we don't verify that this is in a function.
                let val = self.do_expression(*expr);
                return Some(val);
            }
            Statement::While {
                condition,
                body
            } => {
                while self.do_expression(*condition.clone()).truthy() {
                    self.do_statement(*body.clone());
                }
            }
        }

        None
    }

    pub fn run(&mut self) {
        let stmts = self.ast.statements.iter().cloned().collect::<Vec<_>>();
        self.builtins.load(&mut self.scope);

        for stmt in stmts {
            self.do_statement(stmt);
        }
    }
}