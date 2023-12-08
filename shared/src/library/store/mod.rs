
use serde::{Serialize, Deserialize};
pub use path::Value as Path;
pub use dependency::Value as Dependency;

pub mod directory;
pub mod package;
pub mod path;
pub mod dependency;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PackageMetaError {
    Internal
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PackageCodeError {
    Internal
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AddPackageError {
    Internal,
    PathExists
}