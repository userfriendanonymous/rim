use std::fmt::Display;
use chumsky::{Parser, error::Simple};
use serde::{Serialize, Deserialize};
use crate::Ident;

#[derive(Clone, Serialize, Deserialize)]
pub struct Path {
    path: super::Path,
    name: Ident
}

impl Path {
    pub fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
        super::Path::parser()
            .then(Ident::parser())
            .map(|(path, name)| Self { path, name })
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}-{}", self.path, self.name))
    }
}


#[derive(Serialize, Deserialize)]
pub struct Meta {
    packages: Vec<Ident>,
    directories: Vec<Ident>,
}