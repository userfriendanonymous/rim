use chumsky::{Parser, error::Simple};
use crate::compiler::{syntax::val::{Value, InfixOp, Infix}, parsing::{space, space::IndentBound}};
use super::infix;

pub fn left<SP: Parser<char, Value, Error = Simple<char>> + Clone + Sized + 'static>(
    ind: IndentBound,
    to_op: impl Fn(Infix) -> Option<InfixOp> + 'static,
    side: impl Fn(IndentBound) -> SP,
) -> impl Parser<char, Value, Error = Simple<char>> + Clone + Sized {
    side(ind).map(Ok).or_else(|e| Ok(Err(e)))
        .then(
            space(ind)
                .ignore_then(infix())
                .try_map(move |v, span| to_op(v).ok_or(Simple::custom(span, "Failed to turn infix into an op")))
                .then_ignore(space(ind))
                .then(side(ind).or_not())
                .repeated()
        )
        .foldl(|left, (op, right)|
            Ok(Value::InfixApply(Box::new(Value::InfixOp(op)), left.ok().map(Box::new), right.map(Box::new)))
        )
        .boxed()
        .try_map(|result, _| result)
        .boxed()
}

pub fn right<SP: Parser<char, Value, Error = Simple<char>> + Clone + Sized + 'static>(
    ind: IndentBound,
    to_op: impl Fn(Infix) -> Option<InfixOp> + 'static,
    side: impl Fn(IndentBound) -> SP,
) -> impl Parser<char, Value, Error = Simple<char>> + Clone + Sized {
    side(ind).or_not()
        .then_ignore(space(ind))
        .then(
            infix().try_map(move |v, span| to_op(v).ok_or(Simple::custom(span, "Failed to turn infix into an op")))
        )
        .then_ignore(space(ind))
        .repeated()
        .then(side(ind).map(Ok).or_else(|e| Ok(Err(e))))
        .foldr(|(left, op), right|
            Ok(Value::InfixApply(Box::new(Value::InfixOp(op)), left.map(Box::new), right.ok().map(Box::new)))
        )
        .boxed()
        .try_map(|result, _| result)
        .boxed()
}
