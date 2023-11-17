use chumsky::{Parser, text::keyword, prelude::Simple, primitive::{just, empty}};
use crate::syntax::{val::Value, Number, Path};
use super::{space, space::IndentBound, path, module, function};

mod string;
mod number;

#[derive(Clone, Debug)]
pub enum Level1 {
    Scope,
    Let,
    Val,
    Lambda,
    Path(Path),
    Number(Number),
    String(String),
}

pub fn value(ind: IndentBound) -> impl Parser<char, Value, Error = Simple<char>> + Clone + Sized {
    let let_in = |ind: IndentBound| {
        let ind: IndentBound = ind.into();
        module::value(ind + 1)
            .then_with(move |bindings| {
                space(ind)
                .ignore_then(keyword("in"))
                .ignore_then(space(ind + 1))
                .then_with(|ind| value(ind.into()).boxed())
                .map(move |value| Value::LetIn(Clone::clone(&bindings), Box::new(value)))
            })
    };

    let val_in = |ind| module::val(ind + 1)
        .then_with(move |bindings| {
            space(ind)
            .ignore_then(keyword("in"))
            .ignore_then(space(ind + 1))
            .then_with(|ind| value(ind.into()).boxed())
            .map(move |value| Value::LetIn(Clone::clone(&bindings), Box::new(value)))
        });

    let level1 = move |ind| 
        just('(').to(Level1::Scope).map(Ok)
            .or(just("let").to(Level1::Let).map(Ok))
            .or(just("val").to(Level1::Val).map(Ok))
            .or(just("\\").to(Level1::Lambda).map(Ok))
            .or(number::value().map(Level1::Number).map(Ok))
            .or(string::value(ind).map(Level1::String).map(Ok))
            .or(path::value(ind).map(Level1::Path).map(Ok))
            .try_map(|result, _| result)
            .then_with(move |branch| {
                match branch {
                    Level1::Scope => self::value(ind)
                        .then_ignore(space(ind))
                        .then_ignore(just(')')).boxed(),
                    Level1::Let => let_in(ind).boxed(),
                    Level1::Val => val_in(ind).boxed(),
                    Level1::Lambda => function::value(ind).boxed(),
                    Level1::Number(v) => empty().to(Value::Number(v)).boxed(),
                    Level1::String(v) => empty().to(Value::String(v)).boxed(),
                    Level1::Path(v) => empty().to(Value::Ref(v)).boxed()
                }
            });

        // just('(')
        //     .ignored()
        //     .then_with(move |_| self::value(ind).boxed())
        //     .then_ignore(space(ind, 0))
        //     .then_ignore(just(')'))
        //     .or(
        //         keyword("let")
        //             .ignore_then()
        //     )
        //     .or(
        //         keyword("val")
        //             .ignore_then(val_in(ind).boxed())
            // )
            // .or(
            //     number::value().map(Value::Number)
            // )
            // .or(
            //     path::value(ind).map(Value::Ref).boxed()
            // )
            // .or(
            //     just('\\')
            //         .ignored()
            //         .then_with(move |_| function::value(ind).boxed())
            // )
            // .or(
            //     string::value(ind).map(Value::String)
            // );
            
    let call = move |ind| level1(ind)
        .then(
            space(ind)
                .then_with(move |ind| {
                    let ind = ind.into();
                    just("in").to(())
                    .or_not()
                    .then_with(move |v| {
                        if let Some(_) = v {
                            empty().try_map(|_, span| Err(Simple::custom(span, "Unexpected 'in' keyword"))).boxed()
                        } else {
                            level1(ind).boxed()
                                // .or_else(|err| Ok(Err(err)))
                                // .then_with(|result| {
                                //     match result {
                                //         Ok(v) => empty().to(Ok(v)).boxed(),
                                //         Err(e) => any().to(Err(e)).boxed()
                                //     }
                                // })
                                // .boxed()
                        }
                    })
                })
                .repeated()
        )
        .foldl(|f, input| Value::Call(Box::new(f), Box::new(input)));
        // .foldl(|f: Result<_, _>, input: Result<_, _>| match (f, input) {
        //     (Ok(f), Ok(input)) => Ok(Value::Call(Box::new(f), Box::new(input))),
        //     (Err(f_error), Err(input_error)) => Err(f_error.merge(input_error)),
        //     (Err(f_error), _) => Err(f_error),
        //     (_, Err(input_error)) => Err(input_error)
        // })
        // .try_map(move |result, _| result);

    call(ind)
}
