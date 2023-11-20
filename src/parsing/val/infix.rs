use chumsky::{Parser, error::Simple};
use crate::{syntax::val::{Value, Infix}, parsing::{space, space::IndentBound}};

pub fn value<SP: Parser<char, Value, Error = Simple<char>> + Clone + Sized, IP: Parser<char, Infix, Error = Simple<char>> + Clone + Sized>(
    ind: IndentBound, infix: impl Fn(IndentBound) -> IP, side: impl Fn(IndentBound) -> SP,
) -> impl Parser<char, Value, Error = Simple<char>> + Clone + Sized {
    side(ind).map(Ok).or_else(|e| Ok(Err(e)))
        .then(
            space(ind)
                .ignore_then(infix(ind))
                .then_ignore(space(ind))
                .then(side(ind).or_not())
                .repeated()
        )
        .foldl(|left, (op, right)|
            Ok(Value::InfixApply(Box::new(Value::Infix(op)), left.ok().map(Box::new), right.map(Box::new)))
        )
        .try_map(|result, _| result)
}
