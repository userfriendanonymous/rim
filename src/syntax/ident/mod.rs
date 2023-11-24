use std::fmt::Display;

use chumsky::{Parser, primitive::end};
pub use item::Value as Item;
use crate::parsing::ident::value as parser;
pub mod item;

macro_rules! value {
    ($literal:literal) => {
        Value::from_str($literal)
    };
}
pub(crate) use value;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    pub last: Item,
    pub others: Vec<Item>
}

impl Value {
    pub fn from_str(s: &str) -> Value {
        parser()
            .then_ignore(end())
            .parse(s)
            .unwrap()
    }

    pub fn to_string(&self) -> String {
        self.others.iter().map(|item| item.to_char()).collect::<String>() + &self.last.to_char().to_string()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string().fmt(f)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::from_str(value)
    }
}