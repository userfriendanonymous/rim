use std::array;

use crate::syntax;
use super::{Globe, Env, module, globe::ValId};
pub use out::Value as Out;

pub mod out;

fn curried_function<const LEN: usize>(globe: &mut Globe, f: impl FnOnce([ValId; LEN]) -> Out) -> Out {
    let ids = array::from_fn(|_| globe.new_val(Value::In));
    let body = f(ids.clone());
    ids.into_iter().rev().fold(body, |body, id| {
        Out::Function(id, Box::new(body))
    })
}

#[derive(Clone, Debug)]
pub enum Value {
    In,
    Out(Out)
}


#[derive(Clone, Debug)]
pub enum Error {
    PathNotFound(syntax::Path, Option<usize>),
    LetInInput(Box<module::Error>)
}

pub fn out<'a>(input: &'a syntax::Val, env: Env, globe: &mut Globe) -> Result<Out, Error> {
    type E = Error;

    Ok(match input {
        syntax::Val::Ref(path) => {
            Out::Ref(*env.val_id_by_path(path, &globe).map_err(|e| E::PathNotFound(path.clone(), e))?)
        }
        syntax::Val::Apply(f, input) => {
            let f = out(f.as_ref(), env.clone(), globe)?;
            let input = out(input.as_ref(), env.clone(), globe)?;
            Out::Apply(Box::new(f), Box::new(input))
        },
        syntax::Val::Function(f) => {
            let id = globe.new_val(Value::In);
            let mut env = env.clone();
            env.shadow_val(f.input.clone(), id);
            let out = out(&f.output, env, globe)?;
            Out::Function(id, Box::new(out))
        },
        syntax::Val::LetIn(input, output) => {
            let input = module::r#where(input, env.clone(), globe).map_err(|e| E::LetInInput(Box::new(e)))?;
            Out::LetIn(input.clone(), Box::new(out(output.as_ref(), env.clone().shadowed(input), globe)?))
        },
        syntax::Val::If(cond, then, otherwise) => {
            let cond = self::out(&cond, env.clone(), globe)?;
            let then = self::out(&then, env.clone(), globe)?;
            let otherwise = self::out(&otherwise, env, globe)?;
            Out::apply(
                Out::apply(Out::apply(Out::Boolean(out::Boolean::Match), otherwise), then),
                cond,
            )
        },
        syntax::Val::String(v) => Out::String(v.clone()),
        syntax::Val::Number(v) => Out::Number(v.clone()),
        syntax::Val::Js(v) => Out::Js(v.clone()),
        syntax::Val::Boolean(v) => Out::Boolean(v.clone()),
        syntax::Val::InfixOp(op) => match op {
            syntax::val::InfixOp::Add => Out::Number(out::Number::Add),
            syntax::val::InfixOp::Sub => Out::Number(out::Number::Sub),
            syntax::val::InfixOp::Mul => Out::Number(out::Number::Mul),
            syntax::val::InfixOp::Div => Out::Number(out::Number::Div),
            syntax::val::InfixOp::Modulo => Out::Number(out::Number::Modulo),

            syntax::val::InfixOp::And => Out::Boolean(out::Boolean::And),
            syntax::val::InfixOp::Or => Out::Boolean(out::Boolean::Or),

            syntax::val::InfixOp::ApplyLeft => curried_function(globe, |[i, f]| {
                Out::apply(Out::Ref(f), Out::Ref(i))
            }),
            syntax::val::InfixOp::ApplyRight => curried_function(globe, |[f, i]| {
                Out::apply(Out::Ref(f), Out::Ref(i))
            }),
            syntax::val::InfixOp::Apply => curried_function(globe, |[f, i]| {
                Out::apply(Out::Ref(f), Out::Ref(i))
            }),
            syntax::val::InfixOp::ComposeLeft => curried_function(globe, |[i, f, x]| {
                Out::apply(Out::Ref(f), Out::apply(Out::Ref(i), Out::Ref(x)))
            }),
            syntax::val::InfixOp::ComposeRight => curried_function(globe, |[f, i, x]| {
                Out::apply(Out::Ref(f), Out::apply(Out::Ref(i), Out::Ref(x)))
            }),
            syntax::val::InfixOp::Compose => curried_function(globe, |[f, i, x]| {
                Out::apply(Out::Ref(f), Out::apply(Out::Ref(i), Out::Ref(x)))
            }),
        },
        syntax::Val::InfixApply(f, left, right) => {
            let f = out(f, env.clone(), globe)?;
            let left = left.as_ref().map(|v| out(&v, env.clone(), globe)).transpose()?;
            let right = right.as_ref().map(|v| out(&v, env.clone(), globe)).transpose()?;
            match (left, right) {
                (Some(left), Some(right)) => Out::Apply(Box::new(Out::Apply(Box::new(f), Box::new(left))), Box::new(right)),
                (Some(left), None) => Out::Apply(Box::new(f), Box::new(left)),
                (None, Some(right)) => {
                    let left_id = globe.new_val(Value::In);
                    Out::Function(left_id, Box::new(Out::Apply(Box::new(Out::Apply(Box::new(f), Box::new(Out::Ref(left_id)))), Box::new(right))))
                },
                (None, None) => f
            }
        }
    })
}
