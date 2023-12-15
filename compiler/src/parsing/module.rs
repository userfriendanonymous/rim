use chumsky::{Parser, prelude::Simple, text::keyword, primitive::{just, any, empty}, Error};
use super::{function, ident, space, space::IndentBound, path};
use crate::{syntax::module::{Module, Item}, target};

#[derive(Clone, Debug)]
enum ItemType {
    Module,
    LetIn,
    Val,
    Sum,
    Product,
    Enum,
    From,
    // Target,
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

    let from = |ind: IndentBound| space(ind + 1)
        .then_with(|ind| {
            let ind: IndentBound = ind.into();
            path()
                .then(self::value(ind + 1).boxed())
                .map(|(path, items)| Item::From(path, items))
        })
        .repeated();

    // let target = |ind: IndentBound| space(ind + 1)
    //     .then_with(|ind| {
    //         let ind: IndentBound = ind.into();
    //         just("js").to(target::Type::Js)
    //             .then_with(move |r#type| {
    //                 space(ind + 1)
    //                     .then_with(move |ind| {
    //                         let ind: IndentBound = ind.into();
    //                         ident()
    //                             .then_ignore(space(ind + 1))
    //                             .then(path())
    //                             .map(move |(name, path)| {
    //                                 Item::Target(r#type, name, path)
    //                             })
    //                     })
    //                     .repeated()
    //             })
    //     })
    //     .repeated()
    //     .map(|items| items.into_iter().flatten().collect::<Vec<_>>());
    
    space(ind)
        .then_with(move |ind| {
            let ind = ind.into();
            just("val").to(ItemType::Val)
                .or(just("let").to(ItemType::LetIn))
                .or(just("mod").to(ItemType::Module))
                .or(just("sum").to(ItemType::Sum))
                .or(just("pro").to(ItemType::Product))
                .or(just("enum").to(ItemType::Enum))
                .or(just("from").to(ItemType::From))
                // .or(just("target").to(ItemType::Target))
                .then_with(move |r#type| {
                    match r#type {
                        ItemType::Val => val(ind).boxed(),
                        ItemType::Module => module(ind).boxed(),
                        ItemType::LetIn => let_in(ind).map(|item| vec![item]).boxed(),
                        ItemType::Sum => sum(ind).boxed(),
                        ItemType::Product => product(ind).boxed(),
                        ItemType::Enum => r#enum(ind).boxed(),
                        ItemType::From => from(ind).boxed(),
                        // ItemType::Target => target(ind).boxed()
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
