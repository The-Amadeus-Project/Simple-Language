
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
    // Converts TokenType -> ParseType
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
    // Converts VarTypes -> ParseType
    match token {
        VarTypes::Bool => Some(ParseTypes::Bool),
        VarTypes::Int => Some(ParseTypes::Int),
        VarTypes::Float => Some(ParseTypes::Float),
        VarTypes::Str => Some(ParseTypes::String),
        _ => None
    }
}

fn parse_type_to_var_types(token: &ParseTypes) -> Option<VarTypes> {
    // Converts ParseType -> VarTypes
    match token {
       ParseTypes::String => Some(VarTypes::Str),
       ParseTypes::Bool => Some(VarTypes::Bool),
       ParseTypes::Int => Some(VarTypes::Int),
       ParseTypes::Float => Some(VarTypes::Float),
        _ => None
    }
}


struct TypeEvaluator {
    to_parse: Vec<Token>,
    defined_var: HashMap<String, VarTypes>,
    defined_struct: HashMap<String, VarTypes>,
    //                                 args type      return type
    defined_function: HashMap<String, (Vec<ArgTypes>, Vec<VarTypes>)>,
    removed: Vec<String>
}

impl TypeEvaluator {
    fn new(to_evaluate: Vec<Token>,
           defined_var: HashMap<String, VarTypes>,
           defined_struct: HashMap<String, VarTypes>,
           defined_function: HashMap<String, (Vec<ArgTypes>, Vec<VarTypes>)>,
           removed: Vec<String>) -> Self
    {

        Self {
            to_parse: to_evaluate,
            defined_var,
            defined_struct,
            defined_function,
            removed
        }
    }

    fn eval(&mut self, evaluate: Vec<(ParseTypes, u32, u32)>) -> (ParseTypes, u32, u32) {
        let mut curly_brackets_count = 0; // represents {}
        let mut values_to_eval = vec![]; // to eval, to recurse return gets added here
        let mut values_to_recurse = vec![]; // to recurse
        for part in evaluate {
            if part.0 == ParseTypes::ParenthesisClose {
                curly_brackets_count -= 1;
                if curly_brackets_count != 0 {
                    values_to_recurse.push(part);
                } else if curly_brackets_count == 0 {
                    // runs a recursion of self.eval
                    values_to_eval.push(self.eval(values_to_recurse.clone()));
                    values_to_recurse.clear();
                }
            } else if part.0 == ParseTypes::ParenthesisOpen {
                if curly_brackets_count != 0 {
                    // will be sent to recurse
                    values_to_recurse.push(part)
                }
                curly_brackets_count += 1;
            } else if curly_brackets_count != 0 {
                // will be sent to recurse
                values_to_recurse.push(part);
            } else {
                // stuff that isn't inside {}
                values_to_eval.push(part);
            }
        }
        let mut current_index = -1;
        let mut round_finished = 0;
        while values_to_eval.len() != 1 {
            current_index += 1;
            if round_finished == 1 {
                // if after going another round and nothing changed
                break
            }
            if current_index >= values_to_eval.len() as i32{
                current_index = 0;
                round_finished += 1;
            }
            let current = values_to_eval.get(current_index as usize).unwrap();
            if current.0 == ParseTypes::Math {
                round_finished = 0;
                if values_to_eval.get((current_index - 1) as usize).unwrap().0 == values_to_eval.get((current_index + 1) as usize).unwrap().0{
                    values_to_eval.remove((current_index - 1) as usize);
                    values_to_eval.remove((current_index - 1) as usize);
                } else {
                    // when before and after arent of same types
                    let number_before = values_to_eval.get((current_index - 1) as usize).unwrap();
                    let number_after = values_to_eval.get((current_index + 1) as usize).unwrap();
                    panic!("Expected {:?} got '{:?}' instead, at line {} char {}", number_before.0, number_after.0, number_after.2, number_after.1)
                }
            }
        }

        let mut current_index2 = -1;
        let mut round_finished2 = 0;
        while values_to_eval.len() != 1 {
            current_index2 += 1;
            if round_finished2 == 1 {
                break
                // if after going another round and nothing changed
            }
            if current_index2 >= values_to_eval.len() as i32{
                current_index2 = 0;
                round_finished2 += 1;
            }
            let current = values_to_eval.get(current_index2 as usize).unwrap();
            if current.0 == ParseTypes::Comparison {
                if values_to_eval.get((current_index2 - 1) as usize).unwrap().0 == values_to_eval.get((current_index2 + 1) as usize).unwrap().0{
                    values_to_eval.remove((current_index2 - 1) as usize);
                    values_to_eval.remove((current_index2 - 1) as usize);
                    let d = values_to_eval.remove((current_index2 - 1) as usize);
                    values_to_eval.insert((current_index2 - 1) as usize, (ParseTypes::Bool, d.1, d.2));
                } else {
                    // when before and after arent of same types
                    let number_before = values_to_eval.get((current_index2 - 1) as usize).unwrap();
                    let number_after = values_to_eval.get((current_index2 + 1) as usize).unwrap();
                    panic!("Expected {:?} got '{:?}' instead, at line {} char {}", number_before.0, number_after.0, number_after.2, number_after.1)
                }
            }
        }

        if values_to_eval.len() > 1 {
            panic!("Eval error")
        }
        values_to_eval[0 as usize]
    }

