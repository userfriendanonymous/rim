use crate::resolution::{Env, Globe, globe::ValId};


pub mod js;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    Js(js::Type),
}

impl Type {
    pub fn compile(self, env: &Env, globe: &Globe, val_id: ValId) -> String {
        match self {
            Self::Js(t) => t.compile(env, globe, val_id)
        }
    }
}