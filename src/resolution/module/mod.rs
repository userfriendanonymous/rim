use super::{Val, Globe, val, Type, globe::{ModuleId, ValId}};
use crate::{syntax::{self, Ident, Path}, resolution::module::r#where::LetIn};
pub use r#where::{Value as Where, MergeCollision};

pub mod r#where;

#[derive(Clone, Debug)]
pub enum Value {
    Where(Where),
    Ref(ModuleId),
}

#[derive(Clone, Debug)]
pub enum Error<'a> {
    Val(val::Error<'a>),
    ModuleNotFound(&'a Path, Option<usize>),
    ValNameTaken(&'a Ident, ValId),
    ModuleNameTaken(&'a Ident, ModuleId),
    LetInNameTaken(MergeCollision)
}

pub fn r#where<'a>(input: &'a [syntax::module::Item], env: Where, globe: &mut Globe) -> Result<Where, Error<'a>> {
    type E<'a> = Error<'a>;

    let mut value = Where::default();

    for item in input {
        match item {
            syntax::module::Item::Val(name, val) => {
                let val = val::out(val, env.clone(), globe).map_err(E::Val)?;
                let id = globe.new_val(Val::Out(val));
                value.merge_val(name.clone(), id).map_err(|id| E::ValNameTaken(name, *id))?;
            }

            syntax::module::Item::Module(name, module) => {
                let id = match module {
                    syntax::module::Module::Ref(path) => {
                        let id = *env.module_id_by_path(path, &globe).map_err(|e| E::ModuleNotFound(path, e))?;
                        globe.new_module(Value::Ref(id))
                    },
                    syntax::module::Module::Where(module_items) => {
                        let module = self::r#where(module_items, env.clone(), globe)?;
                        globe.new_module(Value::Where(module))
                    },
                    _ => todo!()
                };
                value.merge_module(name.clone(), id).map_err(|id| E::ModuleNameTaken(name, *id))?;
            },

            syntax::module::Item::LetIn(input, output) => {
                let input = self::r#where(input, env.clone(), globe)?;
                let new_env = env.clone().shadowed(input.clone());
                let output = self::r#where(output, new_env, globe)?;
                value.merge_let_in(LetIn {
                    input,
                    output
                }).map_err(E::LetInNameTaken)?;
            },

            syntax::module::Item::Sum(name, fields) => {
                let id = globe.new_type(Type::Sum(fields.len()));
                {
                    let id = globe.new_val(Val::Out(val::Out::SumMatch(id)));
                    value.merge_val(name.clone(), id).map_err(|&id| E::ValNameTaken(name, id))?;
                }
                for (idx, field) in fields.into_iter().enumerate() {
                    let field_id = globe.new_val(Val::Out(val::Out::Sum(idx, id)));
                    value.merge_val(field.clone(), field_id).map_err(|&id| E::ValNameTaken(field, id))?;
                }
            }

            syntax::module::Item::Product(name, fields) => {
                let id = globe.new_type(Type::Product(fields.len()));
                {
                    let id = globe.new_val(Val::Out(val::Out::Product(id)));
                    value.merge_val(name.clone(), id).map_err(|&id| E::ValNameTaken(name, id))?;
                }
                for (idx, field) in fields.into_iter().enumerate() {
                    let field_id = globe.new_val(Val::Out(val::Out::ProductField(idx, id)));
                    value.merge_val(field.clone(), field_id).map_err(|&id| E::ValNameTaken(field, id))?;
                }
            },

            syntax::module::Item::Enum(name, fields) => {
                todo!()
            }
        }
    }

    Ok(value)
}
