use super::{Ident, Val, Path};

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
