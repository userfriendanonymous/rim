use std::{fmt::Display, str::FromStr};

use chumsky::{Parser, error::Simple, primitive::just};
use serde::{Serialize, Deserialize};

use crate::Ident;

#[derive(Clone, Debug)]
pub struct Value {
    others: Vec<Ident>,
    last: Ident,
}

impl Value {
    pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        Ident::parser()
            .then_ignore(just('-'))
            .repeated()
            .then(Ident::parser())
            .map(|(others, last)| Self { others, last })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&(self.others.iter().map(|v| format!("{v}-")).collect::<String>() + &self.last.to_string()))
    }
}


impl FromStr for Value {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parser()
            .then_ignore(chumsky::primitive::end())
            .parse(s)
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
        String::deserialize(deserializer)?.parse().map_err(|_| serde::de::Error::custom("parsing failed"))
    }
}
