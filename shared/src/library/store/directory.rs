use std::{fmt::Display, str::FromStr};
use chumsky::{Parser, error::Simple};
use serde::{Serialize, Deserialize};
use crate::Ident;

#[derive(Clone)]
pub struct Path {
    path: super::Path,
    name: Ident
}

impl Path {
    pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        super::Path::parser()
            .then(Ident::parser())
            .map(|(path, name)| Self { path, name })
    }
}

impl FromStr for Path {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parser()
            .then_ignore(chumsky::primitive::end())
            .parse(s)
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}{}", self.path, self.name))
    }
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        String::deserialize(deserializer)?.parse().map_err(|_| serde::de::Error::custom("parsing failed"))
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Meta {
    packages: Vec<Ident>,
    directories: Vec<Ident>,
}