pub use item::Value as Item;

pub mod item;

#[derive(Debug, Clone)]
pub struct Value {
    pub items: Vec<item::Value>,
    pub last: item::Value
}
