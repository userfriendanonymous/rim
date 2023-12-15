use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};
use super::Dependency;
use shared::Ident;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub dependencies: BTreeMap<Ident, Dependency>,
    pub targets: Targets,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Targets {
    pub js: Vec<Ident>,
}
