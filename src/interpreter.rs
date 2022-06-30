use std::collections::HashMap;
use eval::eval;
use crate::{Parsed, parser};
use crate::check::Checker;
use crate::lexer::{Token, TokenType};
use crate::parser::VarTypes;


fn substring(str: String, start: i32, end: i32) ->  Option<String>
{
    if end <= start
    {
        return None;
    }
    let ss = (&str[(start as usize)..(end as usize)]).to_string();
    Option::from(ss)

}

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
                built_in_func: vec!["out".to_string(), ],
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
            let initial: Option<&Token> =  eval.get(ind as usize);
            if initial.is_none(){
                break
            }
            let mut tok: Token = initial.unwrap().clone();

            if tok.token_type == TokenType::MathOperation || tok.token_type == TokenType::ComparisonOperation || tok.is_data_type() {
                to_eval += &tok.true_value()
            } else if self.defined_var.contains_key(&tok.value){
                let d = self.defined_var.get(&tok.value).unwrap().1.clone();
                match d {
                    Value::String(val) => to_eval += &*("\"".to_string() + &val + &*"\"".to_string()),
                    Value::Int(val) => to_eval += &val.to_string(),
                    Value::Float(val) => to_eval += &val.to_string(),
                    Value::Bool(val) => to_eval += &val.to_string(),
                }
            } else {
                unimplemented!("{:?}", tok)
            }
        }
        eval::eval(&to_eval).expect("sir! you got a shitty error").to_string()
    }
    fn var_assign_template(&mut self, var_name: Token, var_type: VarTypes, var_value: Vec<Token>) -> (String, u32){
        match var_type {
            VarTypes::Str => {
                let ret_eval = self.evaluate(var_value).parse::<String>().expect("sir! you got another shitty error");
                self.defined_var.insert(var_name.value.clone(), (var_type, Value::String(substring(ret_eval.clone(), 1, (&ret_eval.len() - 1) as i32).unwrap())));
                (var_name.value.clone(), 1)
            },
            VarTypes::Int => {
                let ret_eval = self.evaluate(var_value).parse::<i128>().expect("sir! you got another shitty error");
                self.defined_var.insert(var_name.value.clone(), (var_type, Value::Int(ret_eval)));
                (var_name.value.clone(), 1)
            },
            VarTypes::Float => {
                let ret_eval = self.evaluate(var_value).parse::<f64>().expect("sir! you got another shitty error");
                self.defined_var.insert(var_name.value.clone(), (var_type, Value::Float(ret_eval)));
                (var_name.value.clone(), 1)
            },
            VarTypes::Bool => {
                let ret_eval = self.evaluate(var_value).parse::<bool>().expect("sir! you got another shitty error");
                self.defined_var.insert(var_name.value.clone(), (var_type, Value::Bool(ret_eval)));
                (var_name.value.clone(), 1)
            },
            VarTypes::Struct => {
                unimplemented!()
            },
        }
    }
    fn var_reassignment(&mut self, var_name: Token, var_value: Vec<Token>){
        let mut reassign = self.defined_var.get(&var_name.value).unwrap().clone();
        let var_type = reassign.0.clone();
        self.var_assign_template(var_name, var_type, var_value);
    }
    fn var_assignment(&mut self, var_name: Token, var_type: VarTypes, var_value: Vec<Token>) -> (String, u32){
        self.var_assign_template(var_name, var_type, var_value)
    }
    fn built_in_funcs(&mut self, func_name: Token, func_args: Vec<Token>) -> Option<Value>{
        match &*func_name.value {
            "out" => {
                let arg = func_args[0 as usize].clone();
                if arg.is_data_type(){
                    println!("{}", arg.value)
                } else if arg.token_type == TokenType::Identifier {
                    let arg_name = arg.value;
                    if self.defined_var.contains_key(&arg_name){
                        let referred = self.defined_var.get(&arg_name).unwrap();
                        match &referred.1 {
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
        if self.built_in_func.contains(&func_name.value){
            self.built_in_funcs(func_name, func_args);
        } else {
            unimplemented!()
        }
    }
    fn conditions(&mut self, cond: Vec<(Vec<Parsed>, Vec<Token>, (u32, u32))>){
        for block in cond {
            let res = self.evaluate(block.1).parse::<bool>().expect("typ checker failed");
            if res {
                let mut definitions = vec![];
                for statement in block.0  {
                    let ret = self.individuals(statement);
                    if ret.1 != 0 {
                       definitions.push(ret)
                    }
                }
                for defined in definitions {
                    match defined.1 {
                        1 => { self.defined_var.remove(&defined.0).expect("huh?");},
                        _ => unimplemented!()
                    }
                }
            }
        }

    }
    fn individuals(&mut self, part: Parsed) -> (String, u32){
        match part {
                Parsed::VariableAssignment(var_name, var_type, var_value) => {
                    let d = self.var_assignment(var_name.clone(), var_type.clone(), var_value.clone());
                    ("".to_string(), 0)
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
