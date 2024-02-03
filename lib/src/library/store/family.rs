use std::{fmt::Display, str::FromStr};

use chumsky::error::Simple;
use serde::{Serialize, Deserialize};
use super::package;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meta {
    pub next_version: package::Version,
}

impl Meta {
    pub fn next_version(mut self) -> (package::Version, Self) {
        let version = self.next_version;
        self.next_version = self.next_version.succ();
        (version, self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Path(super::Path);

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Path {
    type Err = Vec<Simple<char>>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        super::Path::from_str(s).map(Self)
    }
}