use crate::resolution::globe::TypeId;

#[derive(Clone, Debug)]
pub enum Value {
    Init(usize, TypeId),
    Match(TypeId),
}
