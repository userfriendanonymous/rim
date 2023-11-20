use chumsky::{Parser, primitive::just, prelude::Simple};
use super::{ident, space::IndentBound, space};
use crate::syntax::path::Value;


pub fn value() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    ident::value()
        .then_ignore(just('.'))
        .repeated()
        .then(ident::value())
        .map(|(items, name)| Value { items, name })
}
