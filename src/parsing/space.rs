

use chumsky::{Parser, primitive::{just, any}, prelude::Simple};

use super::comment;

// value - minimum indentation allowed
pub fn indent(value: u16) -> impl Parser<char, u16, Error = Simple<char>> + Clone + Sized {
    just(' ')
        .repeated()
        .ignore_then(
            just('\n')
            .ignore_then(
                just(' ')
                .repeated()
                .map(|v| v.len() as u16)
            )
            .repeated()
        )
        .try_map(move |vals, span| {
            let new = *vals.last().unwrap_or(&value);
            if new < value { Err(Simple::custom(span, "Wrong indentation")) } else { Ok(new) }
        })
}

pub fn value(ind: u16) -> impl Parser<char, u16, Error = Simple<char>> + Clone + Sized {
    indent(ind)
        .then(comment(ind))
        .ignored()
        .repeated()
        .ignore_then(indent(ind))
}
