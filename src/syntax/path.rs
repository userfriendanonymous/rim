
pub type Item = super::Ident;

#[derive(Debug, Clone)]
pub struct Value {
    pub name: Item,
    pub items: Vec<Item>,
}
