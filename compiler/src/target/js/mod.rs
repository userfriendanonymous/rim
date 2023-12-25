use crate::resolution::{Env, Globe, globe::ValId};
use serde::{Serialize, Deserialize};
pub mod strict;
pub mod lazy;

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Environment {
    Browser,
    Node,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Evaluation {
    Lazy,
    Strict,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Type {
    pub environment: Environment,
    pub evaluation: Evaluation,
}

impl Type {
    pub fn compile(self, env: &Env, globe: &Globe, val_id: ValId) -> String {
        match self.evaluation {
            Evaluation::Lazy => lazy::value(env, globe, val_id),
            Evaluation::Strict => strict::value(env, globe, val_id)
        }
    }
}