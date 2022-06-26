use crate::lexer::{Lexer, Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum VarTypes {
    Int,
    Str,
    Bool,
    Float,
}


fn str_to_types(from: String) -> Option<VarTypes>{
    match &*from {
        "int" => Some(VarTypes::Int),
        "str" => Some(VarTypes::Str),
        "bool" => Some(VarTypes::Bool),
        "float" => Some(VarTypes::Float),
        _ => {None}
    }
}

fn data_token_type_to_types(from: TokenType) -> Option<VarTypes>{
    match from {
        TokenType::Integer => Some(VarTypes::Int),
        TokenType::String => Some(VarTypes::Str),
        TokenType::Boolean => Some(VarTypes::Bool),
       TokenType::FloatingPoint => Some(VarTypes::Float),
        _ => {None}
    }
}

#[derive(Debug, Clone)]
pub struct ParsedVar {
    value: Vec<Token>,
    name: String,
    var_type: VarTypes
}

impl ParsedVar {
    pub fn new(name: String, var_type: VarTypes, value: Vec<Token>) -> Self {
        Self {
            value,
            name,
            var_type
        }
    }
}

#[derive(Debug, Clone)]
pub enum Parsed {
    Var(ParsedVar),
    Program(Vec<Parsed>),
    // If(),
    // ElseIf(),
    // Else(),
}




pub struct Parser {
    ind: i32,
    run: bool,
    lexer: Lexer,
    current_token: Token,
    to_parse: Vec<Token>,
    defined_names: Vec<String>,
    scope: Vec<Parsed>
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
            scope: vec![Parsed::Program(vec![])]
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
    pub fn parse_text(&mut self, text: String) -> Parsed{
        self.to_parse = self.lexer.lex_text(text);
        self.parse()
    }
    fn error(&self, error: String){
        panic!("{}, at line {} char {}", error, self.current_token.y, self.current_token.x)
    }
    fn add_var(&mut self, var_name: String, var_type: VarTypes, values: Vec<Token>){
        let ind = self.scope.len() - 1;
        let d = &mut self.scope[ind];
        match d {
            Parsed::Program(sd) => sd.push(Parsed::Var(ParsedVar::new(var_name, var_type, values))),
            _ => {unimplemented!()}
        }
    }
    fn parse(&mut self) -> Parsed {
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
                    } else if self.current_token.is_data_type() {
                        // let converted = data_token_type_to_types(self.current_token.token_type.clone()).unwrap();
                        // if converted != var_type {
                        //     self.error(format!("Expected Data type of '{:?}' got '{:?}' instead", var_type, converted))
                        // }

                        if expect_op {
                            self.error(format!("Expected Operation got '{:?}'", self.current_token.token_type))
                        }
                        expect_op = true;
                        values.push(self.current_token.clone())
                    } else if self.current_token.token_type == TokenType::MathOperation {
                        if !expect_op {
                            self.error(format!("Expected Data Type got '{:?}'", self.current_token.token_type))
                        }
                        expect_op = false;
                        values.push(self.current_token.clone())
                    }
                }
                self.add_var(var_name, var_type, values)

            }
        }
        let last_ind = self.scope.len() - 1;
        self.scope[last_ind].clone()
    }
}