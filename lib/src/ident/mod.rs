use std::{fmt::Display, str::FromStr, ops::Range};
use serde::{Serialize, Deserialize};
use chumsky::{Parser, primitive::{end, filter_map}, error::Simple};
pub use item::Value as Item;
pub mod item;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    pub last: Item,
    pub others: Vec<Item>
}

impl Value {
    pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        filter_map(|span: Range<usize>, c: char| Item::from_char(c).ok_or(Simple::custom(span, "invalid char")))
            .repeated()
            .try_map(|mut others, span| {
                let last = others.pop().ok_or(Simple::custom(span, "ident can't be empty"))?;
                Ok(Value { last, others })
            })
    }
}

impl FromStr for Value {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parser()
            .then_ignore(end())
            .parse(s)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&(self.others.iter().map(|item| item.to_char()).collect::<String>() + &self.last.to_char().to_string()))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::from_str(value).unwrap()
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        String::deserialize(deserializer).and_then(|s| {
            s.parse().map_err(|e| serde::de::Error::custom("parsing"))
        })
    }
}