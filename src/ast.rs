
#[derive(Debug, Clone, PartialEq)]
pub enum VarTypes {
    Int,
    Str,
    Bool,
    Float,
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