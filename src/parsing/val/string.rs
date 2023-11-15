use chumsky::{Parser, prelude::Simple, primitive::{any, custom, just}};

use crate::parsing::space;


pub fn value(ind: u16) -> impl Parser<char, String, Error = Simple<char>> + Clone + Sized {
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
