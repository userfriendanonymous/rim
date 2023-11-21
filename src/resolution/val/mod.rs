use crate::syntax::{Ident, self};
use super::{Globe, Env, module};
pub use out::Value as Out;

pub mod out;

#[derive(Clone, Debug)]
pub enum Value {
    In,
    Out(Out)
}


#[derive(Clone, Debug)]
pub enum Error<'a> {
    PathNotFound(&'a [Ident], Option<usize>),
    LetInInput(Box<module::Error<'a>>)
}

pub fn out<'a>(input: &'a syntax::Val, env: Env, globe: &mut Globe) -> Result<Out, Error<'a>> {
    type E<'a> = Error<'a>;

    Ok(match input {
        syntax::Val::Ref(path) => {
            Out::Ref(*env.val_id_by_path(path, &globe).map_err(|e| E::PathNotFound(&path.items, e))?)
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
        syntax::Val::String(v) => {
            Out::String(out::String::Value(v.clone()))
        },
        syntax::Val::Number(v) => {
            Out::Number(out::Number::Value(v.clone()))
        },
        syntax::Val::InfixOp(op) => match op {
            syntax::val::InfixOp::Add => Out::Number(out::Number::Add),
            syntax::val::InfixOp::Sub => Out::Number(out::Number::Sub),
            syntax::val::InfixOp::Mul => Out::Number(out::Number::Mul),
            syntax::val::InfixOp::Div => Out::Number(out::Number::Div),
            syntax::val::InfixOp::ApplyLeft => {
                let input_id = globe.new_val(Value::In);
                Out::Function(input_id, Box::new({
                    let f_id = globe.new_val(Value::In);
                    Out::Function(f_id, Box::new(Out::Apply(Box::new(Out::Ref(f_id)), Box::new(Out::Ref(input_id)))))
                }))
            },
            syntax::val::InfixOp::ApplyRight => {
                let f_id = globe.new_val(Value::In);
                Out::Function(f_id, Box::new({
                    let input_id = globe.new_val(Value::In);
                    Out::Function(input_id, Box::new(Out::Apply(Box::new(Out::Ref(f_id)), Box::new(Out::Ref(input_id)))))
                }))
            },
            syntax::val::InfixOp::Apply => {
                let f_id = globe.new_val(Value::In);
                Out::Function(f_id, Box::new({
                    let input_id = globe.new_val(Value::In);
                    Out::Function(input_id, Box::new(Out::Apply(Box::new(Out::Ref(f_id)), Box::new(Out::Ref(input_id)))))
                }))
            }
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
