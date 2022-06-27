use crate::{Parsed, parser};
use crate::check::Checker;

pub fn compile(file_path: String) {
    unimplemented!()
}

pub fn interpret(file_path: String) {
    let content= std::fs::read_to_string(file_path).expect("couldnt open file");
    let mut the_parser = parser::Parser::new();
    let return_parsed = the_parser.parse_text(content);
    if let Parsed::Program(parsed) = &return_parsed {
        for part in parsed {
            println!("{:?}", part)
        }
    }
    println!("--------------------------------------------------------");
    let mut the_checker = Checker::new();
    the_checker.check_program(return_parsed.clone())
}