use std::{fmt::Display, str::FromStr, collections::BTreeMap};
use chumsky::{Parser, primitive::just, error::Simple};
use serde::{Serialize, Deserialize};
use crate::{Ident, PackageId as Id};
use super::Dependency;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MetaError {
    Internal
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CodeError {
    Internal
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AddError {
    Internal,
    PathExists
}

#[derive(Clone, Debug)]
pub struct Path {
    path: super::Path,
    name: Ident,
    version: u32,
}

impl Path {
    pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        super::Path::parser()
            .then(Ident::parser())
            .then_ignore(just('@'))
            .then(chumsky::text::int(10))
            .map(|((path, name), version)| Self { path, name, version: version.parse().unwrap() })
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}{}@{}", self.path, self.name, self.version))
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

#[derive(Serialize, Deserialize)]
pub struct AddMeta {
    pub dependencies: BTreeMap<Ident, Dependency>
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub dependencies: BTreeMap<Ident, Dependency>,
    pub id: Id,
}
