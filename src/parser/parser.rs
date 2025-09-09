use crate::parser::{AST, BinaryOperator, Expression, Literal, Statement, UnaryOperator};
use crate::lexer::Token;
use std::cmp::PartialEq;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn done(&self) -> bool {
        self.tokens[self.pos] == Token::EOF || self.pos >= self.tokens.len()
    }

    fn peek(&self) -> Token {
        self.peek_by(0)
    }

    fn peek_by(&self, offset: usize) -> Token {
        self.tokens[self.pos + offset].clone()
    }

    fn consume(&mut self) -> Token {
        let token = self.peek();
        self.pos += 1;
        token
    }

    fn expect(&mut self, token: Token) {
        if self.peek() == token {
            self.consume();
        } else {
            panic!("Expected {:?}", token);
        }
    }

    fn do_if(&mut self) -> Statement {
        self.consume(); // if
        self.expect(Token::LeftParen);
        let condition = self.expression();
        self.expect(Token::RightParen);

        let consequence = self.statement();

        let alternative = if self.peek() == Token::Else {
            self.consume();
            Some(self.statement())
        } else {
            None
        };

        Statement::If {
            condition: condition.into(),
            consequence: consequence.into(),
            alternative: alternative.map(Box::new),
        }
    }

    fn do_let(&mut self) -> Statement {
        self.expect(Token::Let);

        let name = match self.consume() {
            Token::Identifier(name) => name,
            tok => panic!("Expected identifier after let, got {:?}", tok),
        };

        let value;
        if self.peek() == Token::Semicolon {
            value = Expression::Literal(Literal::Undefined);
        } else {
            self.expect(Token::Equal);
            value = self.expression();
        }
        self.expect(Token::Semicolon);

        Statement::Let {
            name,
            value: value.into(),
        }
    }

    fn do_while(&mut self) -> Statement {
        self.expect(Token::While);
        self.expect(Token::LeftParen);
        let condition = self.expression();
        self.expect(Token::RightParen);
        let body = self.statement();
        Statement::While {
            condition: condition.into(),
            body: body.into(),
        }
    }

    fn do_for(&mut self) -> Statement {
        self.expect(Token::For);
        self.expect(Token::LeftParen);
        let init = if self.peek() != Token::Semicolon {
            Some(self.statement())
        } else {
            self.expect(Token::Semicolon);
            None
        };
        // statement() already handled the semicolon.

        let condition = if self.peek() != Token::Semicolon {
            Some(self.expression())
        } else {
            None
        };
        self.expect(Token::Semicolon);

        let update = if self.peek() != Token::Semicolon {
            Some(self.expression())
        } else {
            None
        };
        self.expect(Token::RightParen);

        let body = self.statement();

        Statement::For {
            init: init.map(Box::new),
            condition: condition.map(Box::new),
            update: update.map(Box::new),
            body: body.into(),
        }
    }

    fn do_function(&mut self) -> Statement {
        self.expect(Token::Function);
        let name = match self.consume() {
            Token::Identifier(name) => name,
            tok => panic!("Expected identifier after function, got {:?}", tok),
        };

        self.expect(Token::LeftParen);
        let mut args = Vec::new();
        if self.peek() != Token::RightParen {
            loop {
                let arg = match self.consume() {
                    Token::Identifier(name) => name,
                    tok => panic!("Expected identifier after function, got {:?}", tok),
                };
                args.push(arg);

                if self.peek() == Token::RightParen {
                    break;
                }
                self.expect(Token::Comma);
            }
        }
        self.expect(Token::RightParen);

        let body = Statement::Scope {
            statements: self.do_scope(),
        };

        Statement::Function {
            name,
            args,
            body: body.into(),
        }
    }

    fn do_scope(&mut self) -> Vec<Statement> {
        self.expect(Token::LeftBrace);
        let mut statements = Vec::new();
        while self.peek() != Token::RightBrace && !self.done() {
            statements.push(self.statement());
        }
        self.expect(Token::RightBrace);

        statements
    }

    // Base case for all statements
    fn statement(&mut self) -> Statement {
        match self.peek() {
            Token::Return => {
                self.consume();

                if self.peek() == Token::Semicolon {
                    Statement::Return(Box::new(Expression::Literal(Literal::Undefined)))
                } else {
                    let expr = self.expression();
                    self.expect(Token::Semicolon);
                    Statement::Return(Box::new(expr))
                }
            }
            Token::If => self.do_if(),
            Token::Let => self.do_let(),
            Token::While => self.do_while(),
            Token::For => self.do_for(),
            Token::Function => self.do_function(),
            Token::Break => {
                self.consume();
                self.expect(Token::Semicolon);
                Statement::Break
            },
            Token::Continue => {
                self.consume();
                self.expect(Token::Semicolon);
                Statement::Continue
            }
            Token::LeftBrace => {
                let statements = self.do_scope();
                Statement::Scope { statements }
            }
            _ => {
                let expr = self.expression();
                self.expect(Token::Semicolon);
                Statement::Expression(Box::new(expr))
            }
        }
    }

    fn do_args(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();
        self.expect(Token::LeftParen);
        if self.peek() != Token::RightParen {
            loop {
                args.push(self.expression());
                if self.peek() == Token::RightParen {
                    break;
                }
                self.expect(Token::Comma);
            }
        }

        args
    }

    fn do_array(&mut self) -> Vec<Expression> {
        let mut elements = Vec::new();
        if self.peek() != Token::RightBracket {
            loop {
                elements.push(self.expression());
                if self.peek() == Token::RightBracket {
                    break;
                }
                self.expect(Token::Comma);
            }
        }

        elements
    }

    fn do_object(&mut self) -> Vec<(String, Box<Expression>)> {
        let mut properties = Vec::new();
        if self.peek() != Token::RightBrace {
            loop {
                let key = match self.consume() {
                    Token::StringLiteral(s) => s,
                    Token::Identifier(s) => s,
                    tok => panic!(
                        "Expected string literal or identifier in object literal, got {:?}",
                        tok
                    ),
                };
                self.expect(Token::Colon);
                let value = self.expression();

                properties.push((key, Box::new(value)));

                if self.peek() == Token::RightBrace {
                    break;
                }

                self.expect(Token::Comma);
            }
        }
        self.expect(Token::RightBrace);

        properties
    }

    fn match_infix_operators(&mut self) -> Option<BinaryOperator> {
        match self.peek() {
            Token::Plus => {
                self.consume();
                Some(BinaryOperator::Add)
            }
            Token::Minus => {
                self.consume();
                Some(BinaryOperator::Sub)
            }
            Token::Star => {
                self.consume();
                Some(BinaryOperator::Mul)
            }
            Token::Slash => {
                self.consume();
                Some(BinaryOperator::Div)
            }
            Token::Percent => {
                self.consume();
                Some(BinaryOperator::Mod)
            }
            Token::AmpAmp => {
                self.consume();
                Some(BinaryOperator::BinaryAnd)
            }
            Token::PipePipe => {
                self.consume();
                Some(BinaryOperator::BinaryOr)
            },
            Token::EqualEqual => {
                self.consume();
                Some(BinaryOperator::Equal)
            },
            Token::BangEqual => {
                self.consume();
                Some(BinaryOperator::NotEqual)
            },
            Token::Greater => {
                self.consume();
                Some(BinaryOperator::GreaterThan)
            },
            Token::GreaterEqual => {
                self.consume();
                Some(BinaryOperator::GreaterThanOrEqual)
            },
            Token::Less => {
                self.consume();
                Some(BinaryOperator::LessThan)
            },
            Token::LessEqual => {
                self.consume();
                Some(BinaryOperator::LessThanOrEqual)
            },
            Token::PlusEqual => {
                self.consume();
                Some(BinaryOperator::PlusEqual)
            },
            Token::MinusEqual => {
                self.consume();
                Some(BinaryOperator::MinusEqual)
            },
            Token::StarEqual => {
                self.consume();
                Some(BinaryOperator::MulEqual)
            },
            Token::SlashEqual => {
                self.consume();
                Some(BinaryOperator::DivEqual)
            },
            _ => None,
        }
    }

    // Base case for all expressions
    fn expression(&mut self) -> Expression {
        let mut expr = match self.consume() {
            Token::Number(n) => Expression::Literal(Literal::Number(n)),
            Token::StringLiteral(s) => Expression::Literal(Literal::String(s)),
            Token::Identifier(name) => {
                // Function Call
                if matches!(self.peek(), Token::LeftParen) {
                    let args = self.do_args().into_iter().map(Box::new).collect();
                    self.expect(Token::RightParen);
                    Expression::FunctionCall { callee: Expression::Identifier(name).into() , args }
                } else {
                    Expression::Identifier(name)
                }
            }
            Token::True => Expression::Literal(Literal::Boolean(true)),
            Token::False => Expression::Literal(Literal::Boolean(false)),
            Token::Null => Expression::Literal(Literal::Null),
            Token::Undefined => Expression::Literal(Literal::Undefined),
            Token::LeftParen => {
                let expr = self.expression();
                self.expect(Token::RightParen);
                expr
            }
            Token::LeftBracket => {
                let exprs = self.do_array().into_iter().map(Box::new).collect();
                self.expect(Token::RightBracket);
                Expression::Array { elements: exprs }
            }
            Token::LeftBrace => {
                let properties = self.do_object();
                Expression::Object { properties }
            },
            Token::Minus => {
                let expr = self.expression();
                Expression::UnaryOp {
                    op: UnaryOperator::Negate,
                    expr: expr.into(),
                }
            }
            Token::Bang => {
                let expr = self.expression();
                Expression::UnaryOp {
                    op: UnaryOperator::Not,
                    expr: expr.into(),
                }
            }
            tok => panic!("Unexpected token {:?}", tok),
        };

        // Postfix operators
        loop {
            match self.peek() {
                Token::LeftBracket => {
                    self.consume();
                    let index = self.expression();
                    self.expect(Token::RightBracket);
                    expr = Expression::Index {
                        target: expr.into(),
                        index: index.into()
                    }
                }
                Token::Dot => {
                    self.consume();
                    let name = match self.consume() {
                        Token::Identifier(name) => name,
                        tok => panic!("Expected identifier after dot, got {:?}", tok),
                    };

                    expr = Expression::Property {
                        target: expr.into(),
                        name,
                    };
                },
                Token::LeftParen => {
                    let args = self.do_args();
                    self.expect(Token::RightParen);
                    expr = Expression::FunctionCall {
                        callee: expr.into(),
                        args: args.into_iter().map(Box::new).collect(),
                    };
                },
                Token::PlusPlus => {
                    self.consume();
                    expr = Expression::Increment {
                        target: expr.into()
                    };
                },
                Token::MinusMinus => {
                    self.consume();
                    expr = Expression::Decrement {
                        target: expr.into()
                    };
                },
                _ => break,
            }
        }

        // Assignment
        if self.peek() == Token::Equal {
            self.consume();
            let value = self.expression();

            return Expression::Assignment {
                target: expr.into(),
                value: value.into(),
            };
        }

        // Infix operators
        while let Some(op) = self.match_infix_operators() {
            let rhs = self.expression();
            expr = Expression::BinaryOp {
                left: expr.into(),
                op,
                right: rhs.into(),
            };
        }

        expr
    }

    pub fn parse(&mut self) -> AST {
        let mut statements = Vec::new();

        while !self.done() {
            statements.push(self.statement());
        }

        AST { statements }
    }
}
