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
    defined_struct: HashMap<String, ()>
}

impl Interpreter {
    pub fn new(prog: Parsed) -> Self {
        if let Parsed::Program(program) = prog  {
            Self {
                program,
                defined_var: HashMap::new(),
                defined_func: HashMap::new(),
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
                    _ => panic!("HOW?!!!")
                }
            } else {
                unimplemented!()
            }
        }
        eval::eval(&to_eval).expect("sir! you got a shitty error").to_string()
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
    pub fn run(&mut self){
        let program = self.program.clone();
        for part in program {
            match part {
                Parsed::VariableAssignment(var_name, var_type, var_value) =>
                self.var_assignment(var_name.clone(), var_type.clone(), var_value.clone()),
                _ => unimplemented!()
            }
        }
    }
}
