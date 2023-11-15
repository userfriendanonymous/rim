pub use val::Value as Val;
pub use module::Value as Module;
pub use id::Value as Id;
pub use r#type::Value as Type;
pub use globe::Value as Globe;
pub use module::Where as Env;

use crate::syntax;

pub mod val;
pub mod module;
pub mod globe;
pub mod id;
pub mod r#type;
// pub mod type_check;

pub fn value<'a>(input: &'a [syntax::module::Item]) -> Result<(Env, Globe), module::Error<'a>> {
    let mut globe = Globe::new();
    let env = module::r#where(input, module::Where::default(), &mut globe)?;
    Ok((env, globe))
}
