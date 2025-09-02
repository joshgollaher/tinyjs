use std::cmp::PartialEq;
use crate::parser::{Expression, Statement, AST};
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

    fn statement(&mut self) -> Statement {

        todo!();
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