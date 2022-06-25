extern crate core;

use crate::ast::SL;

mod lexer;
mod parser;
mod ast;

fn main() {
    let file = "main.sl";
    let content= std::fs::read_to_string(file).expect("couldnt open file");
    let mut the_parser = parser::Parser::new();
    let parsed = the_parser.parse_text(content);
    for part in parsed {
        println!("{:?}", part)
    }
}
