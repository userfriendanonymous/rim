use std::collections::BTreeMap;
use super::{Val, Globe, val, Type, globe::{ModuleId, ValId}};
use crate::syntax::{self, Ident, Path, ident};

#[derive(Clone, Debug)]
pub enum Value {
    Where(Where),
    Ref(ModuleId),
}

#[derive(Clone, Debug, Default)]
pub struct LetIn {
    pub input: Where,
    pub output: Vec<LetIn>,
}

#[derive(Clone, Debug, Default)]
pub struct Where {
    let_ins: Vec<LetIn>,
    modules: BTreeMap<Ident, ModuleId>,
    vals: BTreeMap<Ident, ValId>
}

impl Where {
    pub fn main_val(&self) -> Option<&ValId> {
        self.val(&ident::main())
    }

    pub fn modules(&self) -> &BTreeMap<Ident, ModuleId> {
        &self.modules
    }

    pub fn vals(&self) -> &BTreeMap<Ident, ValId> {
        &self.vals
    }

    pub fn let_ins(&self) -> &Vec<LetIn> {
        &self.let_ins
    }

    pub fn to_path<'a>(&'a self, path_items: &[Ident], globe: &'a Globe) -> Result<&'a Where, usize> {
        let mut module = self;
        for (id, name) in path_items.into_iter().enumerate() {
            let mut value = globe.module(module.module(name).ok_or(id)?);
            module = loop {
                match value {
                    Value::Ref(id) => value = globe.module(id),
                    Value::Where(value) => break value
                }
            };
        }
        Ok(module)
    }

    pub fn module_id_by_path<'a>(&'a self, path: &Path, globe: &'a Globe) -> Result<&'a ModuleId, Option<usize>> {
        let module = self.to_path(&path.items, globe).map_err(Some)?;
        module.modules.get(&path.name).ok_or(None)
    }

    pub fn value_id_by_path<'a>(&'a self, path: &Path, globe: &'a Globe) -> Result<&'a ValId, Option<usize>> {
        let module = self.to_path(&path.items, globe).map_err(Some)?;
        module.vals.get(&path.name).ok_or(None)
    }

    pub fn module(&self, name: &Ident) -> Option<&ModuleId> {
        self.modules.get(name)
    }

    pub fn val(&self, name: &Ident) -> Option<&ValId> {
        self.vals.get(name)
    }

    pub fn append(mut self, mut other: Self) -> Self {
        self.modules.append(&mut other.modules);
        self.vals.append(&mut other.vals);
        self
    }

    pub fn with_val(mut self, name: Ident, id: ValId) -> Self {
        self.add_val(name, id);
        self
    }

    pub fn add_val(&mut self, name: Ident, id: ValId) {
        self.vals.insert(name, id);
    }

    pub fn add_module(&mut self, name: Ident, id: ModuleId) {
        self.modules.insert(name, id);
    }

    pub fn add_let_in(&mut self, value: LetIn) {
        self.let_ins.push(value);
    }

    pub fn try_add_val(&mut self, name: Ident, id: ValId) -> Result<(), ValId> {
        if let Some(id) = self.vals.get(&name) {
            Err(*id)
        } else {
            self.vals.insert(name, id);
            Ok(())
        }
    }

    pub fn try_add_module(&mut self, name: Ident, id: ModuleId) -> Result<(), ModuleId> {
        if let Some(id) = self.modules.get(&name) {
            Err(*id)
        } else {
            self.modules.insert(name, id);
            Ok(())
        }
    }

    pub fn try_append(&mut self, other: Self) -> Result<(), TryAppendError> {
        type E = TryAppendError;
        for (name, id) in other.vals {
            self.try_add_val(name, id).map_err(E::Val)?;
        }
        for (name, id) in other.modules {
            self.try_add_module(name, id).map_err(E::Module)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum TryAppendError {
    Val(ValId),
    Module(ModuleId),
}

#[derive(Clone, Debug)]
pub enum Error<'a> {
    Val(val::Error<'a>),
    ModuleNotFound(&'a Path, Option<usize>),
    ValNameTaken(&'a Ident, ValId),
    ModuleNameTaken(&'a Ident, ModuleId),
    LetInNameTaken(TryAppendError)
}

pub fn r#where<'a>(input: &'a [syntax::module::Item], env: Where, globe: &mut Globe) -> Result<Where, Error<'a>> {
    type E<'a> = Error<'a>;

    let mut value = Where::default();

    for item in input {
        match item {
            syntax::module::Item::Val(name, val) => {
                let val = val::out(val, env.clone(), globe).map_err(E::Val)?;
                let id = globe.new_val(Val::Out(val));
                value.try_add_val(name.clone(), id).map_err(|id| E::ValNameTaken(name, id))?;
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
                value.try_add_module(name.clone(), id).map_err(|id| E::ModuleNameTaken(name, id))?;
            },

            syntax::module::Item::LetIn(input, output) => {
                let input = self::r#where(input, env.clone(), globe)?;
                let new_env = env.clone().append(input.clone());
                let output = self::r#where(output, new_env, globe)?;
                value.add_let_in(LetIn { input, output: output.let_ins.clone() });
                value.try_append(output).map_err(E::LetInNameTaken)?;
            },

            syntax::module::Item::Sum(name, fields) => {
                let id = globe.new_type(Type::Sum(fields.len()));
                {
                    let id = globe.new_val(Val::Out(val::Out::SumMatch(id)));
                    value.try_add_val(name.clone(), id).map_err(|id| E::ValNameTaken(name, id))?;
                }
                for (idx, field) in fields.into_iter().enumerate() {
                    let field_id = globe.new_val(Val::Out(val::Out::SumInit(idx, id)));
                    value.try_add_val(field.clone(), field_id).map_err(|id| E::ValNameTaken(field, id))?;
                }
            }

            syntax::module::Item::Product(name, fields) => {
                let id = globe.new_type(Type::Product(fields.len()));
                {
                    let id = globe.new_val(Val::Out(val::Out::ProductInit(id)));
                    value.try_add_val(name.clone(), id).map_err(|id| E::ValNameTaken(name, id))?;
                }
                for (idx, field) in fields.into_iter().enumerate() {
                    let field_id = globe.new_val(Val::Out(val::Out::ProductField(idx, id)));
                    value.try_add_val(field.clone(), field_id).map_err(|id| E::ValNameTaken(field, id))?;
                }
            },

            syntax::module::Item::Enum(name, fields) => {
                todo!()
            }
        }
    }

    Ok(value)
}

