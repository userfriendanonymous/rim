use chumsky::{Parser, error::Simple, primitive::any};
use nonempty::NonEmpty;
use crate::compiler::syntax::val::infix::{Value, Item};

pub fn value() -> impl Parser<char, Value, Error = Simple<char>> {
    any()
        .try_map(|ch, span| {
            Item::from_char(ch).ok_or(Simple::custom(span, "Invalid infix item"))
        })
        .repeated()
        .at_least(1)
        .map(|v| Value(NonEmpty::from_vec(v).unwrap()))
}
