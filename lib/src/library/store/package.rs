use std::{fmt::Display, str::FromStr, collections::BTreeMap};
use chumsky::{Parser, error::Simple};
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Path(super::Path);

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Path {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::Path::from_str(s).map(Self)
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