    fn parse(&mut self) -> ParseTypes {
        let mut to_eval = vec![];
        for token in &self.to_parse {
            // converts tokens to parse types for easier evaluation
            if token.token_type != TokenType::Identifier {
                to_eval.push((token_type_to_parse_type(token).expect(&*format!("{:?}", token)), token.x, token.y))
            } else {
                if self.defined_var.contains_key(&token.value){
                    let var = var_types_to_parse_type(self.defined_var.get(&token.value).unwrap()).unwrap();
                    to_eval.push((var, token.x, token.y));
                } else if self.defined_function.contains_key(&token.value) {
                    let referred_function = self.defined_function.get(&token.value).unwrap();
                    if referred_function.1.len() > 0 {
                        unimplemented!()
                    } else {

                    }

                } else {
                    unimplemented!("{:?}", token)
                }
            }
        }
        self.eval(to_eval).0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ArgTypes {
    // types for function arguments
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
    // converts VarTypes -> ArgTypes
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
        new.defined_function.insert("out".to_string(), (vec![ArgTypes::Any], vec![])); // print function
        new
    }
    fn variable_check(&mut self, var_name: Token, var_type: VarTypes, var_given_values: Vec<Token>) -> (String, VarTypes){
        if self.defined_var.contains_key(&var_name.value){
            // checks if variable name already exists
            panic!("variable '{}' already exists, error at line {} char {}", var_name.value, var_name.y, var_name.x);
        }
        if var_given_values.len() % 2 == 0  {
            // will tell you if a expression is wrong because its not odd
            // ex:
            // 1 + 1 -> odd
            // (4 % 6) * 4 -> odd
            // 13 >= 13 -> odd

            let last_ind = var_given_values.len() - 1;
            let last = var_given_values.get(last_ind).unwrap();
            panic!("something wrong with expression just doesnt know, at line {} char {}", last.y, last.x)
        }
        // creates a new TypeEvaluator
        let mut type_evaluator = TypeEvaluator::new(
            var_given_values,
            self.defined_var.clone(),
            self.defined_struct.clone(),
            self.defined_function.clone(),
            self.removed.clone());

        // runs the evaluator
        let type_evaluator_return = parse_type_to_var_types(&type_evaluator.parse()).expect("please help me");

        // checker for return evaluation type and var type
        if type_evaluator_return != var_type {
            panic!("expression result in '{:?}' and not '{:?}', at line {} char {}", type_evaluator_return, var_type, var_name.y, var_name.x)
        }
        (var_name.value.clone(), var_type)
    }

    fn var_reassign_check(&mut self, var_name: Token, given_values: Vec<Token>){
        if !self.defined_var.contains_key(&var_name.value){
            // wont reassign if variable doesnt exists
            panic!("variable '{}' does not exists, error at line {} char {}", var_name.value, var_name.y, var_name.x);
        }
        let var_type = self.defined_var.get(&var_name.value).unwrap().clone();
        if given_values.len() % 2 == 0  {
            // will tell you if a expression is wrong because its not odd
            // ex:
            // 1 + 1 -> odd
            // (4 % 6) * 4 -> odd
            // 13 >= 13 -> odd

            let last_index_of_given_values = given_values.len() - 1;
            let last_token_of_given_values = given_values.get(last_index_of_given_values).unwrap();
            panic!("something wrong with expression just doesnt know, at line {} char {}", last_token_of_given_values.y, last_token_of_given_values.x)
        }
        // creates a new TypeEvaluator
        let mut type_evaluator = TypeEvaluator::new(
            given_values,
            self.defined_var.clone(),
            self.defined_struct.clone(),
            self.defined_function.clone(),
            self.removed.clone());

        // runs the evaluator
        let type_evaluator_return = parse_type_to_var_types(&type_evaluator.parse()).expect("please help me");

        // checker for return evaluation type and var type
        if type_evaluator_return != var_type {
            panic!("expression result in '{:?}' and not '{:?}', at line {} char {}", type_evaluator_return, var_type, var_name.y, var_name.x)
        }

    }

    fn individual_conditional_check(&mut self, compound_statements: Vec<Parsed>, if_condition: Vec<Token>, if_position: (u32, u32)){
        let mut type_evaluator = TypeEvaluator::new(
            if_condition,
            self.defined_var.clone(),
            self.defined_struct.clone(),
            self.defined_function.clone(),
            self.removed.clone());
        let type_evaluator_return = parse_type_to_var_types(&type_evaluator.parse()).expect("please help me");
        if type_evaluator_return != VarTypes::Bool {
            panic!("expression result in '{:?}' and not '{:?}', at line {} char {}", type_evaluator_return, VarTypes::Bool, if_position.1, if_position.0)
        }
        // scoping
        let mut local_scope = vec![];
        for statement in compound_statements {
            let return_value = self.individual_check(statement);
            if return_value.1 != 0 {
                local_scope.push(return_value)
            }
        }
        for defined in local_scope {
            match defined.1 {
                1 => { self.defined_var.remove(&defined.0).expect("huh?"); self.removed.push(defined.0)},
                _ => unimplemented!()
            }
        }
        // unimplemented!()
    }

    fn func_call_check(&mut self, func_name: Token, func_args: Vec<Token>){
        if !self.defined_function.contains_key(&func_name.value){
            panic!("function '{}' does not exists, error at line {} char {}", func_name.value, func_name.y, func_name.x);
        }
        let function_referred_to = self.defined_function.get(&func_name.value).unwrap();
        if func_args.len() != function_referred_to.0.len(){
            panic!("expected {} arguments {} were given, error at line {} char {}", function_referred_to.0.len(), func_args.len(), func_name.y, func_name.x);
        }

        for ind in 0..func_args.len() {
            let given_argument = func_args.get(ind).unwrap();
            let expected_argument_type = function_referred_to.0.get(ind).unwrap();
            if given_argument.is_data_type(){
                match expected_argument_type {
                    ArgTypes::Int => {
                        if  data_token_type_to_types(&given_argument.token_type).unwrap() != VarTypes::Int {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", expected_argument_type, data_token_type_to_types(&given_argument.token_type), given_argument.y, given_argument.x);
                        }
                    },
                    ArgTypes::Str => {
                        if  data_token_type_to_types(&given_argument.token_type).unwrap() != VarTypes::Str {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", expected_argument_type, data_token_type_to_types(&given_argument.token_type), given_argument.y, given_argument.x);
                        }
                    },
                    ArgTypes::Float => {
                        if  data_token_type_to_types(&given_argument.token_type).unwrap() != VarTypes::Float {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", expected_argument_type, data_token_type_to_types(&given_argument.token_type), given_argument.y, given_argument.x);
                        }
                    },
                    ArgTypes::Bool => {
                        if  data_token_type_to_types(&given_argument.token_type).unwrap() != VarTypes::Bool {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", expected_argument_type, data_token_type_to_types(&given_argument.token_type), given_argument.y, given_argument.x);
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
            } else if given_argument.token_type == TokenType::Identifier {
                if self.defined_var.contains_key(&given_argument.value){
                    let referred_variable_type = self.defined_var.get(&given_argument.value).unwrap().clone();

                    match expected_argument_type {
                    ArgTypes::Int => {
                        if  referred_variable_type != VarTypes::Int {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", expected_argument_type, data_token_type_to_types(&given_argument.token_type), given_argument.y, given_argument.x);
                        }
                    },
                    ArgTypes::Str => {
                        if  referred_variable_type != VarTypes::Str {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", expected_argument_type, data_token_type_to_types(&given_argument.token_type), given_argument.y, given_argument.x);
                        }
                    },
                    ArgTypes::Float => {
                        if  referred_variable_type != VarTypes::Float {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", expected_argument_type, data_token_type_to_types(&given_argument.token_type), given_argument.y, given_argument.x);
                        }
                    },
                    ArgTypes::Bool => {
                        if  referred_variable_type != VarTypes::Bool {
                            panic!("Expected {:?} got {:?}, error at line {} char {}", expected_argument_type, data_token_type_to_types(&given_argument.token_type), given_argument.y, given_argument.x);
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
            self.individual_conditional_check(condition.0, condition.1, condition.2)
        }
    }

    fn individual_check(&mut self, to_check: Parsed) -> (String, u32) {
        match to_check {
            Parsed::VariableAssignment(name, var_type, values) => {
                let return_value = self.variable_check(name, var_type, values);
                self.defined_var.insert(return_value.0.clone(), return_value.1);
                (return_value.0, 1)
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