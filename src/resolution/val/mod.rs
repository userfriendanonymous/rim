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
        syntax::Val::Call(f, input) => {
            let f = out(f.as_ref(), env.clone(), globe)?;
            let input = out(input.as_ref(), env.clone(), globe)?;
            Out::Call(Box::new(f), Box::new(input))
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
            Out::String(v.clone())
        },
        syntax::Val::Number(v) => {
            Out::Number(v.clone())
        }
    })
}

