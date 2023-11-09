use super::{Function, Path, module};

#[derive(Clone, Debug)]
pub enum Value {
    Call(Box<Value>, Box<Value>),
    LetIn(Vec<module::Item>, Box<Value>),
    Function(Box<Function>),
    Ref(Path),
}
