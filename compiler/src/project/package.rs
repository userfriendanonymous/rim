use std::collections::BTreeMap;
use shared::Ident;
use crate::syntax::Value as Syntax;
use shared::library::store::Dependency;

pub struct Value {
    pub dependencies: Vec<Dependency>,
    pub syntax: Syntax
}
