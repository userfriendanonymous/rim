
#[derive(Clone, Debug)]
pub enum Value {
    Value(crate::syntax::Number),
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
}
