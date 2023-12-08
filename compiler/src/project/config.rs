use serde::{Serialize, Deserialize};
use super::Dependency;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub dependencies: Vec<Dependency>
}