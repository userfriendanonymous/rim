use chumsky::{Parser, primitive::end};
pub use item::Value as Item;
use crate::parsing::ident::value as parser;
pub mod item;

pub fn main() -> Value {
    Value { others: vec![Item::M, Item::A, Item::I], last: Item::N }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    pub last: Item,
    pub others: Vec<Item>
}

pub fn from_str(s: &str) -> Value {
    parser()
        .then_ignore(end())
        .parse(s)
        .unwrap()
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        from_str(value)
    }
}