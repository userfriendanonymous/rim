
use crate::resolution::globe::Store;
use crate::resolution::{Val, val, Module};
use crate::syntax::Ident;

use super::LetIn;

pub struct Value<'a> {
    store: &'a mut Store,
    inner: super::Value
}

impl<'a> From<Value<'a>> for super::Value {
    fn from(value: Value<'a>) -> Self {
        value.end()
    }
}

impl<'a> Value<'a> {
    pub fn new(store: &'a mut Store) -> Self {
        Self {
            store,
            inner: super::Value::default(),
        }
    }

    pub fn end(self) -> super::Value {
        self.inner
    }

    pub fn with_val<N: Into<Ident>>(mut self, name: N, value: val::Out) -> Self {
        self.inner.shadow_val(name.into(), self.store.new_val(Val::Out(value)));
        self
    }

    pub fn with_module<N: Into<Ident>, V: Into<super::Value>>(mut self, name: N, value: V) -> Self {
        self.inner.shadow_module(name.into(), self.store.new_module(Module::Where(value.into())));
        self
    }

    pub fn with_let_in(mut self, value: LetIn) -> Self {
        self.inner.shadow_let_in(value);
        self
    }

    pub fn nest_let_in(self, input: impl for<'v> FnOnce(Value<'v>) -> Value<'v>, output: impl for<'v> FnOnce(Value<'v>) -> Value<'v>) -> Self {
        let input = input(Value::new(self.store)).into();
        let output = output(Value::new(self.store)).into();
        self.with_let_in(LetIn { input, output })
    }

    pub fn nest_module<N: Into<Ident>>(self, name: N, f: impl for<'v> FnOnce(Value<'v>) -> Value<'v>) -> Self {
        let value: super::Value = f(Value::new(self.store)).into();
        self.with_module(name, value)
    }
}
