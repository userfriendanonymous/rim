
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value(u64);

impl Value {
    pub fn succ(&self) -> Self {
        Self(self.0 + 1)
    }

    pub fn unwrap(&self) -> u64 {
        self.0
    }
}
