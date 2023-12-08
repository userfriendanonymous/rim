use crate::resolution::{globe::ValId, module};
pub use js::Value as Js;
pub use number::Value as Number;
pub use string::Value as String;
pub use sum::Value as Sum;
pub use product::Value as Product;
pub use boolean::Value as Boolean;

pub mod js;
pub mod number;
pub mod string;
pub mod sum;
pub mod product;
pub mod boolean;

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
