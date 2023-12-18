use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};
use crate::{syntax, target};

use super::Dependency;
use shared::Ident;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub dependencies: BTreeMap<Ident, Dependency>,
    pub targets: Targets,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Targets {
    pub js: BTreeMap<Ident, (syntax::Path, target::js::Type)>,
}
