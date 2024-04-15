use crate::*;
use std::io::{self, BufRead};
use std::rc::Rc;
use std::cell::RefCell;

// TODO Make it to accept Bufreader to make it compatabale with
// files
// FIXME supports only line-by-line input?
pub fn repl(mut reader: impl BufRead) {
    let mut env = Rc::new(RefCell::new(evaluator::Env::new()));

    let mut line = String::new();
    while line != "(exit)" {
        line.clear();
        reader.read_line(&mut line).expect("An error while reading input occured!");
        println!("{}", line);
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
        }
    }
}
