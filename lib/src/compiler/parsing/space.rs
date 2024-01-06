
use chumsky::{Parser, primitive::just, prelude::Simple};
use super::comment;
use core::ops::Add;

#[derive(Clone, Copy, Debug, Default)]
pub struct Indent(u16);

#[derive(Clone, Copy, Debug, Default)]
pub struct IndentBound {
    value: Indent,
    more: Indent,
}

impl IndentBound {
    pub fn add_value(&self, v: u16) -> Self {
        Self {
            value: Indent(self.value.0 + v),
            more: self.more,
        }
    }
}

impl Add<u16> for IndentBound {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        Self {
            value: self.value,
            more: Indent(self.more.0 + rhs)
        }
    }
}

impl Add<Indent> for IndentBound {
    type Output = Self;
    fn add(self, rhs: Indent) -> Self::Output {
        self + rhs.0
    }
}

impl From<Indent> for IndentBound {
    fn from(value: Indent) -> Self {
        Self {
            value,
            more: Indent(0)
        }
    }
}

pub fn indent(bound: IndentBound) -> impl Parser<char, Indent, Error = Simple<char>> + Clone + Sized {
    just(' ')
        .repeated()
        .ignore_then(
            just('\n')
            .ignore_then(
                just(' ')
                .repeated()
                .map(|v| Indent(v.len() as u16))
            )
            .repeated()
        )
        .try_map(move |vals, span| {
            if let Some(&new) = vals.last() {
                if new.0 < bound.value.0 + bound.more.0 {
                    Err(Simple::custom(span, "Wrong indentation"))
                } else {
                    Ok(new)
                }
            } else {
                Ok(bound.value)
            }
        })
}

pub fn value(bound: IndentBound) -> impl Parser<char, Indent, Error = Simple<char>> + Clone + Sized {
    indent(bound)
        .then(comment(bound))
        .ignored()
        .repeated()
        .ignore_then(indent(bound))
}
