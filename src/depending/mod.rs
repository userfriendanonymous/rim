use std::collections::BTreeMap;
use crate::resolution::module::r#where::LetIn;
use crate::syntax::{Ident, Value as Syntax};
use crate::resolution::{self, Env, built_in_module, Globe, Module};

pub enum Dependency {
    BuiltIn,
}

impl Dependency {
    pub fn create(&self, globe: &mut Globe) -> Env {
        match self {
            Self::BuiltIn => built_in_module::create(globe)
        }
    }
}

type Dependencies = BTreeMap<Ident, Dependency>;

pub fn resolve(syntax: &Syntax, deps: Dependencies) -> Result<(Env, Globe), resolution::module::Error<'_>> {
    let mut globe = Globe::new();

    let mut env = Env::default();
    for (name, value) in deps.iter() {
        let module = value.create(&mut globe);
        let id = globe.new_module(Module::Where(module));
        env.shadow_module(name.clone(), id);
    }

    let output = resolution::value(syntax, env.clone(), &mut globe)?;

    // let mut input = Env::default();
    // for (name, value) in deps.iter() {
    //     let module = value.create(&mut globe);
    //     let id = globe.new_module(Module::Where(module));
    //     input.shadow_module(name.clone(), id);
    // }

    let env = globe.build_module()
        .with_let_in(LetIn { input: env, output })
        .end();

    Ok((env, globe))
}
