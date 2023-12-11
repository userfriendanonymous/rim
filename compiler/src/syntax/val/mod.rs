use super::{Function, Path, module};
pub use infix::Value as Infix;
pub use number::Value as Number;
pub use string::Value as String;
pub use js::Value as Js;
pub use boolean::Value as Boolean;

pub mod infix;
pub mod number;
pub mod string;
pub mod js;
pub mod boolean;

#[derive(Clone, Debug)]
pub enum Value {
    Apply(Box<Value>, Box<Value>),
    InfixApply(Box<Value>, Option<Box<Value>>, Option<Box<Value>>),
    LetIn(Vec<module::Item>, Box<Value>),
    Function(Box<Function>),
    Ref(Path),
    String(String),
    Number(Number),
    Boolean(Boolean),
    InfixOp(InfixOp),
    If(Box<Value>, Box<Value>, Box<Value>),
    Js(Js)
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
