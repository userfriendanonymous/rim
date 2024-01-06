use super::super::super::globe::TypeId;

#[derive(Clone, Debug)]
pub enum Value {
    Init(usize, TypeId),
    Match(TypeId),
}
