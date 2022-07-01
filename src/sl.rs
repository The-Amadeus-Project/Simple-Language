use std::collections::HashMap;
use crate::{Parsed, parser};
use crate::check::{Checker, ArgTypes};
use crate::interpreter::{Interpreter, Value};
use crate::lexer::{Lexer, Token,TokenType };
use crate::parser::VarTypes;



fn pre_compile(file_path: String) -> Parsed {
    let file_content = std::fs::read_to_string(file_path).expect("couldnt open file");
    let mut the_parser = parser::Parser::new();
    let return_parsed = the_parser.parse_text(file_content);
    if let Parsed::Program(parsed) = &return_parsed {
        for part in parsed {
            println!("{:?}", part)
        }
    }
    println!("--------------------------------------------------------");
    let mut type_checker = Checker::new();
    type_checker.check_program(return_parsed.clone());
    return_parsed
}


pub fn compile(file_path: String) {
    unimplemented!()
}

pub fn interpret(file_path: String) {
    let ret = pre_compile(file_path);

    let mut master = Interpreter::new(ret);
    master.run();
}