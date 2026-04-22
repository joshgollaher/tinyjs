#![allow(dead_code, unused_imports)]

use std::io::{Read, Write};
use std::{env, fs};
use std::rc::Rc;
use std::time::Instant;
use env_logger::Builder;
use log::info;

mod lexer;
mod parser;
mod optim;
mod runtime;


use crate::lexer::Lexer;
use crate::parser::AST;
use crate::runtime::{interpreter, Interpreter};
use crate::optim::Optimizer;

enum Mode {
    File(String),
    Interactive
}

struct Config {
    mode: Mode
}

fn main() {
    Builder::new()
        .filter(None, log::LevelFilter::Trace)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}][{}] {}",
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    let args: Vec<String> = env::args().collect();

    let mode = if args.len() < 2 {
        Mode::Interactive
    } else {
        Mode::File(args[1].clone())
    };

    let config = Rc::new(Config {
        mode
    });

    match &config.mode {
        Mode::File(f) => {
            let contents = fs::read_to_string(f).expect("Something went wrong reading the file");
            let tokens = Lexer::new(&contents).lex();

            let ast = AST::new(tokens);

            println!("{:#?}", ast);

            let mut optim = Optimizer::new(ast);
            let ast = optim.optimize();

            let mut interpreter = Interpreter::new(ast);
            let start = Instant::now();
            interpreter.run();
            info!("Execution finished in {:.2}ms.", start.elapsed().as_micros() as f64 / 1000.0);
        }
        Mode::Interactive => {
            let mut interpreter = Interpreter::new(AST::new(vec![]));
            loop {
                print!(">");
                std::io::stdout().flush().unwrap();
                let mut input_str = "".into();
                let _ = std::io::stdin().lock().read_to_string(&mut input_str).unwrap();

                if input_str.trim() == "exit" {
                    break;
                }

                let tokens = Lexer::new(&input_str).lex();
                let ast = AST::new(tokens);

                interpreter.ast = ast;
                interpreter.run();
            }
        }
    };
}
