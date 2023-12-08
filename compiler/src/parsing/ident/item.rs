use chumsky::{Parser, primitive::filter_map, prelude::Simple};
use crate::syntax::ident::item::{Value, WithCasing};

pub fn value() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    filter_map(|span, c: char| Value::from_char(c).ok_or(Simple::custom(span, "invalid char")))
}
