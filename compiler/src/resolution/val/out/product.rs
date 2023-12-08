use crate::resolution::globe::TypeId;

#[derive(Clone, Debug)]
pub enum Value {
    Init(TypeId),
    Field(usize, TypeId)

}