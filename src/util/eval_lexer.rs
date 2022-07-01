use std::collections::HashMap;
use std::ops::Deref;

// (1 + 1) ** 2 -> 4
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum MathTokenType {
    // types
    String,
    Integer,
    FloatingPoint,
    Boolean,

    Multiplication,
    Division,
    Addition,
    Subtraction,
    Modulos,
    Power,
    ParenthesisOpen,
    ParenthesisClose,

    GreaterThan,
    LessThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    EqualTo,
    NotEqualTo,


    NullForParser
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MathToken {
    pub token_type: MathTokenType,
    pub value: String,
    pub x: u32,
    pub y: u32,
}

impl MathToken {
    pub fn new(token_type: MathTokenType, value: String) -> Self {
        Self {
            token_type,
            value,
            x: 0,
            y: 0
        }
    }
    pub fn set_xy(&mut self, x: u32, y: u32){
        self.x = x;
        self.y = y;
    }
    pub fn is_string(&self) -> bool {
        self.token_type == MathTokenType::String
    }
    pub fn is_integer(&self) -> bool {
        self.token_type == MathTokenType::Integer
    }
    pub fn is_float(&self) -> bool {
        self.token_type == MathTokenType::FloatingPoint
    }
    pub fn is_bool(&self) -> bool {
        self.token_type == MathTokenType::Boolean
    }
    pub fn is_data_type(&self) -> bool {
        self.is_float() || self.is_bool() || self.is_string() || self.is_integer()
    }
    pub fn true_value(&self) -> String{
        if self.is_string(){
            format!("\"{}\"", self.value)
        } else {
            self.value.clone()
        }
    }
}

pub struct MathLexer {
    text_to_lex: Vec<char>,
    index: i32,
    run: bool,
    current_char: char,
    x: i32,
    y: i32,
    tok_start_x: i32,
    tok_start_y: i32,
    log: Vec<String>,
    error: Vec<String>,
    current_tokens: Vec<MathToken>
}

impl MathLexer {
    pub fn new() -> Self {
        Self {
            text_to_lex: vec![],
            index: -1,
            run: true,
            current_char: ' ',
            x: 0,
            y: 1,
            tok_start_x: 0,
            tok_start_y: 0,
            log: vec![],
            error: vec![],
            current_tokens: vec![]
        }
    }
    pub fn pos_starter(&mut self){
        self.tok_start_x = self.x;
        self.tok_start_y = self.y;
    }
    pub fn next_char(&mut self) -> bool {
        self.index += 1;
        if self.index >= self.text_to_lex.len() as i32{
            false

        } else {
            self.current_char = self.text_to_lex[self.index as usize];
            if self.current_char == '\n'{
                self.y += 1;
                self.x = 0;
            } else {
                self.x += 1;
            }
            true
        }
    }
    pub fn get_next_char_ignore_space(&self) -> Option<char> {
        let mut ind = self.index as usize;
        ind += 1;
        let mut character = self.text_to_lex[ind];
        while character == ' ' {
            ind += 1;
            character = self.text_to_lex[ind];
        }
        Some(character)
    }
    pub fn get_char(&self, ahead: i32) -> Option<char> {
        Some(self.text_to_lex[(self.index + ahead) as usize])
    }
     pub fn get_next_char(&self) -> Option<char> {
        self.get_char(1)
    }
    pub fn add_base(&mut self, tok_type: MathTokenType, value: String){
        let mut tok = MathToken::new(tok_type, value);
        tok.set_xy(self.tok_start_x as u32, self.tok_start_y as u32);
        self.current_tokens.push(tok);
    }
    pub fn add_special(&mut self, tok_type: MathTokenType){
        self.add_base(tok_type, "".to_string());
    }
    pub fn add_special_bare(&mut self, tok_type: MathTokenType, value: String){
        self.add_base(tok_type, value);
    }
    pub fn add_string(&mut self, value: String){
        self.add_base(MathTokenType::String, value);
    }
    pub fn add_integer(&mut self, value: String){
        self.add_base(MathTokenType::Integer, value);
    }
    pub fn add_float(&mut self, value: String){
        self.add_base(MathTokenType::FloatingPoint, value);
    }
    fn lex(&mut self) -> Vec<MathToken> {
        /*
        hierarchy

        comment
        string

        special
        identifier
        number

        */
        let num: Vec<char> = "0123456789".chars().collect();
        let allowed_for_id: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_1234567890".chars().collect();

        let mut unknown_length = "".to_string();
        let mut unknown_length_being_used= false;
        let mut str_on = false;
        let mut comment_on = false;
        let mut id_on = false;
        let mut num_on = false;

        let mut int = false;
        let mut float = false;


        while self.run {
            if !self.next_char(){
                self.run = false;
                break
            }

            if self.current_char == '"' && !comment_on {
                if str_on {
                    str_on = false;
                    unknown_length_being_used = false;
                    self.add_string(unknown_length.clone());
                    unknown_length = "".to_string();
                } else {
                    self.pos_starter();
                    str_on = true;

                    if unknown_length_being_used {
                        panic!("shit something went wrong! at line {} char {}", self.tok_start_y, self.tok_start_x);
                    }

                    unknown_length_being_used = true;
                }

            } else if str_on || comment_on {
                if self.current_char == '\n' && str_on{
                    panic!("unclosed string at line {} char {}", self.y, self.x);
                } else {
                    unknown_length += &self.current_char.to_string();
                }

            } else if num.contains(&self.current_char) && !id_on {
                if num_on {
                    unknown_length += &self.current_char.to_string();
                } else {
                    unknown_length += &self.current_char.to_string();
                    self.pos_starter();
                    num_on = true;
                    int = true;

                    if unknown_length_being_used {
                        panic!("shit something went wrong! at line {} char {}", self.tok_start_y, self.tok_start_x);
                    }

                    unknown_length_being_used = true;
                }
            } else if self.current_char == '.' && num_on && num.contains(&self.get_next_char_ignore_space().expect(&format!("expected char! at line {} char {}", self.y, self.tok_start_y))){
                int = false;
                float = true;
                unknown_length += ".";

            } else if allowed_for_id.contains(&self.current_char){

                // to avoid errors
                if num_on {
                    if int {
                        self.add_integer(unknown_length.clone());
                        int = false;
                    } else if float {
                        self.add_float(unknown_length.clone());
                        float = false;
                    } else {
                        panic!("what? how?! at line {} char {}", self.tok_start_y, self.tok_start_x);
                    }
                    num_on = false;
                    unknown_length_being_used = false;
                    unknown_length = "".to_string();
                }

                if id_on {
                    unknown_length += &self.current_char.to_string();
                } else {
                    unknown_length += &self.current_char.to_string();
                    self.pos_starter();
                    id_on = true;

                    if unknown_length_being_used {
                        panic!("shit something went wrong! at line {} char {}", self.tok_start_y, self.tok_start_x);
                    }

                    unknown_length_being_used = true;
                }
            } else {
                if num_on {
                    if int {
                        self.add_integer(unknown_length.clone());
                        int = false;
                    } else if float {
                        self.add_float(unknown_length.clone());
                        float = false;
                    } else {
                        panic!("what? how?! at line {} char {}", self.tok_start_y, self.tok_start_x);
                    }
                    num_on = false;
                    unknown_length_being_used = false;
                    unknown_length = "".to_string();
                } else if id_on {
                    match &*unknown_length {
                        "true" => self.add_special_bare(MathTokenType::Boolean, "true".to_string()),
                        "false" => self.add_special_bare(MathTokenType::Boolean, "false".to_string()),
                        _ => { panic!("Unexpected Identifier inside Math, at line {} char {}", self.tok_start_y, self.tok_start_x) }
                    }
                    id_on = false;
                    unknown_length_being_used = false;
                    unknown_length = "".to_string();
                }

                self.pos_starter();
                match self.current_char {
                    '=' =>
                        {
                            let next = self.get_next_char();
                            if !next.is_some() {
                                panic!("Expected Continuation at line {} char {}", self.tok_start_y, self.tok_start_x);
                            }
                            let next_char = next.unwrap();
                            if next_char == '=' {
                                self.add_special(MathTokenType::EqualTo);
                                self.next_char();
                            } else {
                               panic!("Unexpected '{}',  at line {} char {}", self.current_char, self.tok_start_y, self.tok_start_x);
                            }
                        },
                    '!' =>
                        {
                            let next = self.get_next_char();
                            if !next.is_some() {
                                panic!("Expected Continuation at line {} char {}", self.tok_start_y, self.tok_start_x);
                            }
                            let next_char = next.unwrap();
                            if next_char == '=' {
                                self.add_special(MathTokenType::NotEqualTo);
                                self.next_char();
                            } else {
                                panic!("Unexpected '{}',  at line {} char {}", self.current_char, self.tok_start_y, self.tok_start_x);
                            }
                        },
                    '%' => self.add_special(MathTokenType::Modulos),
                    '+' => self.add_special(MathTokenType::Addition),
                    '-' => self.add_special(MathTokenType::Subtraction),
                    '/' => self.add_special(MathTokenType::Division),
                    '*' => self.add_special(MathTokenType::Multiplication),
                    '(' => self.add_special(MathTokenType::ParenthesisOpen),
                    ')' => self.add_special(MathTokenType::ParenthesisClose),
                    '>' =>
                        {
                            let next = self.get_next_char();
                            if !next.is_some() {
                                panic!("Expected Continuation at line {} char {}", self.tok_start_y, self.tok_start_x);
                            }
                            let next_char = next.unwrap();
                            if next_char == '=' {
                                self.add_special(MathTokenType::GreaterThan);
                                self.next_char();
                            } else {
                                self.add_special(MathTokenType::GreaterThan);
                            }
                        },
                    '<' =>
                        {
                            let next = self.get_next_char();
                            if !next.is_some() {
                                panic!("Expected Continuation at line {} char {}", self.tok_start_y, self.tok_start_x);
                            }
                            let next_char = next.unwrap();
                            if next_char == '=' {
                                self.add_special(MathTokenType::LessThanOrEqualTo);
                                self.next_char();
                            } else {
                                self.add_special(MathTokenType::LessThan)
                            }
                        },
                    ' ' => {},
                    '\n' => {},
                    '\t' => {},
                    _ => {unimplemented!("not added -> {} <-, at line {} char {}", self.current_char, self.tok_start_y, self.tok_start_x)}
                }
            }
        }
        if num_on {
            if int {
                self.add_integer(unknown_length.clone());
            } else if float {
                self.add_float(unknown_length.clone());
            } else {
                panic!("what? how?")
            }
        } else if id_on {
            match &*unknown_length {
                "true" => self.add_special_bare(MathTokenType::Boolean, "true".to_string()),
                "false" => self.add_special_bare(MathTokenType::Boolean, "false".to_string()),
                _ => { panic!("Unexpected Identifier inside Math, at line {} char {}", self.tok_start_y, self.tok_start_x) }
            }
            id_on = false;
            unknown_length_being_used = false;
            unknown_length = "".to_string();
        } else if str_on {
            panic!("unclosed string at line {} char {}", self.tok_start_y, self.tok_start_x);
        }
        self.current_tokens.clone()
    }

    pub fn lex_text(&mut self, text: String) -> Vec<MathToken> {
        self.text_to_lex = text.chars().collect();
        self.lex()
    }
}