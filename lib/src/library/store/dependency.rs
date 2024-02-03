use serde::{Serialize, Deserialize};

use super::{package, family};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    BuiltIn,
    Library(family::Path, package::Version),
}
