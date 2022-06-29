use crate::lexer::{Lexer, Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum VarTypes {
    Int,
    Str,
    Bool,
    Float,
    Struct,
}


pub fn str_to_types(from: String) -> Option<VarTypes>{
    match &*from {
        "int" => Some(VarTypes::Int),
        "str" => Some(VarTypes::Str),
        "bool" => Some(VarTypes::Bool),
        "float" => Some(VarTypes::Float),
        _ => {None}
    }
}

pub fn data_token_type_to_types(from: &TokenType) -> Option<VarTypes> {
    match *from {
        TokenType::Integer => Some(VarTypes::Int),
        TokenType::String => Some(VarTypes::Str),
        TokenType::Boolean => Some(VarTypes::Bool),
        TokenType::FloatingPoint => Some(VarTypes::Float),
        _ => { None }
    }
}

#[derive(Debug, Clone)]
pub enum Parsed {
    VariableAssignment(Token, VarTypes, Vec<Token>),
    VariableReassignment(Token, Vec<Token>),
    Program(Vec<Parsed>),
    FuncCall(Token, Vec<Token>),
    Conditions(Vec<(Vec<Parsed>, Vec<String>, (u32, u32))>),
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
    fn get_next_token(&self) -> Option<Token> {
        let ind = self.ind + 1;
        if ind >= self.to_parse.len() as i32{
            None
        } else {
            Some(self.to_parse[ind as usize].clone())
        }
    }
    pub fn parse_text(&mut self, text: String) -> Parsed{
        self.to_parse = self.lexer.lex_text(text);
        println!("--------------------------------------------------------");
        for tok in &self.to_parse {
            println!("{:?}", tok)
        }
        println!("--------------------------------------------------------");
        self.parse()
    }
    fn error(&self, error: String){
        panic!("{}, at line {} char {}", error, self.current_token.y, self.current_token.x)
    }
    fn add_to_top_of_stack(&mut self, to_push: Parsed){
        let ind = self.scope.len() - 1;
        let d = &mut self.scope[ind];
        match d {
            Parsed::Program(sd) => sd.push(to_push),
            Parsed::Conditions(if_block, ..) => {
                // [(stuff, cond, (pos)), (stuff, cond, (pos))]
                let ind = if_block.len();
                if_block[ind - 1].0.push(to_push)
            },
            _ => {unimplemented!()}
        }
    }
    fn add_var(&mut self, var_name: Token, var_type: VarTypes, values: Vec<Token>){
        self.add_to_top_of_stack(Parsed::VariableAssignment(var_name, var_type, values))
    }
    fn reassign_var(&mut self, var_name: Token, values: Vec<Token>){
        self.add_to_top_of_stack(Parsed::VariableReassignment(var_name, values))
    }
    fn add_func_call(&mut self, func_name: Token, args: Vec<Token>){
        self.add_to_top_of_stack(Parsed::FuncCall(func_name, args))
    }
    fn add_if(&mut self, condition: Vec<String>, loc: (u32, u32)){
        self.scope.push(Parsed::Conditions(vec![(vec![], condition, loc)]))
    }
    fn add_else(&mut self, condition: Vec<String>, loc: (u32, u32)){
        let ind = self.scope.len() - 1;
        let d = &mut self.scope[ind];
        match d {
            Parsed::Program(sd) => self.error("Unexpected Else Block".to_string()),
            Parsed::Conditions(if_block, ..) => {
                // [(stuff, cond, (pos)), (stuff, cond, (pos))]
                if_block.push((vec![], condition, loc))
            },
            _ => {unimplemented!()}
        }
    }
    fn un_scope(&mut self) {
        let block = self.scope.pop().expect("weird error pop top");
        self.add_to_top_of_stack(block);
    }
    fn parse(&mut self) -> Parsed {
        let types = vec!["int".to_string(), "str".to_string(), "bool".to_string(), "float".to_string()];
        let allowed_in_var_func_call = vec![
            TokenType::Identifier, TokenType::MathOperation, TokenType::ParenthesisClose,
            TokenType::ComparisonOperation
        ];
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
                let var_name = self.current_token.clone();

                if !self.next_token() || self.current_token.token_type != TokenType::AssignmentArrow {
                    self.error(format!("Expected a variable assignment operator '<-' got '{:?}' instead", &self.current_token.token_type))
                }

                let mut values = vec![];
                loop {
                    self.next_token();
                    if self.current_token.token_type == TokenType::EndLine {
                        break
                    } else if self.current_token.token_type == TokenType::EndOfFile {
                        self.error(format!("Expected end of line got '{:?}' instead", &self.current_token.token_type))
                    } else if self.current_token.is_data_type() || allowed_in_var_func_call.contains(&self.current_token.token_type) {
                        values.push(self.current_token.clone())
                    } else {
                        self.error(format!("Expected Values got {:?}", self.current_token.token_type))
                    }

                }
                if values.len() == 0 {
                    println!("WANING! uninitialized but declared {}", var_name.value)
                }
                self.add_var(var_name, var_type, values)
            }
            else if self.current_token.token_type == TokenType::If {
                let if_pos = (self.current_token.x, self.current_token.y);

                let mut condition = vec![];
                loop {
                    self.next_token();
                    if self.current_token.token_type == TokenType::EndOfFile {
                        self.error("Expected Arguments".to_string());
                    } else if self.current_token.token_type == TokenType::CurlyBracketOpen {
                        break
                    } else {
                        condition.push(self.current_token.true_value())
                    }
                }
                self.add_if(condition, if_pos);
            }
            else if self.current_token.token_type == TokenType::Else {
                self.next_token();
                if self.current_token.token_type == TokenType::If {
                    let if_pos = (self.current_token.x, self.current_token.y);

                    let mut condition = vec![];
                    loop {
                        self.next_token();
                        if self.current_token.token_type == TokenType::EndOfFile {
                            self.error("Expected Arguments".to_string());
                        } else if self.current_token.token_type == TokenType::CurlyBracketOpen {
                            break
                        } else {
                            condition.push(self.current_token.true_value())
                        }
                    }
                    self.add_else(condition, if_pos);
                } else {
                    unimplemented!()
                }

            }
            else if self.current_token.token_type == TokenType::Fun { unimplemented!() }
            else if self.current_token.token_type == TokenType::Identifier {
                let name = self.current_token.clone();
                if self.next_token() && self.current_token.token_type == TokenType::ParenthesisOpen {
                    let mut func_args = vec![];
                    let mut calls = 0;
                    loop {
                        self.next_token();
                        if self.current_token.token_type == TokenType::ParenthesisClose && calls == 0 {
                            break
                        } else if self.current_token.token_type == TokenType::ParenthesisClose {
                            func_args.push(self.current_token.clone());
                            calls -= 1
                        } else if self.current_token.token_type == TokenType::ParenthesisOpen {
                            calls += 1;
                            func_args.push(self.current_token.clone());
                            unimplemented!()
                        } else if self.current_token.is_data_type() || allowed_in_var_func_call.contains(&self.current_token.token_type) ||
                            self.current_token.token_type == TokenType::SeperatorComma {
                            func_args.push(self.current_token.clone());
                        } else {
                            self.error(format!("Expected arguments got {:?}", self.current_token.token_type))
                        }
                    }
                    if !self.next_token() || self.current_token.token_type != TokenType::EndLine {
                        self.error(format!("Expected {:?} got {:?}", TokenType::EndLine,  self.current_token.token_type))
                    }
                    self.add_func_call(name, func_args)
                }
                else if self.current_token.token_type == TokenType::AssignmentArrow {
                     let mut values = vec![];
                    loop {
                        self.next_token();
                        if self.current_token.token_type == TokenType::EndLine {
                            break
                        } else if self.current_token.token_type == TokenType::EndOfFile {
                            self.error(format!("Expected end of line got '{:?}' instead", &self.current_token.token_type))
                        } else if self.current_token.is_data_type() || allowed_in_var_func_call.contains(&self.current_token.token_type) {
                            values.push(self.current_token.clone())
                        } else {
                            self.error(format!("Expected Values got {:?}", self.current_token.token_type))
                        }

                    }
                    if values.len() == 0 {
                        self.error("Expected values".to_string());
                    }
                    self.reassign_var(name, values)
                }
            }
            else if self.current_token.token_type == TokenType::CurlyBracketClose &&
                self.get_next_token().unwrap().token_type != TokenType::Else {

                if self.scope.len() == 1 {
                    self.error(format!("Unexpected `{}`", self.current_token.value))
                }
                self.un_scope();
            }
            else if self.current_token.token_type == TokenType::EndOfFile { }
            else if self.current_token.token_type == TokenType::CurlyBracketClose {}
            else {
                println!("{:?}",self.get_next_token().unwrap().token_type);
                self.error(format!("Unknown! {:?}", self.current_token))
            }
        }
        if self.scope.len() > 1 {
            let ind = self.scope.len() - 1;
            let last = &self.scope[ind];
            let mut pos = (0, 0);
            match last {
                Parsed::Conditions(cond) => {
                    let ind = cond.len();
                    pos = cond[ind - 1].2.clone();

                },
                _ => unimplemented!()
            }
            panic!("unclosed Block, at line {} char {}", pos.1, pos.0)
        }
        self.scope[0].clone()
    }
}