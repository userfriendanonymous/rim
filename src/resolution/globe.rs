use std::collections::BTreeMap;
use super::{Id, Val, Module, Type, val, module};

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

#[derive(Default)]
pub struct Value {
    modules: BTreeMap<ModuleId, Module>,
    types: BTreeMap<TypeId, Type>,
    vals: BTreeMap<ValId, Val>,
    val_id: ValId,
    module_id: ModuleId,
    type_id: TypeId,
    main_val_id: Option<ValId>,
}

impl Value {
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

    pub fn set_main_val_id(&mut self, id: ValId) {
        self.main_val_id = Some(id);
    }

    pub fn main_val_id(&self) -> Option<ValId> {
        self.main_val_id
    }
}

