use chumsky::{Parser, text::keyword, prelude::Simple, primitive::just};
use crate::syntax::val::Value;
use super::{space, path, module, function};

mod string;
mod number;

pub fn value(ind: u16) -> impl Parser<char, Value, Error = Simple<char>> + Clone + Sized {
    let let_in = |ind| module::value(ind)
        .then_with(move |bindings| {
            space(ind)
            .ignore_then(keyword("in"))
            .then_ignore(space(ind))
            .ignore_then(value(ind).boxed())
            .map(move |value| Value::LetIn(Clone::clone(&bindings), Box::new(value)))
        });

    let val_in = |ind| module::val(ind)
        .then_with(move |bindings| {
            space(ind)
            .ignore_then(keyword("in"))
            .then_ignore(space(ind))
            .ignore_then(value(ind).boxed())
            .map(move |value| Value::LetIn(Clone::clone(&bindings), Box::new(value)))
        });

    let level1 = move |ind| 
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
                keyword("val")
                    .ignore_then(val_in(ind).boxed())
            )
            .or(
                number::value().map(Value::Number)
            )
            .or(
                path::value(ind).map(Value::Ref).boxed()
            )
            .or(
                just('\\')
                    .ignored()
                    .then_with(move |_| function::value(ind).boxed())
            )
            .or(
                string::value(ind).map(Value::String)
            );
            
    let call = move |ind| level1(ind)
        .then(
            space(ind)
            .then_with(level1)
            .repeated()
        )
        .foldl(|f, input| Value::Call(Box::new(f), Box::new(input)));

    space(ind)
        .then_with(call)
}
