use std::collections::BTreeMap;
use crate::syntax::{Ident, Value as Syntax};
use crate::resolution::{self, Env, built_in_module, Globe, Module};

pub enum Dependency {
    BuiltIn,
}

impl Dependency {
    pub fn create(self, globe: &mut Globe) -> Env {
        match self {
            Self::BuiltIn => built_in_module::create(globe)
        }
    }
}

type Dependencies = BTreeMap<Ident, Dependency>;

pub fn resolve(syntax: &Syntax, deps: Dependencies) -> Result<(Env, Globe), resolution::module::Error<'_>> {
    let mut globe = Globe::new();
    let mut env = Env::default();

    for (name, value) in deps {
        let id = globe.new_module(Module::Where(value.create(&mut globe)));
        env.shadow_module(name, id);
    }

    Ok((resolution::value(syntax, env, &mut globe)?, globe))
}
