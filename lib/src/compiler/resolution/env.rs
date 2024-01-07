use std::collections::BTreeMap;
use crate::syntax::{Ident, Path};
use super::{Id, Module};

#[derive(Clone, Debug)]
pub struct Value<'a> {
    pub values: BTreeMap<&'a Ident, Id>,
    pub modules: BTreeMap<&'a Ident, (Id, Module)>
}

impl<'a> Value<'a> {
    pub fn value_by_path(&self, path: &Path) -> Result<&'a Id, Option<usize>> {
        if let Some((first, items)) = path.items.split_first() {
            let mut module = self.module(first).ok_or(Some(0))?;
            for (id, ident) in items.into_iter().enumerate() {
                module = self.module(ident).ok_or(Some(id + 1))?;
            }
            module.values.get(&path.name).ok_or(None)
        } else {
            self.value_id(&path.name).ok_or(None)
        }
    }

    pub fn module_by_path(&self, path: &Path) -> Result<&'a Id, Option<usize>> {
        if let Some((first, items)) = path.items.split_first() {
            let mut module = self.module(first).ok_or(Some(0))?;
            for (id, ident) in items.into_iter().enumerate() {
                module = self.module(ident).ok_or(Some(id + 1))?;
            }
            module.modules.get(&path.name).ok_or(None)
        } else {
            self.module_id(&path.name).ok_or(None)
        }
    }

    pub fn module(&self, name: &Ident) -> Option<&Module> {
        Some(&self.modules.get(name)?.1)
    }

    pub fn module_id(&self, name: &Ident) -> Option<&Id> {
        Some(&self.modules.get(name)?.0)
    }

    pub fn value_id(&self, name: &Ident) -> Option<&Id> {
        self.values.get(&name)
    }

    pub fn with_value(mut self, name: &'a Ident, id: Id) -> Self {
        self.values.insert(name, id);
        self
    }
}
