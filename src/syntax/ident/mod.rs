pub use item::Value as Item;

pub mod item;

pub fn main() -> Value {
    Value { others: vec![Item::M, Item::A, Item::I], last: Item::N }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
    pub last: Item,
    pub others: Vec<Item>
}

impl Value {
    
}