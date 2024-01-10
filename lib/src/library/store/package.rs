use std::{fmt::Display, str::FromStr, collections::BTreeMap};
use chumsky::{Parser, error::Simple};
use clap::builder::Str;
use serde::{Serialize, Deserialize};
use crate::{Ident, PackageId as Id};
use super::{Dependency, family};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Version(u32);

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Version {
    pub fn init() -> Self { Self::default() }
    pub fn succ(&self) -> Self { Self(self.0 + 1) }
}

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

#[derive(Serialize, Deserialize)]
pub struct AddMeta {
    pub dependencies: BTreeMap<Ident, Dependency>
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub dependencies: BTreeMap<Ident, Dependency>,
    pub id: Id,
}
