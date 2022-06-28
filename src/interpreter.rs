use std::collections::HashMap;
use eval::eval;
use crate::{Parsed, parser};
use crate::check::Checker;
use crate::lexer::{Token, TokenType};
use crate::parser::VarTypes;




#[derive(PartialEq, Debug, Clone)]
enum Value {
    Int(i128),
    String(String),
    Bool(bool),
    Float(f64)
}

pub struct Interpreter {
    program: Vec<Parsed>,
    defined_var: HashMap<String, (VarTypes, Value)>,
    defined_func: HashMap<String, (u32, Vec<VarTypes>)>,
    built_in_func: Vec<String>,
    defined_struct: HashMap<String, ()>
}

impl Interpreter {
    pub fn new(prog: Parsed) -> Self {
        if let Parsed::Program(program) = prog  {
            Self {
                program,
                defined_var: HashMap::new(),
                defined_func: HashMap::new(),
                built_in_func: vec!["iprint".to_string(), "fprint".to_string(), "bprint".to_string(), "sprint".to_string(), ],
                defined_struct: HashMap::new()
            }
        } else {
            panic!("huh? what?")
        }
    }
    fn evaluate(&self, eval: Vec<Token>) -> String {
        let mut to_eval = "".to_string();
        let mut ind = -1;
        loop {
            ind += 1;
            let initial =  eval.get(ind as usize);
            if initial.is_none(){
                break
            }
            let mut tok: Token = initial.unwrap().clone();

            if tok.token_type == TokenType::MathOperation || tok.is_data_type(){
                to_eval += &tok.true_value()
            } else if self.defined_var.contains_key(&tok.value){
                let d = self.defined_var.get(&tok.value).unwrap().1.clone();
                match d {
                    Value::String(val) => to_eval += &val,
                    Value::Int(val) => to_eval += &val.to_string(),
                    Value::Float(val) => to_eval += &val.to_string(),
                    Value::Bool(val) => to_eval += &val.to_string(),
                }
            } else {
                unimplemented!()
            }
        }
        eval::eval(&to_eval).expect("sir! you got a shitty error").to_string()
    }
    fn var_reassignment(&mut self, var_name: Token, var_value: Vec<Token>){
        let mut reassign = self.defined_var.remove(&var_name.value).unwrap();
        self.defined_var.insert(var_name.value.clone(), reassign.clone());
        let var_type = reassign.0.clone();
        match var_type {
            VarTypes::Str => {
                unimplemented!()
            },
            VarTypes::Int => {
                let ret_eval = self.evaluate(var_value).parse::<i128>().expect("sir! you got another shitty error");
                self.defined_var.insert(var_name.value, (var_type, Value::Int(ret_eval)));
            },
            VarTypes::Float => {
                unimplemented!()
            },
            VarTypes::Bool => {
                unimplemented!()
            }
        }
    }

    fn var_assignment(&mut self, var_name: Token, var_type: VarTypes, var_value: Vec<Token>){
        match var_type {
            VarTypes::Str => {
                unimplemented!()
            },
            VarTypes::Int => {
                let ret_eval = self.evaluate(var_value).parse::<i128>().expect("sir! you got another shitty error");
                self.defined_var.insert(var_name.value, (var_type, Value::Int(ret_eval)));
            },
            VarTypes::Float => {
                unimplemented!()
            },
            VarTypes::Bool => {
                unimplemented!()
            }
        }
    }
    fn built_in_funcs(&mut self, func_name: Token, func_args: Vec<Token>) -> Option<Value>{
        match &*func_name.value {
            "iprint" => {
                let arg = func_args[0 as usize].clone();
                if arg.is_integer(){
                    println!("{}", arg.value)
                } else if arg.token_type == TokenType::Identifier {
                    let arg_name = arg.value;
                    if self.defined_var.contains_key(&arg_name){
                        let referred = self.defined_var.get(&arg_name).unwrap();
                        match referred.1 {
                            Value::Int(val) => println!("{}", val),
                            _ => panic!("type checker failed me")
                        }
                    } else {
                        unimplemented!("not yet '{}'", func_name.value)
                    }
                } else {
                    panic!("how did you get here?")
                }
            }
            _ => unimplemented!("not yet '{}'", func_name.value)
        }
        None
    }
    fn func_call(&mut self, func_name: Token, func_args: Vec<Token>){
        if self.built_in_func.contains(&func_name.value){
            self.built_in_funcs(func_name, func_args);
        } else {
            unimplemented!()
        }
    }
    pub fn run(&mut self){
        let program = self.program.clone();
        for part in program {
            match part {
                Parsed::VariableAssignment(var_name, var_type, var_value) => {
                    self.var_assignment(var_name.clone(), var_type.clone(), var_value.clone())
                },
                Parsed::FuncCall(func_name, func_args) => {
                    self.func_call(func_name, func_args);
                },
                 Parsed::VariableReassignment(var_name, var_value) => {
                    self.var_reassignment(var_name.clone(), var_value.clone())
                },
                _ => unimplemented!()
            }
        }
    }
}
