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
    InfixOp(InfixOp),
    If(Box<Value>, Box<Value>, Box<Value>),
}

#[derive(Clone, Debug)]
pub enum InfixOp {
    // Number
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    // Function
    ApplyLeft, // i < f
    ApplyRight, // f > i
    Apply, // f $ i
    // Boolean
    And,
    Or,
}
