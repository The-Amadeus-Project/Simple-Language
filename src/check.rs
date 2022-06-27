use std::collections::HashMap;
use crate::lexer::{Token, TokenType};
use crate::Parsed;
use crate::parser::{data_token_type_to_types, VarTypes};

pub struct Checker
{
    defined_var: HashMap<String, VarTypes>,
    defined_struct: HashMap<String, VarTypes>,
    defined_function: HashMap<String, VarTypes>,
    removed: Vec<String>
}

impl Checker {
    pub fn new() -> Self {
        Self {
            defined_var: HashMap::new(),
            defined_struct: HashMap::new(),
            defined_function: HashMap::new(),
            removed: vec![]
        }
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
                    if var_type != *reference {
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

    fn if_check(&mut self, compound: Vec<Parsed>, condition: Vec<String>, position: (u32, u32)){
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
    }

    fn individual_check(&mut self, to_check: Parsed) -> (String, u32) {
        match to_check {
            Parsed::Var(name, var_type, values) => {
                let ret = self.var_check(name, var_type, values);
                self.defined_var.insert(ret.0.clone(), ret.1);
                return (ret.0, 1)
            },
            Parsed::If(compound, condition, position) => {
                self.if_check(compound, condition, position);
                return ("".to_string(), 0)
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