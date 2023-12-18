use crate::resolution::{Env, Globe, globe::ValId};
use serde::{Serialize, Deserialize};
pub mod strict;
pub mod lazy;

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Lazy,
    Strict,
}

impl Type {
    pub fn compile(self, env: &Env, globe: &Globe, val_id: ValId) -> String {
        match self {
            Type::Lazy => lazy::value(env, globe, val_id),
            Type::Strict => strict::value(env, globe, val_id)
        }
    }
}