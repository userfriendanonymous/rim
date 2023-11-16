use chumsky::{Parser, prelude::Simple, text::keyword, primitive::just};
use super::{function, ident, space, path};
use crate::syntax::module::{Module, Item};

#[derive(Clone, Debug)]
enum ItemType {
    Module,
    LetIn,
    Val,
    Sum,
    Product,
    Enum
}


#[derive(Clone, Debug)]
enum ModuleType {
    Where,
    File,
    Ref
}

pub fn value(ind: u16) -> impl Parser<char, Vec<Item>, Error = Simple<char>> {
    let val = |ind: u16| space(ind + 1)
        .then_with(|ind| {
            let ind = ind + 1;
            ident()
                .then_ignore(space(ind))
                .then(function::value(ind))
                .map(|(name, value)| Item::Val(name, value))
        })
        .repeated();

    let module = |ind: u16| space(ind + 1)
        .then_with(|ind| {
            let ind = ind + 1;
            ident()
                .then_ignore(space(ind))
                .then(
                    keyword("where").to(ModuleType::Where)
                    .or(keyword("file").to(ModuleType::File))
                    .or(just('=').to(ModuleType::Ref))
                )
                .then_with(move |(name, r#type)| {
                    match r#type {
                        ModuleType::Ref => space(ind)
                            .ignore_then(path(ind))
                            .map(Module::Ref)
                            .boxed(),
                        ModuleType::File => space(ind)
                            .ignore_then(ident())
                            .map(Module::File)
                            .boxed(),
                        ModuleType::Where => value(ind + 1)
                            .map(Module::Where)
                            .boxed()
                    }
                    .map(move |value| Item::Module(name.clone(), value))
                })
        })
        .repeated();

    let sum = |ind: u16| space(ind + 1)
        .then_with(|ind| {
            let ind = ind + 1;
            ident()
                .then(
                    space(ind)
                        .ignore_then(ident())
                        .repeated()
                )
                .map(|(name, fields)| Item::Sum(name, fields))
        })
        .repeated();

    let product = |ind: u16| space(ind + 1)
        .then_with(|ind| {
            let ind = ind + 1;
            ident()
                .then(
                    space(ind)
                        .ignore_then(ident())
                        .repeated()
                )
                .map(|(name, fields)| Item::Product(name, fields))
        })
        .repeated();

    let r#enum = |ind: u16| space(ind + 1)
        .then_with(|ind| {
            let ind = ind + 1;
            ident()
                .then(
                    space(ind)
                        .ignore_then(ident())
                        .repeated()
                )
                .map(|(name, fields)| Item::Enum(name, fields))
        })
        .repeated();

    let let_in = |ind: u16| value(ind + 1).boxed()
        .then_ignore(space(ind))
        .then_ignore(keyword("in"))
        .then(value(ind).boxed())
        .map(|(input, output)| Item::LetIn(input, output));
    
    space(ind)
        .then_with(move |ind| {
            keyword("val").to(ItemType::Val)
                .or(keyword("let").to(ItemType::LetIn))
                .or(keyword("mod").to(ItemType::Module))
                .or(keyword("sum").to(ItemType::Sum))
                .or(keyword("pro").to(ItemType::Product))
                .or(keyword("enum").to(ItemType::Enum))
                .then_with(move |r#type| {
                    // let ind = ind + 1;
                    match r#type {
                        ItemType::Val => val(ind).boxed(),
                        ItemType::Module => module(ind).boxed(),
                        ItemType::LetIn => let_in(ind).map(|item| vec![item]).boxed(),
                        ItemType::Sum => sum(ind).boxed(),
                        ItemType::Product => product(ind).boxed(),
                        ItemType::Enum => r#enum(ind).boxed(),
                    }
                })
        })
        .repeated()
        .map(|items| items.into_iter().flatten().collect())
}
