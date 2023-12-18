use std::{str::FromStr, fmt::Display};
use serde::{Serialize, Deserialize};
use chumsky::{Parser, primitive::{just, end}, error::Simple};

pub type Item = super::Ident;

#[derive(Debug, Clone)]
pub struct Value {
    pub name: Item,
    pub items: Vec<Item>,
}

impl Value {
    pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        Item::parser()
            .then_ignore(just('.'))
            .repeated()
            .then(Item::parser())
            .map(|(items, name)| Value { items, name })
    }
}

impl From<Item> for Value {
    fn from(name: Item) -> Self {
        Self {
            name,
            items: vec![]
        }
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
        f.write_str(&(
            self.items.iter().map(|v| format!("{v}.")).collect::<String>()
            + &self.name.to_string()
        ))
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
        String::deserialize(deserializer).and_then(|s|
            s.parse().map_err(|_| serde::de::Error::custom("parsing error"))
        )
        
    }
}