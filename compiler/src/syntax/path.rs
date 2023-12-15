
pub type Item = super::Ident;

#[derive(Debug, Clone)]
pub struct Value {
    pub name: Item,
    pub items: Vec<Item>,
}

impl From<Item> for Value {
    fn from(name: Item) -> Self {
        Self {
            name,
            items: vec![]
        }
    }
}