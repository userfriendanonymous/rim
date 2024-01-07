use super::{Ident, Module, Item};
use crate::compiler::syntax::Val;

#[derive(Default)]
pub struct Value(super::Value);

impl From<super::Value> for Value {
    fn from(value: super::Value) -> Self {
        Self(value)
    }
}

impl Into<super::Value> for Value {
    fn into(self) -> super::Value {
        self.0
    }
}

impl Value {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_module<N: Into<Ident>>(mut self, name: N, value: Module) -> Self {
        self.0.push(Item::Module(name.into(), value));
        self
    }

    pub fn with_val<N: Into<Ident>>(mut self, name: N, value: Val) -> Self {
        self.0.push(Item::Val(name.into(), value));
        self
    }

    pub fn nest<N: Into<Ident>>(mut self, name: N, value: Value) -> Self {
        self.0.push(Item::Module(name.into(), Module::Where(value.into())));
        self
    }
}