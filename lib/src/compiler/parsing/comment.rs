use chumsky::{Parser, error::Simple, primitive::{just, any}};

use super::space::{self, IndentBound};


pub fn value(ind: IndentBound) -> impl Parser<char, String, Error = Simple<char>> + Clone + Sized {
    just('#')
        .ignore_then(
            any()
                .try_map(|ch, span| {
                    if ch == ' ' || ch == '\n' {
                        Err(Simple::custom(span, "Unexpected space"))
                    } else {
                        Ok(ch)
                    }
                })
                .repeated()
                .collect::<String>()
        )
        .then_with(move |literal| {
            space::indent(ind)
                .ignore_then(just(literal.clone()))
                .ignore_then(just('#'))
                .to(None)
                .or(
                    just('\n').then_ignore(space::indent(ind)).map(Some)
                )
                .or(any().map(Some))
                .try_map(|v, span| {
                    if let Some(v) = v {
                        Ok(v)
                    } else {
                        Err(Simple::custom(span, "unexpected None"))
                    }
                })
                .repeated()
                .collect::<String>()
                .then_ignore(
                    space::indent(ind)
                        .ignore_then(just(literal))
                        .ignore_then(just('#'))
                )
        })
}
