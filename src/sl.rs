use eval::eval;
use crate::{Parsed, parser};
use crate::check::Checker;
use crate::interpreter::Interpreter;
use crate::lexer::{Lexer, Token,TokenType };

fn pre_compile(file_path: String) -> Parsed {
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
    the_checker.check_program(return_parsed.clone());
    return_parsed
}

struct TypeParser {
    to_parse: Vec<Token>,
    ind: i32,
    current_token: Token
}

impl TypeParser {
    fn new() -> Self {
        Self {
            to_parse: vec![],
            ind: -1,
            current_token: Token{
                token_type: TokenType::NullForParser,
                value: "".to_string(),
                x: 0,
                y: 0
            }
        }
    }
    
    fn parse_tokens(&mut self, tokens: Vec<Token>){
        self.to_parse = tokens;
    }
}

fn type_parser(tokens: Vec<Token>){
    let mut open = 0;
    let mut type_parse = vec![];
    for tok in tokens {
        if tok.token_type == TokenType::ParenthesisOpen {
            open += 1;
        } else if tok.token_type == TokenType::ParenthesisClose {
            open -= 1;
            if open == 0 {
                type_parser(type_parse.clone());
                type_parse.clear();
            }
        }
    }
}

pub fn compile(file_path: String) {
    unimplemented!()
}

pub fn interpret(file_path: String) {

    let mut test = Lexer::new();
    let d = test.lex_text(" 1 + 1 + (2 + 2)".to_string());
    type_parser(d);
    return;
    let ret = pre_compile(file_path);

    let mut master = Interpreter::new(ret);
    master.run();
}