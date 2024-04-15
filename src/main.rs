mod errors;
mod token;
mod parser;
mod evaluator;
mod repl;

use std::collections::VecDeque;
use std::io::{stdin, BufRead, BufReader};
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::env;



fn main() {
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();

    match args.len() {
        0 => {
            repl::repl(BufReader::new(stdin()));
        },
        1 => {
            match args[0].as_str() {
                "--help" => { println!("Help"); },
                path => interpret_file(
                    BufReader::new(
                        fs::File::open(path).expect("Path not found!")
                    )
                )
            }
        },
        _ => {println!("write --help, to get available flags");}
    }

}


fn interpret_file(mut buf: impl BufRead) {
    let mut env = Rc::new(RefCell::new(evaluator::Env::new()));
    let mut line = String::new();

    buf.read_to_string(&mut line).expect("Failed to read!");

    match evaluator::eval(&line, &mut env) {
        Ok(val) => {
            use parser::Object::*;
            match val {
                Void => {},
                Number(n) => {println!("{}", n)},
                Bool(f) => {println!("{}", f)},
                Symbol(s) => {println!("{}", s)},
                Function(params, body) => {
                    print!("Function( ");
                    for param in params {
                        print!("{} ", param);
                    }
                    println!(")");

                    for expr in body {
                        println!(" {:?}", expr);
                    }
                },
                _ => println!("{:?}", val)
            };
        },
        Err(e) => println!("Error: {}", e)
    };


}
