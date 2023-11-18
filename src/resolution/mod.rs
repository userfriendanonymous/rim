pub use val::Value as Val;
pub use module::Value as Module;
pub use id::Value as Id;
pub use r#type::Value as Type;
pub use globe::Value as Globe;
pub use module::Where as Env;

use crate::syntax;

use self::module::LetIn;

pub mod val;
pub mod module;
pub mod globe;
pub mod id;
pub mod r#type;
// pub mod type_check;

pub fn value<'a>(input: &'a [syntax::module::Item]) -> Result<(Env, Globe), module::Error<'a>> {
    let mut globe = Globe::new();
    let builtin_env = module::Where::default()
        .with_module("builtin".into(), globe.built_ins().builtin_module_id);
    let main_module = module::r#where(input, builtin_env.clone(), &mut globe)?;

    let env = module::Where::default()
        .with_let_in(LetIn {
            input: builtin_env,
            output: main_module.let_ins().clone(),
        })
        .append(main_module);

    Ok((env, globe))
}
