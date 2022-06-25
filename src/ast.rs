
#[derive(Debug, Clone)]
pub enum VarTypes {
    Int,
    Str,
    Bool,
    Float,
}

enum Expr {
    Number(i128),
    Add(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Division(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Var {
    name: String,
    var_type: VarTypes,
    value: Vec<String>
}

impl Var {
    pub fn new(name: String, var_type: VarTypes, value: Vec<String>) -> Self {
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