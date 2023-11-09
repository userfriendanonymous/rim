use chumsky::{Parser, primitive::just, prelude::Simple};
use super::{ident, space};
use crate::syntax::path::Value;


pub fn value(ind: u16) -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    ident::value()
        .then_ignore(space(ind))
        .then_ignore(just('.'))
        .repeated()
        .then(ident::value())
        .map(|(items, name)| Value { items, name })
}
