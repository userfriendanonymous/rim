use super::{Ident, Val};

#[derive(Clone, Debug)]
pub struct Value {
    pub input: Ident,
    pub output: Val
}
