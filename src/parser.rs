use std::fmt::format;
use crate::ast::{SL, Var, VarTypes};
use crate::lexer::{Lexer, Token, TokenType};

fn str_to_types(from: String) -> Option<VarTypes>{
    match &*from {
        "int" => Some(VarTypes::Int),
        "str" => Some(VarTypes::Str),
        "bool" => Some(VarTypes::Bool),
        "float" => Some(VarTypes::Float),
        _ => {None}
    }
}


pub struct Parser {
    ind: i32,
    run: bool,
    lexer: Lexer,
    current_token: Token,
    to_parse: Vec<Token>,
    defined_names: Vec<String>,
    whole_program: Vec<SL>
}

impl Parser {
    pub fn new() -> Self{
        Self {
            ind: -1,
            run: true,
            lexer: Lexer::new(),
            current_token: Token::new(TokenType::NullForParser, "".to_string()),
            to_parse: vec![],
            defined_names: vec![],
            whole_program: vec![]
        }
    }
    fn next_token(&mut self) -> bool {
        self.ind += 1;
        if self.ind >= self.to_parse.len() as i32{
            false
        } else {
            self.current_token = self.to_parse[self.ind as usize].clone();
            true
        }
    }
    pub fn parse_text(&mut self, text: String) -> Vec<SL>{
        self.to_parse = self.lexer.lex_text(text);
        self.parse()
    }
    fn error(&self, error: String){
        panic!("{}, at line {} char {}", error, self.current_token.y, self.current_token.x)
    }
    fn parse(&mut self) -> Vec<SL>{
        let types = vec!["int".to_string(), "str".to_string(), "bool".to_string(), "float".to_string()];
        while self.run {
            if !self.next_token() {
                self.run = false;
                break
            }
            if self.current_token.token_type == TokenType::Identifier && types.contains(&self.current_token.value) {
                let res_var_type = str_to_types(self.current_token.value.clone());
                if res_var_type.is_none(){
                    self.error(format!("Invalid Type `{}`", self.current_token.value))
                }
                let var_type = res_var_type.unwrap();

                if !self.next_token() || self.current_token.token_type != TokenType::Identifier {
                    self.error(format!("Expected a variable name got '{:?}' instead", &self.current_token.token_type))
                }
                let var_name = self.current_token.value.clone();

                let mut values = vec![];
                let mut expect_op = false;
                loop {
                    self.next_token();
                    if self.current_token.token_type == TokenType::EndLine {
                        break
                    } else if self.current_token.token_type == TokenType::EndOfFile {
                        self.error(format!("Expected end of line got '{:?}' instead", &self.current_token.token_type))
                    } else if self.current_token.token_type == TokenType::Integer {
                        values.push(self.current_token.value.clone())
                    } else if self.current_token.token_type == TokenType::MathOperation {
                        values.push(self.current_token.value.clone())
                    }
                }
                self.whole_program.push(SL::Var(Var::new(var_name, var_type, values)))

            }
        }
        self.whole_program.clone()
    }
}