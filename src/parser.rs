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
    Var(Token, VarTypes, Vec<Token>),
    Program(Vec<Parsed>),
    If(Vec<Parsed>, Vec<String>, (u32, u32)),
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
        println!("--------------------------------------------------------");
        for tok in &self.to_parse {
            println!("{:?}-", tok)
        }
        println!("--------------------------------------------------------");
        self.parse()
    }
    fn error(&self, error: String){
        panic!("{}, at line {} char {}", error, self.current_token.y, self.current_token.x)
    }
    fn add_var(&mut self, var_name: Token, var_type: VarTypes, values: Vec<Token>){
        let ind = self.scope.len() - 1;
        let d = &mut self.scope[ind];
        match d {
            Parsed::Program(sd) => sd.push(Parsed::Var(var_name, var_type, values)),
            Parsed::If(if_block, ..) => if_block.push(Parsed::Var(var_name, var_type, values)),
            _ => {unimplemented!()}
        }
    }
    fn add_if(&mut self, condition: Vec<String>, loc: (u32, u32)){
        self.scope.push(Parsed::If(vec![], condition, loc))
    }
    fn un_scope(&mut self) {
        let block = self.scope.pop().expect("weird error pop top");
        let ind = self.scope.len() - 1;
        let d = &mut self.scope[ind];
        match d {
            Parsed::Program(sd) => sd.push(block),
            Parsed::If(if_block, ..) => if_block.push(block),
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
                let var_name = self.current_token.clone();

                let mut values = vec![];
                loop {
                    self.next_token();
                    if self.current_token.token_type == TokenType::EndLine {
                        break
                    } else if self.current_token.token_type == TokenType::EndOfFile {
                        self.error(format!("Expected end of line got '{:?}' instead", &self.current_token.token_type))
                    } else {
                        values.push(self.current_token.clone())
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
            else if self.current_token.token_type == TokenType::Else { unimplemented!() }
            else if self.current_token.token_type == TokenType::Fun { unimplemented!() }
            else if self.current_token.token_type == TokenType::Identifier { unimplemented!() }
            else if self.current_token.token_type == TokenType::CurlyBracketClose {
                self.un_scope();
            }
            else if self.current_token.token_type == TokenType::EndOfFile { }
            else {
                self.error(format!("Unknown! {:?}", self.current_token))
            }
        }
        if self.scope.len() > 1 {
            let ind = self.scope.len() - 1;
            let last = &self.scope[ind];
            let mut pos = (0, 0);
            match last {
                Parsed::If(.., loc) => pos = *loc,
                _ => unimplemented!()
            }
            panic!("unclosed Block, at line {} char {}", pos.1, pos.0)
        }
        self.scope[0].clone()
    }
}