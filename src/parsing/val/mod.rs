use chumsky::{Parser, text::keyword, prelude::Simple, primitive::just};
use crate::syntax::val::Value;
use super::{space, path, module, function};

pub fn value(ind: u16) -> impl Parser<char, Value, Error = Simple<char>> + Clone + Sized {
    let let_in = |ind| module::value(ind + 1)
        .then_with(move |bindings| {
            space(ind)
            .ignore_then(keyword("in"))
            .then_ignore(space(ind + 1))
            .ignore_then(value(ind + 1).boxed())
            .map(move |value| Value::LetIn(Clone::clone(&bindings), Box::new(value)))
        });

    let level1 = |ind| 
        just('(')
            .ignored()
            .then_with(move |_| self::value(ind).boxed())
            .then_ignore(space(ind))
            .then_ignore(just(')'))
            .or(
                keyword("let")
                    .ignore_then(let_in(ind).boxed())
            )
            .or(
                path::value(ind).map(Value::Ref).boxed()
            )
            .or(
                just('\\')
                    .ignored()
                    .then_with(move |_| function::value(ind).boxed())
            );
            
    let call = |ind| level1(ind)
        .then(
            space(ind)
            .ignore_then(level1(ind))
            .repeated()
        )
        .foldl(|f, input| Value::Call(Box::new(f), Box::new(input)));

    space(ind)
        .ignore_then(call(ind))
}
