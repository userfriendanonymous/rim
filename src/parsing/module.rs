use chumsky::{Parser, prelude::Simple, text::keyword, primitive::{just, any, empty}, Error};
use super::{function, ident, space, space::IndentBound, path};
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

pub fn val(ind: IndentBound) -> impl Parser<char, Vec<Item>, Error = Simple<char>> {
    space(ind + 1)
        .then_with(|ind| {
            let ind: IndentBound = ind.into();
            ident()
                .then_ignore(space(ind + 1))
                .then(function::value(ind + 1).boxed())
                .map(|(name, value)| Item::Val(name, value))
        })
        .repeated()
}

pub fn value(ind: IndentBound) -> impl Parser<char, Vec<Item>, Error = Simple<char>> {
    let module = |ind: IndentBound|
        space(ind + 1)
            .then_with(|ind| {
                let ind = ind.into();
                ident()
                    .then_ignore(space(ind))
                    .then(
                        keyword("where").to(ModuleType::Where)
                        .or(keyword("file").to(ModuleType::File))
                        .or(just('=').to(ModuleType::Ref))
                    )
                    .then_with(move |(name, r#type)| {
                        match r#type {
                            ModuleType::Ref => space(ind + 1)
                                .ignore_then(path())
                                .map(Module::Ref)
                                .boxed(),
                            ModuleType::File => space(ind + 1)
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

    let sum = |ind: IndentBound| space(ind + 1)
        .then_with(|ind| {
            let ind: IndentBound = ind.into();
            ident()
                .then(
                    space(ind + 1)
                        .ignore_then(ident())
                        .repeated()
                )
                .map(|(name, fields)| Item::Sum(name, fields))
        })
        .repeated();

    let product = |ind: IndentBound| space(ind + 1)
        .then_with(|ind| {
            let ind: IndentBound = ind.into();
            ident()
                .then(
                    space(ind + 1)
                        .ignore_then(ident())
                        .repeated()
                )
                .map(|(name, fields)| Item::Product(name, fields))
        })
        .repeated();

    let r#enum = |ind: IndentBound| space(ind + 1)
        .then_with(|ind| {
            let ind: IndentBound = ind.into();
            ident()
                .then(
                    space(ind + 1)
                        .ignore_then(ident())
                        .repeated()
                )
                .map(|(name, fields)| Item::Enum(name, fields))
        })
        .repeated();

    let let_in = |ind: IndentBound| value(ind + 1).boxed()
        .then_ignore(space(ind))
        .then_ignore(keyword("in"))
        .then(value(ind + 1).boxed())
        .map(|(input, output)| Item::LetIn(input, output));
    
    space(ind)
        .then_with(move |ind| {
            let ind = ind.into();
            keyword("val").to(ItemType::Val)
                .or(keyword("let").to(ItemType::LetIn))
                .or(keyword("mod").to(ItemType::Module))
                .or(keyword("sum").to(ItemType::Sum))
                .or(keyword("pro").to(ItemType::Product))
                .or(keyword("enum").to(ItemType::Enum))
                .then_with(move |r#type| {
                    match r#type {
                        ItemType::Val => val(ind).boxed(),
                        ItemType::Module => module(ind).boxed(),
                        ItemType::LetIn => let_in(ind).map(|item| vec![item]).boxed(),
                        ItemType::Sum => sum(ind).boxed(),
                        ItemType::Product => product(ind).boxed(),
                        ItemType::Enum => r#enum(ind).boxed(),
                    }
                })
                .map(Ok)
                .or_else(|error| Ok(Err(error)))
                .then_with(|result| match result {
                    Ok(v) => empty().to(Ok(v)).boxed(),
                    Err(e) => any().to(Err(e)).boxed()
                })
        })
        .repeated()
        .map(|items: Vec<Result<Vec<Item>, _>>| {
            items.into_iter().fold(Ok(Vec::<Item>::new()), |prev: Result<Vec<Item>, Simple<char>>, item| {
                match (prev, item) {
                    (Ok(mut prev), Ok(mut item)) => {
                        prev.append(&mut item);
                        Ok(prev)
                    },
                    (Err(prev_error), Err(item_error)) => {
                        Err(prev_error.merge(item_error))
                    },
                    (Err(prev_error), _) => Err(prev_error),
                    (_, Err(item_error)) => Err(item_error),
                }
            })
        })
        .try_map(|result, _| result)
}
