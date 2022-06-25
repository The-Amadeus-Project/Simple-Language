
#[derive(Debug, Clone, PartialEq)]
pub enum VarTypes {
    Int,
    Str,
    Bool,
    Float,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i128),
    String(String),
    Float(f64),
    Addition(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
    Subtraction(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Var {
    name: String,
    var_type: VarTypes,
    value: Expr
}

impl Var {
    pub fn new(name: String, var_type: VarTypes, value: Expr) -> Self {
        Self {
            name,
            var_type,
            value
        }
    }
}

#[derive(Debug, Clone)]
pub enum SL {
    Var(Var),
}