use std::collections::BTreeMap;
use super::super::syntax::Ident;
use crate::compiler::{target, resolution::globe::{ModuleId, ValId}};

#[derive(Clone, Debug)]
pub enum MergeCollision {
    Module(Ident, ModuleId, ModuleId),
    Val(Ident, ValId, ValId),
}

#[derive(Clone, Debug, Default)]
pub struct Value {
    modules: BTreeMap<Ident, ModuleId>,
    vals: BTreeMap<Ident, ValId>,
    targets: BTreeMap<target::Type, BTreeMap<Ident, ValId>>
}

impl Value {
    pub fn shadow(&mut self, mut other: Self) {
        self.modules.append(&mut other.modules);
        self.vals.append(&mut other.vals);
    }

    pub fn shadow_val(&mut self, name: Ident, id: ValId) {
        self.vals.insert(name, id);
    }

    pub fn shadow_module(&mut self, name: Ident, id: ModuleId) {
        self.modules.insert(name, id);
    }

    pub fn merge_val(&mut self, name: Ident, id: ValId) -> Result<(), ValId> {
        if let Some(id) = self.vals.get(&name) {
            Err(*id)
        } else {
            self.shadow_val(name, id);
            Ok(())
        }
    }

    pub fn merge_module(&mut self, name: Ident, id: ModuleId) -> Result<(), ModuleId> {
        if let Some(id) = self.modules.get(&name) {
            Err(*id)
        } else {
            self.shadow_module(name, id);
            Ok(())
        }
    }

    pub fn merge_target(&mut self, r#type: target::Type, name: Ident, id: ValId) -> Result<(), ValId> {
        let targets = self.targets.entry(r#type)
            .or_insert(Default::default());
        match targets.get(&name) {
            Some(id) => Err(id.clone()),
            None => {
                targets.insert(name, id);
                Ok(())
            }
        }
    }

    pub fn merge(&mut self, other: Self) -> Result<(), MergeCollision> {
        for (name, id) in other.vals {
            self.merge_val(name.clone(), id).map_err(|self_id| MergeCollision::Val(name, self_id, id))?;
        }

        for (name, id) in other.modules {
            self.merge_module(name.clone(), id).map_err(|self_id| MergeCollision::Module(name, self_id, id))?;
        }

        Ok(())
    }
    
    pub fn module_id(&self, name: &Ident) -> Option<&ModuleId> {
        self.modules.get(name)
    }

    pub fn val_id(&self, name: &Ident) -> Option<&ValId> {
        self.vals.get(name)
    }
}
