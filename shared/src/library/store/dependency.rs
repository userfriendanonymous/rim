use serde::{Serialize, Deserialize};

use super::package;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    BuiltIn,
    Library(package::Path),
}
