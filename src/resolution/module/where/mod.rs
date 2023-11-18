use std::collections::BTreeMap;
use super::super::Globe;
use crate::resolution::globe::{ValId, ModuleId, Store};
use crate::syntax::{Ident, Path, ident};
use super::LetIn;
pub use builder::Value as Builder;

mod builder;

#[derive(Clone, Debug)]
pub enum TryAppendError {
    Val(ValId),
    Module(ModuleId),
}


#[derive(Clone, Debug, Default)]
pub struct Value {
    let_ins: Vec<LetIn>,
    modules: BTreeMap<Ident, ModuleId>,
    vals: BTreeMap<Ident, ValId>
}

impl Value {
    pub fn build(store: &mut Store) -> Builder {
        Builder::new(store)
    }

    pub fn with_val(mut self, name: Ident, id: ValId) -> Self {
        self.add_val(name, id);
        self
    }

    pub fn with_module(mut self, name: Ident, id: ModuleId) -> Self {
        self.add_module(name, id);
        self
    }

    pub fn with_let_in(mut self, value: LetIn) -> Self {
        self.add_let_in(value);
        self
    }

    pub fn main_val(&self) -> Option<&ValId> {
        self.val(&ident::main())
    }

    pub fn modules(&self) -> &BTreeMap<Ident, ModuleId> {
        &self.modules
    }

    pub fn vals(&self) -> &BTreeMap<Ident, ValId> {
        &self.vals
    }

    pub fn let_ins(&self) -> &Vec<LetIn> {
        &self.let_ins
    }

    pub fn to_path<'a>(&'a self, path_items: &[Ident], globe: &'a Globe) -> Result<&'a Value, usize> {
        let mut module = self;
        for (id, name) in path_items.into_iter().enumerate() {
            let mut value = globe.module(module.module(name).ok_or(id)?);
            module = loop {
                match value {
                    super::Value::Ref(id) => value = globe.module(id),
                    super::Value::Where(value) => break value
                }
            };
        }
        Ok(module)
    }

    pub fn module_id_by_path<'a>(&'a self, path: &Path, globe: &'a Globe) -> Result<&'a ModuleId, Option<usize>> {
        let module = self.to_path(&path.items, globe).map_err(Some)?;
        module.modules.get(&path.name).ok_or(None)
    }

    pub fn value_id_by_path<'a>(&'a self, path: &Path, globe: &'a Globe) -> Result<&'a ValId, Option<usize>> {
        let module = self.to_path(&path.items, globe).map_err(Some)?;
        module.vals.get(&path.name).ok_or(None)
    }

    pub fn module(&self, name: &Ident) -> Option<&ModuleId> {
        self.modules.get(name)
    }

    pub fn val(&self, name: &Ident) -> Option<&ValId> {
        self.vals.get(name)
    }

    pub fn append(mut self, mut other: Self) -> Self {
        self.modules.append(&mut other.modules);
        self.vals.append(&mut other.vals);
        self
    }

    pub fn add_val(&mut self, name: Ident, id: ValId) {
        self.vals.insert(name, id);
    }

    pub fn add_module(&mut self, name: Ident, id: ModuleId) {
        self.modules.insert(name, id);
    }

    pub fn add_let_in(&mut self, value: LetIn) {
        self.let_ins.push(value);
    }

    pub fn try_add_val(&mut self, name: Ident, id: ValId) -> Result<(), ValId> {
        if let Some(id) = self.vals.get(&name) {
            Err(*id)
        } else {
            self.vals.insert(name, id);
            Ok(())
        }
    }

    pub fn try_add_module(&mut self, name: Ident, id: ModuleId) -> Result<(), ModuleId> {
        if let Some(id) = self.modules.get(&name) {
            Err(*id)
        } else {
            self.modules.insert(name, id);
            Ok(())
        }
    }

    pub fn try_append(&mut self, other: Self) -> Result<(), TryAppendError> {
        type E = TryAppendError;
        for (name, id) in other.vals {
            self.try_add_val(name, id).map_err(E::Val)?;
        }
        for (name, id) in other.modules {
            self.try_add_module(name, id).map_err(E::Module)?;
        }
        Ok(())
    }
}