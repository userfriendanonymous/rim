use super::{Ident, Val, Path};
pub use builder::Value as Builder;

mod builder;

pub type Value = Vec<Item>;

#[derive(Clone, Debug)]
pub enum Item {
    LetIn(Vec<Item>, Vec<Item>),
    From(Path, Vec<Item>),
    Val(Ident, Val),
    Module(Ident, Module),
    Sum(Ident, Vec<Ident>),
    Product(Ident, Vec<Ident>),
    Enum(Ident, Vec<Ident>)
}

#[derive(Clone, Debug)]
pub enum Module {
    Where(Vec<Item>),
    Ref(Path),
    File(Ident),
}
