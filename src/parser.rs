use std::fmt::format;
use crate::ast::{Expr, SL, Var, VarTypes};
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

fn data_token_type_to_types(from: TokenType) -> Option<VarTypes>{
    match from {
        TokenType::Integer => Some(VarTypes::Int),
        TokenType::String => Some(VarTypes::Str),
        TokenType::Boolean => Some(VarTypes::Bool),
       TokenType::FloatingPoint => Some(VarTypes::Float),
        _ => {None}
    }
}


pub struct Math {
    given: Vec<String>,
    ind: i32,
    current: String,
    terms: Vec<String>,
    expression: Vec<String>
    // todo: make math with string and float work
}

impl Math {
    pub fn new() -> Self {
        Self {
            given: vec![],
            ind: -1,
            current: "".to_string(),
            terms: vec!["*".to_string(), "/".to_string()],
            expression: vec!["+".to_string(), "-".to_string()]
        }
    }
    fn next(&mut self) -> bool {
        self.ind += 1;
        if self.ind >= self.given.len() as i32 {
            false
        } else {
            self.current = self.given[self.ind as usize].clone();
            true
        }
    }
    fn factor(&mut self) -> Expr {
        Expr::Number(self.current.parse::<i128>().expect("invalid?"))
    }
    fn term(&mut self) -> Expr {
        let mut left = self.factor();
        self.next();

        while self.terms.contains(&self.current) {
            let operation = self.current.clone();
            self.next();
            let right = self.factor();
            let result = self.next();
            if operation == "*" {
                left = Expr::Multiply(Box::new(right), Box::new(left))
            } else {
                left = Expr::Division(Box::new(right), Box::new(left))
            }
            if !result {
                break
            }

        }
        left
    }
    fn expression(&mut self) -> Expr {
        let mut left = self.term();

        while self.expression.contains(&self.current) {
            let operation = self.current.clone();
            self.next();
            let right = self.term();
            let result = self.next();
            if operation == "+" {
                left = Expr::Addition(Box::new(right), Box::new(left))
            } else {
                left = Expr::Subtraction(Box::new(right), Box::new(left))
            }
            if !result {
                break
            }

        }
        left
    }
    pub fn parse(&mut self, given: Vec<String>) -> Expr {
        self.given = given;
        self.next();
        self.expression()

    }
}


pub struct Parser {
    ind: i32,
    run: bool,
    lexer: Lexer,
    current_token: Token,
    to_parse: Vec<Token>,
    defined_names: Vec<String>,
    whole_program: Sl
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
                    } else if self.current_token.is_data_type() {
                        let converted = data_token_type_to_types(self.current_token.token_type.clone()).unwrap();
                        if converted != var_type {
                            self.error(format!("Expected Data type of '{:?}' got '{:?}' instead", var_type, converted))
                        }

                        if expect_op {
                            self.error(format!("Expected Operation got '{:?}'", self.current_token.token_type))
                        }
                        expect_op = true;
                        values.push(self.current_token.value.clone())
                    } else if self.current_token.token_type == TokenType::MathOperation {
                        if !expect_op {
                            self.error(format!("Expected Data Type got '{:?}'", self.current_token.token_type))
                        }
                        expect_op = false;
                        values.push(self.current_token.value.clone())
                    }
                }
                let mut math = Math::new();
                let expr_parsed =  math.parse(values);
                self.whole_program.push(SL::VariableAssignment(var_name, var_type, expr_parsed))

            }
        }
        self.whole_program.clone()
    }
}