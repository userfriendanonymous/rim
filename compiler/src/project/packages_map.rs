use std::collections::BTreeMap;
use shared::{Ident, PackageId};
use crate::syntax::Value as Syntax;
use crate::syntax::module;

pub type Dependencies = BTreeMap<Ident, PackageId>;

fn module_where(name: impl Into<Ident>, items: Vec<module::Item>) -> module::Item {
    module::Item::Module(name.into(), module::Module::Where(items))
}

#[derive(Clone, Debug)]
pub struct Item {
    pub syntax: Syntax,
    pub dependencies: Dependencies,
}

impl Item {
    pub fn to_syntax(self) -> Syntax {
        vec![
            module::Item::LetIn(
                vec![module_where("env", self.dependencies.into_iter().map(|(name, id)| {
                    module::Item::Module(name, module::Module::Ref(id.to_ident().into()))
                }).collect())],
                vec![module::Item::From(
                    Into::<Ident>::into("env").into(),
                    self.syntax
                )]
            )
        ]
    }
}

#[derive(Default, Clone, Debug)]
pub struct Value(BTreeMap<PackageId, Item>);

impl Value {
    pub fn insert(&mut self, id: PackageId, item: Item) {
        self.0.insert(id, item);
    }

    pub fn append(&mut self, other: &mut Self) {
        self.0.append(&mut other.0)
    }

    pub fn to_syntax(self) -> Syntax {
        self.0.into_iter().rev().fold(
            Default::default(),
            |input, (id, item)| {
                vec![module::Item::LetIn(input, vec![module_where(id.to_ident(), item.to_syntax())])]
            }
        )
    }
}
