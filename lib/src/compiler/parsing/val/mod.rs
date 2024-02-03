use chumsky::{Parser, text::keyword, prelude::Simple, primitive::{just, empty}};
use crate::compiler::syntax::{val::{self as value, Value, infix::value as infix, InfixOp}, Number, Path};
use super::{space, space::IndentBound, path, module, function};
use infix_apply::{left as infix_apply_left, right as infix_apply_right};
pub use infix::value as infix;

mod string;
mod number;
mod infix_apply;
mod infix;

#[derive(Clone, Debug)]
pub enum Level1 {
    Scope,
    Let,
    Val,
    If,
    Lambda,
    Path(Path),
    Number(Number),
    String(String),
}

pub fn value(ind: IndentBound) -> impl Parser<char, Value, Error = Simple<char>> + Clone + Sized {
    let let_in = |ind: IndentBound| {
        let ind: IndentBound = ind.into();
        module::value(ind + 1)
            .then_with(move |bindings| {
                space(ind)
                .ignore_then(keyword("in"))
                .ignore_then(space(ind + 1))
                .then_with(|ind| value(ind.into()).boxed())
                .map(move |value| Value::LetIn(Clone::clone(&bindings), Box::new(value)))
            })
    };

    let val_in = |ind| module::val(ind + 1)
        .then_with(move |bindings| {
            space(ind)
            .ignore_then(just("in"))
            .ignore_then(space(ind + 1))
            .then_with(|ind| value(ind.into()).boxed())
            .map(move |value| Value::LetIn(Clone::clone(&bindings), Box::new(value)))
        });

    let r#if = |ind: IndentBound| space(ind + 1)
        .then_with(|ind| self::value(ind.into()).boxed())
        .then_ignore(space(ind))
        .then_ignore(just("then"))
        .then(space(ind + 1).then_with(|ind| self::value(ind.into()).boxed()))
        .then_ignore(space(ind))
        .then_ignore(just("else"))
        .then(space(ind + 1).then_with(|ind| self::value(ind.into()).boxed()))
        .map(|((cond, then), r#else)| Value::If(Box::new(cond), Box::new(then), Box::new(r#else)));

    let level1 = move |ind| 
        just('(').to(Level1::Scope).map(Ok)
            .or(just("if").to(Level1::If).map(Ok))
            .or(just("let").to(Level1::Let).map(Ok))
            .or(just("val").to(Level1::Val).map(Ok))
            .or(just("\\").to(Level1::Lambda).map(Ok))
            .or(number::value().map(Level1::Number).map(Ok))
            .or(string::value(ind).boxed().map(Level1::String).map(Ok))
            .or(path::value().boxed().map(Level1::Path).map(Ok))
            .try_map(|result, _| result)
            .then_with(move |branch| {
                match branch {
                    Level1::Scope => space(ind)
                        .ignore_then(self::value(ind))
                        .then_ignore(space(ind))
                        .then_ignore(just(')')).boxed(),
                    Level1::Let => let_in(ind).boxed(),
                    Level1::Val => val_in(ind).boxed(),
                    Level1::If => r#if(ind).boxed(),
                    Level1::Lambda => function::value(ind).boxed(),
                    Level1::Number(v) => empty().to(Value::Number(value::Number::Value(v))).boxed(),
                    Level1::String(v) => empty().to(Value::String(value::String::Value(v))).boxed(),
                    Level1::Path(v) => empty().to(Value::Ref(v)).boxed()
                }
            }).boxed();

    let apply = move |ind| level1(ind)
        .then(
            space(ind)
                .then_with(move |ind| {
                    let ind = ind.into();
                    just("in").or(just("then")).or(just("else")).to(())
                    .or_not()
                    .then_with(move |v| {
                        if let Some(_) = v {
                            empty().try_map(|_, span| Err(Simple::custom(span, "Unexpected 'in' keyword"))).boxed()
                        } else {
                            level1(ind).boxed()
                                // .or_else(|err| Ok(Err(err)))
                                // .then_with(|result| {
                                //     match result {
                                //         Ok(v) => empty().to(Ok(v)).boxed(),
                                //         Err(e) => any().to(Err(e)).boxed()
                                //     }
                                // })
                                // .boxed()
                        }
                    })
                })
                .repeated()
        )
        .foldl(|f, input| Value::Apply(Box::new(f), Box::new(input)))
        .boxed();

    infix_apply_right(
        ind,
        |v| Some(if v == infix!("$") { InfixOp::Apply } else { None? }),
        move |ind| infix_apply_left(
            ind,
            |v| Some(if v == infix!("<") { InfixOp::ApplyLeft } else if v == infix!(">") { InfixOp::ApplyRight } else { None? }),
            move |ind| infix_apply_left(
                ind,
                |v| Some(if v == infix!("<<") { InfixOp::ComposeLeft } else if v == infix!(">>") { InfixOp::ComposeRight } else { None? }),
                move |ind| infix_apply_right(
                    ind,
                    |v| Some(if v == infix!("$$") { InfixOp::Compose } else { None? }),
                    move |ind| infix_apply_left(
                        ind,
                        |v| Some(if v == infix!(":") { InfixOp::Push } else { None? }),
                        move |ind| infix_apply_left(
                            ind,
                            |v| Some(if v == infix!(",") { InfixOp::Pair } else { None? }),
                            move |ind| infix_apply_left(
                                ind,
                                |v| Some(if v == infix!("&") { InfixOp::And } else if v == infix!("|") { InfixOp::Or } else { None? }),
                                move |ind| infix_apply_left(
                                    ind,
                                    |v| Some(if v == infix!("+") { InfixOp::Add } else if v == infix!("-") { InfixOp::Sub } else { None? }),
                                    move |ind| infix_apply_left(
                                        ind,
                                        |v| Some(if v == infix!("*") { InfixOp::Mul } else if v == infix!("/") { InfixOp::Div } else { None? }),
                                        move |ind| infix_apply_left(
                                            ind,
                                            |v| Some(if v == infix!("%") { InfixOp::Modulo } else { None? }),
                                            apply
                                        )
                                    ).boxed()
                                )
                            ).boxed()
                        )
                    ).boxed()
                )
            ).boxed()
        )
    ).boxed()
}
