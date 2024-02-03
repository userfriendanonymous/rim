
use serde::{Serialize, Deserialize};
pub use path::Value as Path;
pub use dependency::Value as Dependency;

pub mod directory;
pub mod package;
pub mod path;
pub mod dependency;
pub mod family;