use serde::{Serialize, Deserialize};
use crate::Ident;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Value(u32);

impl Value {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn inc(&self) -> Self {
        Self(self.0 + 1)
    }

    pub fn to_ident(&self) -> Ident {
        self.0.to_string().as_str().into()
    }
}
