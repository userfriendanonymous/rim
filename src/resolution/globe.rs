use std::collections::BTreeMap;
use super::{Id, Val, Module, Type, val, module};
use crate::syntax::ident::from_str as ident;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValId(Id);

impl ValId {
    pub fn succ(&self) -> Self { Self(self.0.succ()) }
    pub fn unwrap(&self) -> Id { self.0 }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleId(Id);

impl ModuleId {
    pub fn succ(&self) -> Self { Self(self.0.succ()) }
    pub fn unwrap(&self) -> Id { self.0 }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeId(Id);

impl TypeId {
    pub fn succ(&self) -> Self { Self(self.0.succ()) }
    pub fn unwrap(&self) -> Id { self.0 }
}

#[derive(Clone, Debug, Default)]
pub struct Store {
    modules: BTreeMap<ModuleId, Module>,
    types: BTreeMap<TypeId, Type>,
    vals: BTreeMap<ValId, Val>,
    val_id: ValId,
    module_id: ModuleId,
    type_id: TypeId,
}

impl Store {
    pub fn val(&self, id: &ValId) -> &Val {
        self.vals.get(id).unwrap()
    }

    pub fn val_out(&self, id: &ValId) -> &val::Out {
        if let Val::Out(v) = self.val(id) {
            v
        } else {
            panic!("Out val expected");
        }
    }

    pub fn module(&self, id: &ModuleId) -> &Module {
        self.modules.get(id).unwrap()
    }

    pub fn module_where(&self, id: &ModuleId) -> &module::Where {
        let mut value = self.module(id);
        loop {
            match value {
                Module::Ref(id) => value = self.module(id),
                Module::Where(value) => break value
            }
        }
    }

    pub fn r#type(&self, id: &TypeId) -> &Type {
        self.types.get(id).unwrap()
    }

    pub fn sum_type(&self, id: &TypeId) -> &usize {
        if let Type::Sum(v) = self.r#type(id) {
            v
        } else {
            panic!("This type is not a sum!")
        }
    }

    pub fn product_type(&self, id: &TypeId) -> &usize {
        if let Type::Product(v) = self.r#type(id) {
            v
        } else {
            panic!("This type is not a product!")
        }
    }

    pub fn enum_type(&self, id: &TypeId) -> &usize {
        if let Type::Enum(v) = self.r#type(id) {
            v
        } else {
            panic!("This type is not a enum!")
        }
    }

    pub fn new_val(&mut self, value: Val) -> ValId {
        let id = self.val_id;
        self.vals.insert(id, value);
        self.val_id = self.val_id.succ();
        id
    }

    pub fn new_module(&mut self, value: Module) -> ModuleId {
        let id = self.module_id;
        self.modules.insert(id, value);
        self.module_id = self.module_id.succ();
        id
    }

    pub fn new_type(&mut self, value: Type) -> TypeId {
        let id = self.type_id;
        self.types.insert(id, value);
        self.type_id = self.type_id.succ();
        id
    }
}




#[derive(Clone, Debug)]
pub struct BuiltIns {
    pub string_type_id: TypeId,
    pub number_type_id: TypeId,
}

#[derive(Clone, Debug)]
pub struct Value {
    store: Store,
    main_val_id: Option<ValId>,
    built_ins: BuiltIns
}

impl Value {
    pub fn new() -> Self {
        let mut store = Store::default();

        let string_type_id = store.new_type(Type::String);
        let number_type_id = store.new_type(Type::Number);

        Self {
            main_val_id: None,
            built_ins: BuiltIns {
                number_type_id,
                string_type_id,
            },
            store,
        }
    }

    pub fn store_mut(&mut self) -> &mut Store {
        &mut self.store
    }

    pub fn build_module(&mut self) -> module::r#where::Builder {
        module::r#where::Builder::new(&mut self.store)
    }

    pub fn built_ins(&self) -> &BuiltIns {
        &self.built_ins
    }

    pub fn val(&self, id: &ValId) -> &Val {
        self.store.val(id)
    }

    pub fn val_out(&self, id: &ValId) -> &val::Out {
        self.store.val_out(id)
    }

    pub fn module(&self, id: &ModuleId) -> &Module {
        self.store.module(id)
    }

    pub fn module_where(&self, id: &ModuleId) -> &module::Where {
        self.store.module_where(id)
    }

    pub fn r#type(&self, id: &TypeId) -> &Type {
        self.store.r#type(id)
    }

    pub fn sum_type(&self, id: &TypeId) -> &usize {
        self.store.sum_type(id)
    }

    pub fn product_type(&self, id: &TypeId) -> &usize {
        self.store.product_type(id)
    }

    pub fn enum_type(&self, id: &TypeId) -> &usize {
        self.store.enum_type(id)
    }

    pub fn new_val(&mut self, value: Val) -> ValId {
        self.store.new_val(value)
    }

    pub fn new_module(&mut self, value: Module) -> ModuleId {
        self.store.new_module(value)
    }

    pub fn new_type(&mut self, value: Type) -> TypeId {
        self.store.new_type(value)
    }

    pub fn set_main_val_id(&mut self, id: ValId) {
        self.main_val_id = Some(id);
    }

    pub fn main_val_id(&self) -> Option<ValId> {
        self.main_val_id
    }
}

