#![allow(dead_code, unused_imports)]

use std::{env, fs};

mod lexer;
mod parser;
mod optim;
mod runtime;


use crate::lexer::Lexer;
use crate::parser::AST;
use crate::runtime::{interpreter, Interpreter};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file to run.");
    }

    let file = args[1].clone();
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let tokens = Lexer::new(&contents).lex();
    // println!("{:?}", tokens);

    let ast = AST::from_tokens(tokens);
    println!("{:#?}", ast);

    let mut interpreter = Interpreter::new(ast);
    interpreter.run();
}
