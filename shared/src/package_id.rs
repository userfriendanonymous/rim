use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Value(u32);

impl Value {
    pub fn zero() -> Self {
        Self(0)
    }

    pub fn inc(&self) -> Self {
        Self(self.0 + 1)
    }
}