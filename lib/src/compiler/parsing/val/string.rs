use chumsky::{Parser, prelude::Simple, primitive::{any, just}};

use crate::compiler::parsing::{space, space::IndentBound};


pub fn value(ind: IndentBound) -> impl Parser<char, String, Error = Simple<char>> + Clone + Sized {
    just('"')
        .ignore_then(
            just('\n')
                .then_ignore(space(ind))
                .or(
                    any()
                    .try_map(|ch, span| {
                        if ch == '"' {
                            Err(Simple::custom(span, "Unexpected end"))
                        } else {
                            Ok(ch)
                        }
                    })
                )
                .repeated()
                .collect::<String>()
                .then_ignore(just('"'))
        )
}
