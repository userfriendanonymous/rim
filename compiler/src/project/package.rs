use std::collections::BTreeMap;
use shared::Ident;
use crate::{syntax::Value as Syntax, depending::Dependency};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(u64);

pub struct Value {
    pub dependencies: Vec<Dependency>,
    pub syntax: Syntax
}
