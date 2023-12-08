use std::fmt::Display;

use chumsky::{Parser, error::Simple, primitive::just};
use serde::{Serialize, Deserialize};

use crate::Ident;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Value(Vec<Ident>);

impl Value {
    pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        Ident::parser()
            .then_ignore(just('-'))
            .repeated()
            .map(Self)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("-"))
    }
}