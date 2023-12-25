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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Targets {
    pub js: JsTargets,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct JsTargets {
    pub node: BTreeMap<Ident, (syntax::Path, target::js::Evaluation)>,
    pub browser: BTreeMap<Ident, (syntax::Path, target::js::Evaluation)>,
}