use chumsky::{Parser, primitive::just, prelude::Simple};
use crate::syntax::{function::Value, Val};
use super::{ident, space, val, space::IndentBound};

pub fn value(ind: IndentBound) -> impl Parser<char, Val, Error = Simple<char>> {
    ident()
        .then_ignore(space(ind))
        .repeated()
        .then_ignore(just('='))
        .then_ignore(space(ind))
        .then(val(ind).boxed())
        .foldr(|input, output| Val::Function(Box::new(Value { input, output })))
}
