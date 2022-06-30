
use std::collections::HashMap;
use crate::lexer::{Token, TokenType};
use crate::Parsed;
use crate::parser::{data_token_type_to_types, str_to_types, VarTypes};

#[derive(Debug, PartialEq, Clone, Copy)]
enum ParseTypes {
    Int,
    Float,
    Bool,
    String,
    Math,
    Comparison,
    ParenthesisOpen,
    ParenthesisClose
}

fn token_type_to_parse_type(token: & Token) -> Option<ParseTypes> {
    match token.token_type {
        TokenType::Integer => Some(ParseTypes::Int),
        TokenType::String => Some(ParseTypes::String),
        TokenType::FloatingPoint => Some(ParseTypes::Float),
        TokenType::Boolean => Some(ParseTypes::Bool),
        TokenType::ParenthesisOpen => Some(ParseTypes::ParenthesisOpen),
        TokenType::ParenthesisClose => Some(ParseTypes::ParenthesisClose),
        TokenType::MathOperation => Some(ParseTypes::Math),
        TokenType::ComparisonOperation => Some(ParseTypes::Comparison),
        _ => None
    }
}

fn var_types_to_parse_type(token: &VarTypes) -> Option<ParseTypes> {
    match token {
        VarTypes::Bool => Some(ParseTypes::Bool),
        VarTypes::Bool => Some(ParseTypes::Int),
        VarTypes::Bool => Some(ParseTypes::Float),
        VarTypes::Bool => Some(ParseTypes::String),
        _ => None
    }
}

fn parse_type_to_var_types(token: &ParseTypes) -> Option<VarTypes> {
    match token {
       ParseTypes::String => Some(VarTypes::Str),
       ParseTypes::Bool => Some(VarTypes::Bool),
       ParseTypes::Int => Some(VarTypes::Int),
       ParseTypes::Float => Some(VarTypes::Float),
        _ => None
    }
}


struct TypeParser {
    to_parse: Vec<Token>,
    defined_var: HashMap<String, VarTypes>,
    defined_struct: HashMap<String, VarTypes>,
    //                                 args type      return type
    defined_function: HashMap<String, (Vec<ArgTypes>, Vec<VarTypes>)>,
    removed: Vec<String>
}

impl TypeParser {
    fn new(tokens: Vec<Token>,
           defined_var: HashMap<String, VarTypes>,
           defined_struct: HashMap<String, VarTypes>,
           defined_function: HashMap<String, (Vec<ArgTypes>, Vec<VarTypes>)>,
           removed: Vec<String>) -> Self
    {

        Self {
            to_parse: tokens,
            defined_var,
            defined_struct,
            defined_function,
            removed
        }
    }

    fn eval(&mut self, evaluate: Vec<(ParseTypes, u32, u32)>) -> (ParseTypes, u32, u32) {
        let mut open = 0;
        let mut eval_now = vec![];
        let mut to_eval = vec![];
        for part in evaluate {
            if part.0 == ParseTypes::ParenthesisClose {
                open -= 1;
                if open != 0 {
                    to_eval.push(part);
                }
                if open == 0 {
                    eval_now.push(self.eval(to_eval.clone()));
                    to_eval.clear();
                }
            } else if part.0 == ParseTypes::ParenthesisOpen {
                if open != 0 {
                    to_eval.push(part)
                }
                open += 1;
            } else if open != 0 {
                println!("{:?}", part);
                to_eval.push(part);
            } else {
                eval_now.push(part);
            }
        }
        let mut ind = -1;
        let mut rec = 0;
        while eval_now.len() != 1 {
            ind += 1;
            if rec == 1 {
                break
            }
            if ind >= eval_now.len() as i32{
                ind = 0;
                rec += 1;
            }
            let current = eval_now.get(ind as usize).unwrap();
            if current.0 == ParseTypes::Math {
                rec = 0;
                if eval_now.get((ind - 1) as usize).unwrap().0 == eval_now.get((ind + 1) as usize).unwrap().0{
                    eval_now.remove((ind - 1) as usize);
                    eval_now.remove((ind - 1) as usize);
                } else {
                    let first = eval_now.get((ind - 1) as usize).unwrap();
                    let other = eval_now.get((ind + 1) as usize).unwrap();
                    panic!("Expected {:?} got '{:?}' instead, at line {} char {}", first.0, other.0, other.2, other.1)
                }
            }
        }

        let mut ind = -1;
        let mut rec = 0;
        while eval_now.len() != 1 {
            ind += 1;
            if rec == 1 {
                break
            }
            if ind >= eval_now.len() as i32{
                ind = 0;
                rec += 1;
            }
            let current = eval_now.get(ind as usize).unwrap();
            if current.0 == ParseTypes::Comparison {
                if eval_now.get((ind - 1) as usize).unwrap().0 == eval_now.get((ind + 1) as usize).unwrap().0{
                    eval_now.remove((ind - 1) as usize);
                    eval_now.remove((ind - 1) as usize);
                    let d = eval_now.remove((ind - 1) as usize);
                    eval_now.insert((ind - 1) as usize, (ParseTypes::Bool, d.1, d.2));
                } else {
                    let first = eval_now.get((ind - 1) as usize).unwrap();
                    let other = eval_now.get((ind + 1) as usize).unwrap();
                    panic!("Expected {:?} got '{:?}' instead, at line {} char {}", first.0, other.0, other.2, other.1)
                }
            }
        }

        if eval_now.len() > 1 {
            panic!("AasD?")
        }
        eval_now[0 as usize]
    }

