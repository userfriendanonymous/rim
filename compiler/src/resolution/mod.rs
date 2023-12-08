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
pub mod built_in_module;
// pub mod type_check;

pub fn value<'a>(syntax: &'a syntax::Value, env: Env, globe: &mut Globe) -> Result<Env, module::Error<'a>> {
    module::r#where(syntax, env, globe)
}
