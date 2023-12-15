use crate::resolution::{Env, Globe, globe::ValId};


pub mod js;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Js,
}

impl Type {
    pub fn compile(self, env: &Env, globe: &Globe, val_id: ValId) -> String {
        match self {
            Self::Js => js::Type::Lazy.compile(env, globe, val_id)
        }
    }
}