    fn parse(&mut self) -> ParseTypes {
        let mut to_eval = vec![];
        for token in &self.to_parse {
            if token.token_type != TokenType::Identifier {
                to_eval.push((token_type_to_parse_type(token).expect(&*format!("{:?}", token)), token.x, token.y))
            } else {
                if self.defined_var.contains_key(&token.value){
                    let var = var_types_to_parse_type(self.defined_var.get(&token.value).unwrap()).unwrap();
                    to_eval.push((var, token.x, token.y));
                } else {
                    unimplemented!()
                }
            }
        }
        self.eval(to_eval).0
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ArgTypes {
    Int,
    Str,
    Bool,
    Float,
    Any,
    Struct,
    Variadic(Box<ArgTypes>)
}

pub struct Checker
{
    defined_var: HashMap<String, VarTypes>,
    defined_struct: HashMap<String, VarTypes>,
    //                                 args type      return type
    defined_function: HashMap<String, (Vec<ArgTypes>, Vec<VarTypes>)>,
    removed: Vec<String>
}

fn var_types_to_arg_type(var: &VarTypes) -> Option<ArgTypes>{
    match var {
        VarTypes::Bool => Some(ArgTypes::Bool),
        VarTypes::Int =>   Some(ArgTypes::Int),
        VarTypes::Float => Some(ArgTypes::Float),
        VarTypes::Str => Some(ArgTypes::Str),
        _ => None
    }
}

impl Checker {
    pub fn new() -> Self {
        let mut new = Self {
            defined_var: HashMap::new(),
            defined_struct: HashMap::new(),
            defined_function: HashMap::new(),
            removed: vec![]
        };
        new.defined_function.insert("out".to_string(), (vec![ArgTypes::Any], vec![]));
        new
    }
    fn var_check(&mut self, name: Token, var_type: VarTypes, values: Vec<Token>) -> (String, VarTypes){
        if self.defined_var.contains_key(&name.value){
            panic!("variable '{}' already exists, error at line {} char {}", name.value, name.y, name.x);
        }
        if values.len() % 2 == 0  {
            let last_ind = values.len() - 1;
            let last = values.get(last_ind).unwrap();
            panic!("something wrong with expression just doesnt know, at line {} char {}", last.y, last.x)
        }
        let mut d = TypeParser::new(
            values,
            self.defined_var.clone(),
            self.defined_struct.clone(),
            self.defined_function.clone(),
            self.removed.clone());
        let d = parse_type_to_var_types(&d.parse()).expect("please help me");
        if d != var_type {
            panic!("expression result in '{:?}' and not '{:?}', at line {} char {}", d, var_type, name.y, name.x)
        }
        (name.value.clone(), var_type)
    }

    fn var_reassign_check(&mut self, name: Token, values: Vec<Token>){
        if !self.defined_var.contains_key(&name.value){
            panic!("variable '{}' does not exists, error at line {} char {}", name.value, name.y, name.x);
        }
        let var_type = self.defined_var.get(&name.value).unwrap().clone();
        let mut exp_op = false;
        let mut ind = -1;
        loop {
            ind += 1;
            let initial =  values.get(ind as usize);
            if initial.is_none(){
                break
            }
            let mut val = initial.unwrap();

            if val.is_data_type() {
                if exp_op {
                    panic!("Expected Math Operation got '{}' instead, at line {} char {}", val.true_value(), val.y, val.x);
                }
                exp_op = true;
                let val_type = data_token_type_to_types(&val.token_type).unwrap();
                if var_type != val_type {
                    panic!("Expected a '{:?}' got '{:?}', at line {}, char {}", var_type, val.token_type, val.y, val.x)
                }
            } else if val.token_type == TokenType::MathOperation {
                if val.value != "+" && var_type == VarTypes::Str {
                    panic!("addition is only allowed for strings types, at line {} char {}", val.y, val.x)
                } else if var_type == VarTypes::Bool {
                    panic!("Math operation not allowed for boolean types at line {} char {}", val.y, val.x)
                }
                if !exp_op {
                    panic!("Expected DataType got '{}' instead, at line {} char {}", val.true_value(), val.y, val.x);
                }
                exp_op = false;
            } else {
                if self.defined_var.contains_key(&val.value){
                    if exp_op {
                        panic!("Expected Math Operation got '{}' instead, at line {} char {}", val.true_value(), val.y, val.x);
                    }
                    exp_op = true;
                    let reference = self.defined_var.get(&val.value).unwrap();
                    if var_type != reference.clone() {
                        panic!("Expected a '{:?}' got '{:?}', at line {}, char {}", var_type, val.token_type, val.y, val.x)
                    }
                } else if self.removed.contains(&val.value){
                    panic!("Out of scope '{}', at line {} char {}", val.true_value(), val.y, val.x);
                }else {
                    unimplemented!("{:?}", val)
                }
            }
        }
        if !exp_op  {
            let last_ind = values.len() - 1;
            let last = values.get(last_ind).unwrap();
            panic!("Didn't Close, at line {} char {}", last.y, last.x)
        }
    }

    fn if_check(&mut self, compound: Vec<Parsed>, condition: Vec<Token>, position: (u32, u32)){
        let mut type_parser = TypeParser::new(
            condition,
            self.defined_var.clone(),
            self.defined_struct.clone(),
            self.defined_function.clone(),
            self.removed.clone());
        let d = parse_type_to_var_types(&type_parser.parse()).expect("please help me");
        if d != VarTypes::Bool {
            panic!("expression result in '{:?}' and not '{:?}', at line {} char {}", d,  VarTypes::Bool, position.1, position.0)
        }
        // scoping
        let mut definitions = vec![];
        for statement in compound {
            let ret = self.individual_check(statement);
            if ret.1 != 0 {
                definitions.push(ret)
            }
        }
        for defined in definitions {
            match defined.1 {
                1 => { self.defined_var.remove(&defined.0).expect("huh?"); self.removed.push(defined.0)},
                _ => unimplemented!()
            }
        }
        // unimplemented!()
    }

    fn func_call_check(&mut self, func_name: Token, args: Vec<Token>){
        if !self.defined_function.contains_key(&func_name.value){
            panic!("function '{}' does not exists, error at line {} char {}", func_name.value, func_name.y, func_name.x);
        }
        let function_referred_to = self.defined_function.get(&func_name.value).unwrap();
        if args.len() != function_referred_to.0.len(){
            panic!("expected {} arguments {} were given, error at line {} char {}", function_referred_to.0.len(), args.len(), func_name.y, func_name.x);
        }

        for ind in 0..args.len() {
            let given_arg = args.get(ind).unwrap();
            let receive_arg = function_referred_to.0.get(ind).unwrap();
            if given_arg.is_data_type(){
                match receive_arg {
                    ArgTypes::Int => {
                        if  data_token_type_to_types(&given_arg.token_type).unwrap() != VarTypes::Int {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", receive_arg, data_token_type_to_types(&given_arg.token_type), given_arg.y, given_arg.x);
                        }
                    },
                    ArgTypes::Str => {
                        if  data_token_type_to_types(&given_arg.token_type).unwrap() != VarTypes::Str {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", receive_arg, data_token_type_to_types(&given_arg.token_type), given_arg.y, given_arg.x);
                        }
                    },
                    ArgTypes::Float => {
                        if  data_token_type_to_types(&given_arg.token_type).unwrap() != VarTypes::Float {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", receive_arg, data_token_type_to_types(&given_arg.token_type), given_arg.y, given_arg.x);
                        }
                    },
                    ArgTypes::Bool => {
                        if  data_token_type_to_types(&given_arg.token_type).unwrap() != VarTypes::Bool {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", receive_arg, data_token_type_to_types(&given_arg.token_type), given_arg.y, given_arg.x);
                        }
                    },
                    ArgTypes::Any => {

                    },
                    ArgTypes::Variadic(_) => {
                        unimplemented!()
                    },
                    ArgTypes::Struct => {
                        unimplemented!()
                    },
                }
            } else if given_arg.token_type == TokenType::Identifier {
                if self.defined_var.contains_key(&given_arg.value){
                    let referred = self.defined_var.get(&given_arg.value).unwrap();

                } else {
                    unimplemented!()
                }
            } else {
                unimplemented!()
            }
        }
    }
    fn condition_check(&mut self, conditions: Vec<(Vec<Parsed>, Vec<Token>, (u32, u32))>){
        for condition in conditions{
            self.if_check(condition.0, condition.1, condition.2)
        }
    }

    fn individual_check(&mut self, to_check: Parsed) -> (String, u32) {
        match to_check {
            Parsed::VariableAssignment(name, var_type, values) => {
                let ret = self.var_check(name, var_type, values);
                self.defined_var.insert(ret.0.clone(), ret.1);
                (ret.0, 1)
            },
            Parsed::Conditions(conditions) => {
                self.condition_check(conditions);
               ("".to_string(), 0)
            },
            Parsed::FuncCall(func_name, Args) => {
                self.func_call_check(func_name, Args);
                ("".to_string(), 0)
            }
            Parsed::VariableReassignment(name, values) => {
                self.var_reassign_check(name, values);
                ("".to_string(), 0)
            },
            _ => unimplemented!()
        }
    }

    pub fn check_program(&mut self, program: Parsed) {
        let mut statements = vec![];
        if let Parsed::Program(com_statements) = program {
            statements = com_statements;
        } else {
            panic!("not a program")
        }

        for statement in statements {
            self.individual_check(statement);
        }
    }
}