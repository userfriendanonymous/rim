
use std::{collections::{BTreeMap, BTreeSet}, iter::zip};
use super::{Id, Val};
use base_type::Value as BaseType;
use poly_type::Value as PolyType;
use var::Id as VarId;
use bounds::Value as Bounds;

mod base_type;
mod poly_type;
mod var;
mod bounds;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValueId(Id);

pub struct Value {
    vals: BTreeMap<ValueId, (Val, ValType)>,
    type_vars: TypeVarId
}

impl Value {
    pub fn new_type_var(&mut self) -> TypeVarId {
        let id = self.type_vars;
        self.type_vars += 1;
        id
    }
}


pub enum ValType {
    // Input bounds
    Input(Vec<Type>),
    Output(Type)
}
