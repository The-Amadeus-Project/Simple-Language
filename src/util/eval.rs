use crate::util::eval_lexer::{MathLexer, MathToken, MathTokenType};

pub fn eval_string(to: String) -> MathToken {
    if to.is_empty(){
        panic!("Empty Expression")
    }
    let mut lexer_result = MathLexer::new().lex_text(to);
    eval(lexer_result)
}

pub fn eval(to: Vec<MathToken>) -> MathToken {
    if to.is_empty(){
        panic!("Empty Expression")
    }
    let mut temp_list = vec![];
    let mut scope = 0;
    let mut tokens_parsing_now = vec![];
    for token_result in to {
        if token_result.token_type == MathTokenType::ParenthesisClose {
            scope -= 1;
            if scope > 0 {
                temp_list.push(token_result.clone())
            } else if scope == 0 {
                tokens_parsing_now.push(eval(temp_list.clone()));
                temp_list.clear()
            } else {
                panic!("{} : over closing of parenthesis", scope)
            }
        } else if token_result.token_type == MathTokenType::ParenthesisOpen {
            if scope > 0 {
                temp_list.push(token_result.clone())
            }
            scope += 1;
        } else if scope > 0 {
            temp_list.push(token_result.clone())
        } else {
            tokens_parsing_now.push(token_result.clone())
        }
    }

    let mut index_for_mul_div = -1;
    let mut round_times_mul_div = 0;
    while tokens_parsing_now.len() != 1 {
        index_for_mul_div += 1;
        if round_times_mul_div > 1 {
            break
        }
        else if index_for_mul_div >= tokens_parsing_now.len() as i32 {
            index_for_mul_div = 0;
            round_times_mul_div += 1;
        }
        let part = tokens_parsing_now.get(index_for_mul_div as usize).unwrap().clone();
        if part.token_type == MathTokenType::Multiplication {
            round_times_mul_div = 0;
            if index_for_mul_div == 0 || index_for_mul_div == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_mul_div - 1) as usize);
            tokens_parsing_now.remove((index_for_mul_div - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_mul_div - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x);
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() * last.value.parse::<i128>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_mul_div - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() * last.value.parse::<f64>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_mul_div - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        panic!("invalid");
                    },
                    MathTokenType::Boolean => {
                        panic!("invalid");
                    },

                    _ => {panic!("wong")}
                }

            }
        }
        else if part.token_type == MathTokenType::Division {
            round_times_mul_div = 0;
            if index_for_mul_div == 0 || index_for_mul_div == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_mul_div - 1) as usize);
            tokens_parsing_now.remove((index_for_mul_div - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_mul_div - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x);
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() / last.value.parse::<i128>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_mul_div - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() / last.value.parse::<f64>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_mul_div - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        panic!("invalid");
                    },
                    MathTokenType::Boolean => {
                        panic!("invalid");
                    },

                    _ => {panic!("wong")}
                }

            }
        } else if part.token_type == MathTokenType::Modulos {
            round_times_mul_div = 0;
            if index_for_mul_div == 0 || index_for_mul_div == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_mul_div - 1) as usize);
            tokens_parsing_now.remove((index_for_mul_div - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_mul_div - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x);
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() % last.value.parse::<i128>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_mul_div - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() % last.value.parse::<f64>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_mul_div - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        panic!("invalid");
                    },
                    MathTokenType::Boolean => {
                        panic!("invalid");
                    },

                    _ => {panic!("wong")}
                }

            }
        }
    }

    let mut index_for_add_sub = -1;
    let mut round_times_add_sub = 0;
    while tokens_parsing_now.len() != 1 {
        index_for_add_sub += 1;
        if round_times_add_sub > 1 {
            break
        }
        else if index_for_add_sub >= tokens_parsing_now.len() as i32 {
            index_for_add_sub = 0;
            round_times_add_sub += 1;
        }
        let part = tokens_parsing_now.get(index_for_add_sub as usize).unwrap().clone();
        if part.token_type == MathTokenType::Addition {
            round_times_add_sub = 0;
            if index_for_add_sub == 0 || index_for_add_sub == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_add_sub - 1) as usize);
            tokens_parsing_now.remove((index_for_add_sub - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_add_sub - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x);
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() + last.value.parse::<i128>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_add_sub - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() + last.value.parse::<f64>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_add_sub - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        last.value = (first.value + &*last.value.clone()).to_string();
                        tokens_parsing_now.insert((index_for_add_sub - 1) as usize, last);
                    },
                    MathTokenType::Boolean => {
                        panic!("invalid");
                    },

                    _ => {panic!("wong")}
                }

            }
        }
        else if part.token_type == MathTokenType::Subtraction {
            round_times_add_sub = 0;
            if index_for_add_sub == 0 || index_for_add_sub == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_add_sub - 1) as usize);
            tokens_parsing_now.remove((index_for_add_sub - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_add_sub - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x)
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() - last.value.parse::<i128>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_add_sub - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() - last.value.parse::<f64>().unwrap()).to_string();
                        tokens_parsing_now.insert((index_for_add_sub - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        panic!("invalid");
                    },
                    MathTokenType::Boolean => {
                        panic!("invalid");
                    },

                    _ => {panic!("wong")}
                }

            }
        }
    }

    let mut index_for_comp = -1;
    let mut round_times_comp = 0;
    while tokens_parsing_now.len() != 1 {
        index_for_comp += 1;
        if round_times_comp > 1 {
            break
        }
        else if index_for_comp >= tokens_parsing_now.len() as i32 {
            index_for_comp = 0;
            round_times_comp += 1;
        }
        let part = tokens_parsing_now.get(index_for_comp as usize).unwrap().clone();
        /*
        GreaterThan
        LessThan
        LessThanOrEqualTo
        GreaterThanOrEqualTo
        EqualTo
        NotEqualTo
        */

        if part.token_type == MathTokenType::GreaterThan {
            round_times_comp = 0;
            if index_for_comp == 0 || index_for_comp == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_comp - 1) as usize);
            tokens_parsing_now.remove((index_for_comp - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_comp - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x);
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() > last.value.parse::<i128>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() > last.value.parse::<f64>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        panic!("invalid");
                    },
                    MathTokenType::Boolean => {
                        panic!("invalid");
                    },

                    _ => {panic!("wong")}
                }

            }
        }
        else if part.token_type == MathTokenType::LessThan {
            round_times_comp = 0;
            if index_for_comp == 0 || index_for_comp == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_comp - 1) as usize);
            tokens_parsing_now.remove((index_for_comp - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_comp - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x);
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() < last.value.parse::<i128>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() < last.value.parse::<f64>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        panic!("invalid");
                    },
                    MathTokenType::Boolean => {
                        panic!("invalid");
                    },

                    _ => {panic!("wong")}
                }

            }
        }
        else if part.token_type == MathTokenType::GreaterThanOrEqualTo {
            round_times_comp = 0;
            if index_for_comp == 0 || index_for_comp == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_comp - 1) as usize);
            tokens_parsing_now.remove((index_for_comp - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_comp - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x);
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() >= last.value.parse::<i128>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() >= last.value.parse::<f64>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        panic!("invalid");
                    },
                    MathTokenType::Boolean => {
                        panic!("invalid");
                    },

                    _ => {panic!("wong")}
                }

            }
        }
        else if part.token_type == MathTokenType::LessThanOrEqualTo {
            round_times_comp = 0;
            if index_for_comp == 0 || index_for_comp == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_comp - 1) as usize);
            tokens_parsing_now.remove((index_for_comp - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_comp - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x)
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() <= last.value.parse::<i128>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() <= last.value.parse::<f64>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        panic!("invalid");
                    },
                    MathTokenType::Boolean => {
                        panic!("invalid");
                    },

                    _ => {panic!("wong")}
                }

            }
        }
        else if part.token_type == MathTokenType::EqualTo {
            round_times_comp = 0;
            if index_for_comp == 0 || index_for_comp == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_comp - 1) as usize);
            tokens_parsing_now.remove((index_for_comp - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_comp - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x);
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() == last.value.parse::<i128>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() == last.value.parse::<f64>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        last.value = (first.value == &*last.value.clone()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_add_sub - 1) as usize, last);
                    },
                    MathTokenType::Boolean => {
                        last.value = (first.value == &*last.value.clone()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_add_sub - 1) as usize, last);
                    },

                    _ => {panic!("wong")}
                }

            }
        }
        else if part.token_type == MathTokenType::NotEqualTo {
            round_times_comp = 0;
            if index_for_comp == 0 || index_for_comp == tokens_parsing_now.len() as i32 {
                panic!("ASD!")
            }
            let first = tokens_parsing_now.remove((index_for_comp - 1) as usize);
            tokens_parsing_now.remove((index_for_comp - 1) as usize);
            let mut last = tokens_parsing_now.remove((index_for_comp - 1) as usize);

            if first.token_type != last.token_type {
                panic!("Expected '{:?}' got '{:?}' at line {} char {}", first.token_type, last.token_type, last.y, last.x);
            } else {
                match first.token_type.clone() {
                    MathTokenType::Integer => {
                        last.value = (first.value.parse::<i128>().unwrap() != last.value.parse::<i128>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::FloatingPoint => {
                        last.value = (first.value.parse::<f64>().unwrap() != last.value.parse::<f64>().unwrap()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_comp - 1) as usize, last);
                    },
                    MathTokenType::String => {
                        last.value = (first.value != &*last.value.clone()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_add_sub - 1) as usize, last);
                    },
                    MathTokenType::Boolean => {
                        last.value = (first.value != &*last.value.clone()).to_string();
                        last.token_type = MathTokenType::Boolean;
                        tokens_parsing_now.insert((index_for_add_sub - 1) as usize, last);
                    },

                    _ => {panic!("wong")}
                }

            }
        }
    }
    if tokens_parsing_now.len() > 1{
        panic!("Eval Error")
    }
    tokens_parsing_now[0 as usize].clone()
}

fn single_test(expression: String, expected: String){
    let evaluation = eval_string(expression.clone());
    if expected == evaluation.true_value() {
        println!("Test Passed: {} == {}", expression, expected)
    } else {
        println!("Test Failed: {}", expression);
        println!("  -Expected: {}", expected);
        println!("    -Result: {}", evaluation.true_value());
    }
}

pub fn tests(){
    single_test("123".to_string(), "123".to_string());
    single_test("1 + 1".to_string(), "2".to_string());
    single_test("43 * 0 + 43".to_string(), "43".to_string());
    single_test("1/1".to_string(), "1".to_string());
    single_test("2.0 + 2.3".to_string(), "4.3".to_string());
    single_test("123 == 123".to_string(), "true".to_string());
    single_test("123 > 120".to_string(), "true".to_string());
    single_test("123 < 120".to_string(), "false".to_string());
    single_test("123 != 120".to_string(), "true".to_string());
    single_test("123 >= 120".to_string(), "true".to_string());
    single_test("123 <= 120".to_string(), "false".to_string());
}

