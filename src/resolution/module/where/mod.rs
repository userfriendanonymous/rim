use super::super::Globe;
use crate::resolution::globe::{ValId, ModuleId, Store};
use crate::syntax::{Ident, Path};
pub use builder::Value as Builder;
pub use public::{Value as Public, MergeCollision};
pub use structure::Value as Structure;

mod public;
pub mod structure;
mod builder;

#[derive(Clone, Debug, Default)]
pub struct LetIn {
    pub input: Value,
    pub output: Value,
}

#[derive(Clone, Debug, Default)]
pub struct Value {
    structure: Structure,
    public: Public,
}

impl Value {
    pub fn build(store: &mut Store) -> Builder {
        Builder::new(store)
    }

    pub fn structure(&self) -> &Structure {
        &self.structure
    }

    pub fn module_id(&self, name: &Ident) -> Option<&ModuleId> {
        self.public.module_id(name)
    }

    pub fn val_id(&self, name: &Ident) -> Option<&ValId> {
        self.public.val_id(name)
    }

    pub fn merge_let_in(&mut self, value: LetIn) -> Result<(), MergeCollision> {
        self.public.merge(value.output.public.clone())?;
        self.structure.add_let_in(value.into());
        Ok(())
    }

    pub fn shadow_let_in(&mut self, value: LetIn) {
        self.public.shadow(value.output.public.clone());
        self.structure.add_let_in(value.into());
    }

    pub fn merge(&mut self, other: Self) -> Result<(), MergeCollision> {
        self.public.merge(other.public)?;
        self.structure.shadow(other.structure);
        Ok(())
    }

    pub fn merge_val(&mut self, name: Ident, id: ValId) -> Result<(), ValId> {
        self.public.merge_val(name.clone(), id)?;
        self.structure.shadow_val(name, id);
        Ok(())
    }

    pub fn merge_module(&mut self, name: Ident, id: ModuleId) -> Result<(), ModuleId> {
        self.public.merge_module(name.clone(), id)?;
        self.structure.shadow_module(name, id);
        Ok(())
    }

    pub fn shadow(&mut self, other: Self) {
        self.public.shadow(other.public);
        self.structure.shadow(other.structure);
    }

    pub fn shadowed(mut self, other: Self) -> Self {
        self.shadow(other);
        self
    }

    pub fn shadow_val(&mut self, name: Ident, id: ValId) {
        self.public.shadow_val(name.clone(), id);
        self.structure.shadow_val(name, id);
    }

    pub fn shadow_module(&mut self, name: Ident, id: ModuleId) {
        self.public.shadow_module(name.clone(), id);
        self.structure.shadow_module(name, id);
    }

    pub fn to_path<'a>(&'a self, path_items: &[Ident], globe: &'a Globe) -> Result<&'a Value, usize> {
        let mut module = self;
        for (id, name) in path_items.into_iter().enumerate() {
            let mut value = globe.module(module.public.module_id(name).ok_or(id)?);
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
        module.module_id(&path.name).ok_or(None)
    }

    pub fn val_id_by_path<'a>(&'a self, path: &Path, globe: &'a Globe) -> Result<&'a ValId, Option<usize>> {
        let module = self.to_path(&path.items, globe).map_err(Some)?;
        module.val_id(&path.name).ok_or(None)
    }
}
