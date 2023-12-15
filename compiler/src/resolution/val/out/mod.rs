use crate::resolution::{globe::ValId, module};
pub use crate::syntax::val::{Number, Boolean, String, Js};
pub use sum::Value as Sum;
pub use product::Value as Product;
pub use crate::syntax::val::{js, number, string};

pub mod sum;
pub mod product;

#[derive(Clone, Debug)]
pub enum Value {
    Ref(ValId),
    Apply(Box<Value>, Box<Value>),
    LetIn(module::Where, Box<Value>),
    Function(ValId, Box<Value>),
    Sum(sum::Value),
    Product(Product),
    String(String),
    Boolean(Boolean),
    Number(Number),
    Js(Js)
}
