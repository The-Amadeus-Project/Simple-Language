use std::collections::HashMap;
use crate::{Parsed, parser};
use crate::check::Checker;
use crate::lexer::{Token, TokenType};
use crate::parser::VarTypes;


fn substring(str: String, start_index: i32, end_index: i32) ->  Option<String>
{
    if end_index <= start_index
    {
        return None;
    }
    let substring = (&str[(start_index as usize)..(end_index as usize)]).to_string();
    Option::from(substring)

}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Int(i128),
    String(String),
    Bool(bool),
    Float(f64)
}

pub struct Interpreter {
    program: Vec<Parsed>,
    defined_variable: HashMap<String, (VarTypes, Value)>,
    defined_function: HashMap<String, (u32, Vec<VarTypes>)>,
    built_in_function: Vec<String>,
    defined_struct: HashMap<String, ()>
}

impl Interpreter {
    pub fn new(prog: Parsed) -> Self {
        if let Parsed::Program(program) = prog  {
            Self {
                program,
                defined_variable: HashMap::new(),
                defined_function: HashMap::new(),
                built_in_function: vec!["out".to_string(), ],
                defined_struct: HashMap::new()
            }
        } else {
            panic!("huh? what? Expected a parsed program")
        }
    }
    fn evaluate(&self, to_evaluate: Vec<Token>) -> Value {
        for tok in to_evaluate {
            println!()
        }
        unimplemented!()
    }
    fn var_assign_template(&mut self, variable_name: Token, variable_type: VarTypes, variable_value: Vec<Token>) -> (String, u32){
        match variable_type {
            VarTypes::Str => {
                let ret_eval = self.evaluate(variable_value);
                self.defined_variable.insert(variable_name.value.clone(), (variable_type, ret_eval));
                (variable_name.value.clone(), 1)
            },
            VarTypes::Int => {
                let ret_eval = self.evaluate(variable_value);
                self.defined_variable.insert(variable_name.value.clone(), (variable_type, ret_eval));
                (variable_name.value.clone(), 1)
            },
            VarTypes::Float => {
                let ret_eval = self.evaluate(variable_value);
                self.defined_variable.insert(variable_name.value.clone(), (variable_type, ret_eval));
                (variable_name.value.clone(), 1)
            },
            VarTypes::Bool => {
                let ret_eval = self.evaluate(variable_value);
                self.defined_variable.insert(variable_name.value.clone(), (variable_type, ret_eval));
                (variable_name.value.clone(), 1)
            },
            VarTypes::Struct => {
                unimplemented!()
            },
        }
    }
    fn var_reassignment(&mut self, var_name: Token, var_value: Vec<Token>){
        let mut referred_variable = self.defined_variable.get(&var_name.value).unwrap().clone();
        let referred_variable_type = referred_variable.0.clone();
        self.var_assign_template(var_name, referred_variable_type, var_value);
    }
    fn var_assignment(&mut self, var_name: Token, var_type: VarTypes, var_value: Vec<Token>) -> (String, u32){
        self.var_assign_template(var_name, var_type, var_value)
    }
    fn built_in_funcs(&mut self, func_name: Token, func_args: Vec<Token>) -> Option<Value>{
        match &*func_name.value {
            "out" => {
                let function_argument = func_args[0 as usize].clone();
                if function_argument.is_data_type(){
                    println!("{}", function_argument.value)
                } else if function_argument.token_type == TokenType::Identifier {
                    let argument_name = function_argument.value;
                    if self.defined_variable.contains_key(&argument_name){
                        let referred_variable = self.defined_variable.get(&argument_name).unwrap();
                        match &referred_variable.1 {
                            Value::Int(val) => println!("{}", val),
                            Value::String(val) => println!("{}", val),
                            Value::Float(val) => println!("{}", val),
                            Value::Bool(val) => println!("{}", val),

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
        if self.built_in_function.contains(&func_name.value){
            self.built_in_funcs(func_name, func_args);
        } else {
            unimplemented!()
        }
    }
    fn conditions(&mut self, cond: Vec<(Vec<Parsed>, Vec<Token>, (u32, u32))>){
        for statement in cond {
            let return_of_evaluating_condition = self.evaluate(statement.1);
            let result_of_evaluating_condition: bool;
            if let Value::Bool(result) = return_of_evaluating_condition {
                result_of_evaluating_condition = result;
            } else {
                panic!("its not a boolean")
            }
            if result_of_evaluating_condition {
                let mut scope = vec![];
                for statement in statement.0  {
                    let ret = self.individuals(statement);
                    if ret.1 != 0 {
                       scope.push(ret)
                    }
                }
                for defined in scope {
                    match defined.1 {
                        1 => { self.defined_variable.remove(&defined.0).expect("huh?");},
                        _ => unimplemented!()
                    }
                }
              break
            }
        }

    }
    fn individuals(&mut self, part: Parsed) -> (String, u32){
        match part {
                Parsed::VariableAssignment(var_name, var_type, var_value) => {
                    let (variable_name, variable_type) = self.var_assignment(var_name.clone(), var_type.clone(), var_value.clone());
                    (variable_name, variable_type)
                },
                Parsed::FuncCall(func_name, func_args) => {
                    self.func_call(func_name, func_args);
                    ("".to_string(), 0)
                },
                Parsed::VariableReassignment(var_name, var_value) => {
                    self.var_reassignment(var_name.clone(), var_value.clone());
                    ("".to_string(), 0)
                },
                Parsed::Conditions(cond) => {
                    self.conditions(cond);
                    ("".to_string(), 0)
                },
                _ => unimplemented!()
            }
    }
    pub fn run(&mut self){
        let program = self.program.clone();
        for part in program {
            self.individuals(part);
        }
    }
}
