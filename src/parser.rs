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
    Conditions(Vec<(Vec<Parsed>, Vec<Token>, (u32, u32))>),
    // ElseIf(),
    // Else(),
}




pub struct Parser {
    index: i32,
    run: bool,
    lexer: Lexer,
    current_token: Token,
    to_parse_tokens: Vec<Token>,
    scope: Vec<Parsed>,
    debug: bool
}

impl Parser {
    pub fn new(debug: bool) -> Self{
        Self {
            index: -1,
            run: true,
            lexer: Lexer::new(),
            current_token: Token::new(TokenType::NullForParser, "".to_string()),
            to_parse_tokens: vec![],
            scope: vec![Parsed::Program(vec![])],
            debug
        }
    }
    fn next_token(&mut self) -> bool {
        self.index += 1;
        if self.index >= self.to_parse_tokens.len() as i32{
            false
        } else {
            self.current_token = self.to_parse_tokens[self.index as usize].clone();
            true
        }
    }
    fn get_next_token(&self) -> Option<Token> {
        let ind = self.index + 1;
        if ind >= self.to_parse_tokens.len() as i32{
            None
        } else {
            Some(self.to_parse_tokens[ind as usize].clone())
        }
    }
    pub fn parse_text(&mut self, text: String) -> Parsed{
        self.to_parse_tokens = self.lexer.lex_text(text);
        if self.debug {
            println!("--------------------------------------------------------");
            for tok in &self.to_parse_tokens {
                println!("{:?}", tok)
            }
            println!("--------------------------------------------------------");
        }
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
    fn add_if(&mut self, condition: Vec<Token>, loc: (u32, u32)){
        self.scope.push(Parsed::Conditions(vec![(vec![], condition, loc)]))
    }
    fn add_else(&mut self, condition: Vec<Token>, loc: (u32, u32)){
        let ind = self.scope.len() - 1;
        let d = &mut self.scope[ind];
        match d {
            Parsed::Program(_whole_program) => self.error("Unexpected Else Block".to_string()),
            Parsed::Conditions(if_block, ..) => {
                // [(stuff, cond, (pos)), (stuff, cond, (pos))]
                if_block.push((vec![], condition, loc))
            },
            _ => {unimplemented!()}
        }
    }
    fn un_scope(&mut self) {
        let block = self.scope.pop().expect("Stack had 1 element which was probably the program");
        self.add_to_top_of_stack(block);
    }
    fn parse(&mut self) -> Parsed {
        let variable_types = vec!["int".to_string(), "str".to_string(), "bool".to_string(), "float".to_string()];
        let allowed_tokens_in_evaluation = vec![
            TokenType::Identifier, TokenType::MathOperation, TokenType::ParenthesisClose,
            TokenType::ComparisonOperation
        ];
        while self.run {
            if !self.next_token() {
                self.run = false;
                break
            }
            if self.current_token.token_type == TokenType::Identifier && variable_types.contains(&self.current_token.value) {
                let resulting_var_type = str_to_types(self.current_token.value.clone());
                if resulting_var_type.is_none(){
                    self.error(format!("Invalid Type `{}`", self.current_token.value))
                }
                let variable_type = resulting_var_type.unwrap();

                if !self.next_token() || self.current_token.token_type != TokenType::Identifier {
                    self.error(format!("Expected a variable name got '{:?}' instead", &self.current_token.token_type))
                }
                let variable_name = self.current_token.clone();

                if !self.next_token() || self.current_token.token_type != TokenType::AssignmentArrow {
                    self.error(format!("Expected a variable assignment operator '<-' got '{:?}' instead", &self.current_token.token_type))
                }

                let mut variable_values_for_evaluation = vec![];
                loop {
                    self.next_token();
                    if self.current_token.token_type == TokenType::EndLine {
                        break
                    } else if self.current_token.token_type == TokenType::EndOfFile {
                        self.error(format!("Expected end of line got '{:?}' instead", &self.current_token.token_type))
                    } else if self.current_token.is_data_type() || allowed_tokens_in_evaluation.contains(&self.current_token.token_type) {
                        variable_values_for_evaluation.push(self.current_token.clone())
                    } else {
                        self.error(format!("Expected Values got {:?}", self.current_token.token_type))
                    }

                }
                // if variable_values_for_evaluation.len() == 0 {
                //     println!("WANING! uninitialized but declared {}", variable_name.value)
                // }
                self.add_var(variable_name, variable_type, variable_values_for_evaluation)
            }
            else if self.current_token.token_type == TokenType::If {
                let if_position = (self.current_token.x, self.current_token.y);

                let mut if_condition = vec![];
                loop {
                    self.next_token();
                    if self.current_token.token_type == TokenType::EndOfFile {
                        self.error("Expected Arguments".to_string());
                    } else if self.current_token.token_type == TokenType::CurlyBracketOpen {
                        break
                    } else {
                        if_condition.push(self.current_token.clone())
                    }
                }
                self.add_if(if_condition, if_position);
            }
            else if self.current_token.token_type == TokenType::Else {
                self.next_token();
                if self.current_token.token_type == TokenType::If {
                    let else_if_pos = (self.current_token.x, self.current_token.y);

                    let mut else_if_condition = vec![];
                    loop {
                        self.next_token();
                        if self.current_token.token_type == TokenType::EndOfFile {
                            self.error("Expected Arguments".to_string());
                        } else if self.current_token.token_type == TokenType::CurlyBracketOpen {
                            break
                        } else {
                            else_if_condition.push(self.current_token.clone())
                        }
                    }
                    self.add_else(else_if_condition, else_if_pos);
                } else {
                    let else_pos = (self.current_token.x, self.current_token.y);
                    if self.current_token.token_type != TokenType::CurlyBracketOpen {
                        self.error(format!("Expected start of scope got '{:?}' instead", &self.current_token.token_type))
                    }
                    self.add_else(vec![Token::new(TokenType::Boolean, "true".to_string())], else_pos);
                }

            }
            else if self.current_token.token_type == TokenType::Fun { unimplemented!() }
            else if self.current_token.token_type == TokenType::Identifier {
                let identifier_name = self.current_token.clone();
                if self.next_token() && self.current_token.token_type == TokenType::ParenthesisOpen {
                    let mut function_args = vec![];
                    let mut function_calls = 0;
                    loop {
                        self.next_token();
                        if self.current_token.token_type == TokenType::ParenthesisClose && function_calls == 0 {
                            break
                        } else if self.current_token.token_type == TokenType::ParenthesisClose {
                            function_args.push(self.current_token.clone());
                            function_calls -= 1
                        } else if self.current_token.token_type == TokenType::ParenthesisOpen {
                            function_calls += 1;
                            function_args.push(self.current_token.clone());
                            unimplemented!()
                        } else if self.current_token.is_data_type() || allowed_tokens_in_evaluation.contains(&self.current_token.token_type) ||
                            self.current_token.token_type == TokenType::SeperatorComma {
                            function_args.push(self.current_token.clone());
                        } else {
                            self.error(format!("Expected arguments got {:?}", self.current_token.token_type))
                        }
                    }
                    if !self.next_token() || self.current_token.token_type != TokenType::EndLine {
                        self.error(format!("Expected {:?} got {:?}", TokenType::EndLine,  self.current_token.token_type))
                    }
                    self.add_func_call(identifier_name, function_args)
                }
                else if self.current_token.token_type == TokenType::AssignmentArrow {
                     let mut reassign_values = vec![];
                    loop {
                        self.next_token();
                        if self.current_token.token_type == TokenType::EndLine {
                            break
                        } else if self.current_token.token_type == TokenType::EndOfFile {
                            self.error(format!("Expected end of line got '{:?}' instead", &self.current_token.token_type))
                        } else if self.current_token.is_data_type() || allowed_tokens_in_evaluation.contains(&self.current_token.token_type) {
                            reassign_values.push(self.current_token.clone())
                        } else {
                            self.error(format!("Expected Values got {:?}", self.current_token.token_type))
                        }

                    }
                    if reassign_values.len() == 0 {
                        self.error("Expected values".to_string());
                    }
                    self.reassign_var(identifier_name, reassign_values)
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
                self.error(format!("Unknown! {:?}", self.current_token))
            }
        }
        if self.scope.len() > 1 {
            let index = self.scope.len() - 1;
            let last_compound_statements_container = &self.scope[index];
            let mut position = (0, 0);
            match last_compound_statements_container {
                Parsed::Conditions(condition) => {
                    let index2 = condition.len();
                    position = condition[index2 - 1].2.clone();

                }
                _ => unimplemented!()
            }
            panic!("unclosed Block, at line {} char {}", position.1, position.0)
        }
        self.scope[0].clone()
    }
}