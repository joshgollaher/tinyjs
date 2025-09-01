#![allow(dead_code, unused_imports)]

use std::{env, fs};

mod lexer;
mod token;

use crate::lexer::Lexer;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please specify a file to run.");
    }

    let file = args[1].clone();
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let tokens = Lexer::new(&contents).lex();
    println!("{:?}", tokens);
}
