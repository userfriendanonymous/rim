use super::TypeId;

#[derive(Clone, Debug)]
pub enum Value {
    Init(usize, TypeId),
    Match(TypeId),
}
