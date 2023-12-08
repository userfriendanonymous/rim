use chumsky::{Parser, prelude::Simple};
use crate::syntax::ident::Value;

mod item;

pub fn value() -> impl Parser<char, Value, Error = Simple<char>> + Clone {
    item::value()
        .repeated()
        .try_map(|mut others, span| {
            let last = others.pop().ok_or(Simple::custom(span, "ident can't be empty"))?;
            Ok(Value { last, others })
        })
}
