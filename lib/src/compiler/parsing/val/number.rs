use chumsky::{Parser, error::Simple, primitive::any};
use crate::compiler::syntax::number::{Value, Item};

pub fn value() -> impl Parser<char, Value, Error = Simple<char>> + Clone + Sized {
    any()
        .try_map(|ch, span| {
            if let Some(item) = Item::from_char(ch) {
                Ok(item)
            } else {
                Err(Simple::custom(span, "Not a digit!"))
            }
        })
        .repeated()
        .at_least(1)
        .map(|mut items| {
            Value {
                last: items.pop().unwrap(),
                items,
            }
        })
}
