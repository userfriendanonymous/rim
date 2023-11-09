pub use item::Value as Item;

pub mod item;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    pub last: Item,
    pub others: Vec<Item>
}

impl Value {
    
}