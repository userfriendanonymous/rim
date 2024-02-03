use crate::resolution::globe::TypeId;
use std::iter::zip;
use super::{PolyType, VarId, Bounds};

#[derive(Clone, Debug)]
pub enum MergeError {
    DifferentCons
}

#[derive(Clone)]
pub enum Value {
    Cons(TypeId, Vec<PolyType>),
    Var(VarId),
}

impl Value {
    pub fn replace_var(&mut self, id: VarId, with: Value) {
        match self {
            Self::Var(id) => *self = with,
            Self::Cons(_, args) => {
                for arg in args {
                    arg.t.replace_var(id, with.clone());
                }
            },
            Self::Some => Self::Some
        }
    }
    //     flow(&self, to: &Self)
    pub fn flow(&self, other: &Self) -> Result<Bounds, ()> {
        match (self, other) {
            (Self::Cons(id, args), Self::Cons(other_id, other_args)) => {
                if id == other_id {
                    let mut bounds = Bounds::default();
                    for (arg, other_arg) in zip(args, other_args) {
                        match arg.base.merge(&other_arg.base) {
                            Ok((base, bounds)) => {
                                
                            },
                            Err(err) => {

                            }
                        }
                        let arg_bounds = arg.flow(&other_arg)?;
                        bounds = bounds.merge(arg_bounds);
                    }
                    Ok(bounds)
                } else {
                    Err(())
                }
            },
            (Self::Var(id), other) => {
                let mut bounds = Bounds::default();
                bounds.put(Self::Var(*id), other.clone());
                Ok(bounds)
            },
            (self_, Self::Var(other_id)) => {
                let mut bounds = Bounds::default();
                bounds.put(Self::Var(*other_id), self_.clone());
                Ok(bounds)
            }
        }
    }

    pub fn merge(&self, other: &Self) -> Result<(Self, Bounds), MergeError> {
        type E = MergeError;
        match (self, other) {
            (Self::Cons(id, args), Self::Cons(other_id, other_args)) => {
                if id == other_id {
                    let mut new_args = Vec::new();
                    let mut bounds = Bounds::default();

                    for (arg, other_arg) in zip(args, other_args) {
                        arg.base.merge(&other_arg.base)
                        let (arg, arg_bounds) = arg.merge(other_arg);
                        if !arg_bounds.check_bindings(&arg.bindings) {
                            return None
                        }

                        new_args.push(arg);
                        bounds = bounds.merge(arg_bounds);
                    }
                    
                    Ok((Self::Cons(*id, new_args), bounds))
                } else {
                    Err(E::DifferentCons)
                }
            },

            (Self::Var(id), Self::Var(other_id)) => {
                if id == other_id {
                    // assert!(!bindings.contains(id));
                    // assert!(!other_bindings.contains(id));
                    Ok((Self::Var(*id), Bounds::default()))
                } else {
                    let mut bounds = Bounds::default();
                    bounds.put_var_and_var(*id, *other_id);
                    Ok((Self::Var(*id), bounds))
                }
            },

            (Self::Var(id), poly_type)
            | (poly_type, Self::Var(id)) => {
                let mut bounds = Bounds::default();
                bounds.put_var_and_poly_type(*id, poly_type.clone());
                Some((Self::Var(*id), bounds))
            },
        };
    }
}