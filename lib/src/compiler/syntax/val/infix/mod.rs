use chumsky::Parser;
use nonempty::NonEmpty;
pub use item::Value as Item;
use super::super::super::parsing::val::infix as parser;

mod item;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value(pub NonEmpty<Item>);

pub fn from_str(value: &str) -> Value {
    parser().parse(value).unwrap()
}

macro_rules! value {
    ($literal:literal) => {
        $crate::compiler::syntax::val::infix::from_str($literal)
    };
}
pub(crate) use value;