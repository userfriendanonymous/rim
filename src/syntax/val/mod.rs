use super::{Function, Path, module, Number};
pub use infix::Value as Infix;

pub mod infix;

#[derive(Clone, Debug)]
pub enum Value {
    Apply(Box<Value>, Box<Value>),
    InfixApply(Box<Value>, Option<Box<Value>>, Option<Box<Value>>),
    LetIn(Vec<module::Item>, Box<Value>),
    Function(Box<Function>),
    Ref(Path),
    String(String),
    Number(Number),
    InfixOp(InfixOp)
}

#[derive(Clone, Debug)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    ApplyLeft, // i < f
    ApplyRight, // f > i
    Apply, // f $ i
}
