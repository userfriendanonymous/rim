use crate::resolution::{globe::ValId, module};
pub use crate::syntax::val::{Number, Boolean, String, Js, Array};
pub use sum::Value as Sum;
pub use product::Value as Product;
pub use r#enum::Value as Enum;
pub use crate::syntax::val::{js, number, string};

pub mod sum;
pub mod product;
pub mod r#enum;

#[derive(Clone, Debug)]
pub enum Value {
    Ref(ValId),
    Apply(Box<Value>, Box<Value>),
    LetIn(module::Where, Box<Value>),
    Function(ValId, Box<Value>),
    Sum(Sum),
    Product(Product),
    Enum(Enum),
    String(String),
    Boolean(Boolean),
    Number(Number),
    Array(Array),
    Js(Js)
}

impl Value {
    pub fn apply(f: Value, i: Value) -> Self {
        Self::Apply(Box::new(f), Box::new(i))
    }
}