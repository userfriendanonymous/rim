use std::collections::BTreeMap;
use crate::{syntax::Ident, resolution::globe::{ModuleId, ValId}};

#[derive(Clone, Debug, Default)]
pub struct LetIn {
    pub input: Value,
    pub output: Value,
}

impl From<super::LetIn> for LetIn {
    fn from(value: super::LetIn) -> Self {
        Self {
            input: value.input.structure,
            output: value.output.structure
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Value {
    let_ins: Vec<LetIn>,
    modules: BTreeMap<Ident, ModuleId>,
    vals: BTreeMap<Ident, ValId>,
}

impl Value {
    pub fn shadow(&mut self, mut other: Self) {
        self.vals.append(&mut other.vals);
        self.modules.append(&mut other.modules);
        self.let_ins.append(&mut other.let_ins);
    }

    pub fn shadow_val(&mut self, name: Ident, id: ValId) {
        self.vals.insert(name, id);
    }

    pub fn shadow_module(&mut self, name: Ident, id: ModuleId) {
        self.modules.insert(name, id);
    }

    pub fn add_let_in(&mut self, value: LetIn) {
        self.let_ins.push(value);
    }
}
