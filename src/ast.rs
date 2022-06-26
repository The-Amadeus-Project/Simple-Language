use crate::lexer::{Token, TokenType};
use crate::parser::VarTypes;

#[derive(Debug, Clone)]
pub struct Math {
    given: Vec<Token>,
    ind: i32,
    current: Token,
    terms: Vec<String>,
    expression: Vec<String>
}

impl Math {
    pub fn new() -> Self {
        Self {
            given: vec![],
            ind: -1,
            current: Token::new(TokenType::NullForParser, "".to_string()),
            terms: vec!["*".to_string(), "/".to_string()],
            expression: vec!["+".to_string(), "-".to_string()]
        }
    }
    fn next(&mut self) -> bool {
        self.ind += 1;
        if self.ind >= self.given.len() as i32 {
            false
        } else {
            self.current = self.given[self.ind as usize].clone();
            true
        }
    }
    fn factor(&mut self) -> Expr {
        if self.current.token_type == TokenType::String {
            Expr::String(self.current.value.clone())
        } else if self.current.token_type == TokenType::Boolean {
            if self.current.value == "true" {
                Expr::Bool(true)
            } else if self.current.value == "false" {
                Expr::Bool(false)
            } else {
                panic!("How? {:?}", self.current)
            }
        } else if self.current.token_type == TokenType::FloatingPoint {
            Expr::Float(self.current.value.parse::<f64>().unwrap())
        } else if self.current.token_type == TokenType::Integer {
            Expr::Integer(self.current.value.parse::<i128>().unwrap())
        } else {
            panic!("Invalid type {:?}", self.current)
        }
    }
    fn term(&mut self) -> Expr {
        let mut left = self.factor();
        self.next();

        while self.terms.contains(&self.current.value) {
            let operation = self.current.value.clone();
            self.next();
            let right = self.factor();
            let result = self.next();
            if operation == "*" {
                left = Expr::Multiply(Box::new(right), Box::new(left))
            } else {
                left = Expr::Division(Box::new(right), Box::new(left))
            }
            if !result {
                break
            }

        }
        left
    }
    fn expression(&mut self) -> Expr {
        let mut left = self.term();

        while self.expression.contains(&self.current.value) {
            let operation = self.current.value.clone();
            self.next();
            let right = self.term();
            let result = self.next();
            if operation == "+" {
                left = Expr::Addition(Box::new(right), Box::new(left))
            } else {
                left = Expr::Subtraction(Box::new(right), Box::new(left))
            }
            if !result {
                break
            }

        }
        left
    }
    pub fn parse(&mut self, given: Vec<Token>) -> Expr {
        self.given = given;
        self.next();
        self.expression()

    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Float(f64),
    Integer(i128),
    Bool(bool),

    Addition(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
}

