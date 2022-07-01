extern crate core;

use crate::lexer::lexer_test;
use crate::parser::Parsed;

mod util {
    pub mod eval;
    pub mod eval_lexer;
}
mod lexer;
mod parser;
mod ast;
mod check;
mod sl;
mod error;
mod interpreter;

fn main() {
    sl::interpret("main.sl".to_string(), false)
}
