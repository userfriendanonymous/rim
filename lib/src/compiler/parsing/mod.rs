use chumsky::{Parser, prelude::{Simple, end}};
use space::value as space;
use path::value as path;
use val::value as val;
use ident::value as ident;
use comment::value as comment;
use crate::compiler::syntax;
use space::IndentBound;

pub mod ident;
pub mod val;
pub mod path;
pub mod space;
pub mod module;
pub mod function;
pub mod comment;

pub fn value(ind: IndentBound) -> impl Parser<char, Vec<syntax::module::Item>, Error = Simple<char>> {
    space(ind)
        .ignore_then(module::value(ind))
        .then_ignore(space(ind))
        .then_ignore(end())
}
