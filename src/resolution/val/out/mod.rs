use crate::{resolution::{globe::{ValId, TypeId}, module}, syntax};
pub use js::Value as Js;

pub mod js;

#[derive(Clone, Debug)]
pub enum Value {
    Ref(ValId),
    Call(Box<Value>, Box<Value>),
    LetIn(module::Where, Box<Value>),
    Function(ValId, Box<Value>),
    Sum(usize, TypeId),
    Product(TypeId),
    SumMatch(TypeId),
    ProductField(usize, TypeId),
    String(String),
    Number(syntax::Number),
    Js(Js)
}
