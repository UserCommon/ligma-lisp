mod errors;
mod token;
mod parser;
mod evaluator;
mod repl;


fn main() {
    use std::io::{stdin, BufReader};
    repl::repl(BufReader::new(stdin()));
}
