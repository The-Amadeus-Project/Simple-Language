use crate::lexer::lexer_test;
use crate::parser::Parsed;

mod lexer;
mod parser;
mod ast;
mod check;
mod sl;
mod error;

fn main() {
    sl::interpret("main.sl".to_string())
}
