use super::{Function, Path, module, Number};

#[derive(Clone, Debug)]
pub enum Value {
    Apply(Box<Value>, Box<Value>),
    InfixApply(Box<Value>, Option<Box<Value>>, Option<Box<Value>>),
    LetIn(Vec<module::Item>, Box<Value>),
    Function(Box<Function>),
    Ref(Path),
    String(String),
    Number(Number),
    Infix(Infix)
}

#[derive(Clone, Debug)]
pub enum Infix {
    Add,
    Sub,
    Mul,
    Div,
    ApplyLeft, // i < f
    ApplyRight // f > i
}