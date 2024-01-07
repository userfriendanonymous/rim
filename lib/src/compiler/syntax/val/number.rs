
#[derive(Clone, Debug)]
pub enum Value {
    Value(super::super::Number),
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    IsGreater,
    IsEqual,
}
