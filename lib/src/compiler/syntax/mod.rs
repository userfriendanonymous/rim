pub use ident::Value as Ident;
pub use val::Value as Val;
pub use function::Value as Function;
pub use path::Value as Path;
pub use number::Value as Number;

pub mod ident;
pub mod val;
pub mod function;
pub mod path;
pub mod module;
pub mod number;

pub type Value = Vec<module::Item>;