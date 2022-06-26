use crate::parser::Parsed;

mod lexer;
mod parser;
mod ast;

fn main() {
    let file = "main.sl";
    let content= std::fs::read_to_string(file).expect("couldnt open file");
    let mut the_parser = parser::Parser::new();
    let return_parsed = the_parser.parse_text(content);
    if let Parsed::Program(parsed) = return_parsed {
        println!("{:?}", parsed);
    }
}
