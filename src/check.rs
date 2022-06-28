use std::collections::HashMap;
use crate::lexer::{Token, TokenType};
use crate::Parsed;
use crate::parser::{data_token_type_to_types, str_to_types, VarTypes};

pub struct Checker
{
    defined_var: HashMap<String, VarTypes>,
    defined_struct: HashMap<String, VarTypes>,
    //                                 args type      return type
    defined_function: HashMap<String, (Vec<VarTypes>, Vec<VarTypes>)>,
    removed: Vec<String>
}

impl Checker {
    pub fn new() -> Self {
        let mut new = Self {
            defined_var: HashMap::new(),
            defined_struct: HashMap::new(),
            defined_function: HashMap::new(),
            removed: vec![]
        };
        new.defined_function.insert("iprint".to_string(), (vec![VarTypes::Int], vec![]));
        new.defined_function.insert("sprint".to_string(), (vec![VarTypes::Str], vec![]));
        new.defined_function.insert("fprint".to_string(), (vec![VarTypes::Float], vec![]));
        new.defined_function.insert("bprint".to_string(), (vec![VarTypes::Bool], vec![]));
        new
    }
    fn var_check(&mut self, name: Token, var_type: VarTypes, values: Vec<Token>) -> (String, VarTypes){
        if self.defined_var.contains_key(&name.value){
            panic!("variable '{}' already exists, error at line {} char {}", name.value, name.y, name.x);
        }
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

    fn if_check(&mut self, compound: Vec<Parsed>, condition: Vec<String>, position: (u32, u32)){
        println!("{:?}", condition);
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
        unimplemented!()
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
                if  &data_token_type_to_types(&given_arg.token_type).unwrap() != receive_arg {
                    panic!("Expected {:?} got {:?}, error at line {} char {}", receive_arg, data_token_type_to_types(&given_arg.token_type), given_arg.y, given_arg.x);
                }
            } else if given_arg.token_type == TokenType::Identifier {
                if self.defined_var.contains_key(&given_arg.value){
                    let referred = self.defined_var.get(&given_arg.value).unwrap();
                    if  referred != receive_arg {
                        panic!("Expected {:?} got {:?}, error at line {} char {}", receive_arg, data_token_type_to_types(&given_arg.token_type), given_arg.y, given_arg.x);
                    }
                } else {
                    unimplemented!()
                }
            } else {
                unimplemented!()
            }
        }
    }

    fn individual_check(&mut self, to_check: Parsed) -> (String, u32) {
        match to_check {
            Parsed::VariableAssignment(name, var_type, values) => {
                let ret = self.var_check(name, var_type, values);
                self.defined_var.insert(ret.0.clone(), ret.1);
                (ret.0, 1)
            },
            Parsed::If(compound, condition, position) => {
                self.if_check(compound, condition, position);
               ("".to_string(), 0)
            },
            Parsed::FuncCall(FuncName, Args) => {
                self.func_call_check(FuncName, Args);
                ("".to_string(), 0)
            },
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