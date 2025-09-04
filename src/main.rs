#![allow(dead_code, unused_imports)]

use std::{env, fs};
use std::time::Instant;

mod lexer;
mod parser;
mod optim;
mod runtime;


use crate::lexer::Lexer;
use crate::parser::AST;
use crate::runtime::{interpreter, Interpreter};
use crate::optim::Optimizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file to run.");
    }

    let file = args[1].clone();
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let tokens = Lexer::new(&contents).lex();

    let ast = AST::new(tokens);

    let mut optim = Optimizer::new(ast);
    let ast = optim.optimize();

    let mut interpreter = Interpreter::new(ast);
    let start = Instant::now();
    interpreter.run();
    println!("Execution finished in {:.2}ms.", start.elapsed().as_micros() as f64 / 1000.0);
}
