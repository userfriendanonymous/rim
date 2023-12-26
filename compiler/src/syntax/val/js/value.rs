
#[derive(Clone, Debug)]
pub enum Value {
    String,
    Undefined,
    Null,
    NaN,
    Field,
    Index,
    Typeof,
    Eq,
}