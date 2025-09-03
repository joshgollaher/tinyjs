use std::cmp::PartialEq;
use crate::parser::{Expression, Literal, Statement, AST};
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}



impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
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
        todo!()
    }

    fn do_let(&mut self) -> Statement {
        todo!()
    }

    fn do_while(&mut self) -> Statement {
        todo!()
    }

    fn do_for(&mut self) -> Statement {
        todo!()
    }

    fn do_function(&mut self) -> Statement {
        todo!()
    }

    fn do_block(&mut self) -> Vec<Statement> {
        todo!()
    }

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
            Token::LeftBrace => {
                let statements = self.do_block();
                Statement::Scope { statements }
            }
            _ => {
                let expr = self.expression();
                self.expect(Token::Semicolon);
                Statement::Expression(Box::new(expr))
            }
        }
    }

    fn expression(&mut self) -> Expression {

        todo!();
    }

    pub fn parse(&mut self) -> AST {
        let mut statements = Vec::new();

        while !self.done() {
            statements.push(self.statement());
        }

        AST { statements }
    }
}