use super::{VarId, BaseType, Bounds};

#[derive(Clone)]
pub struct Value {
    pub bindings: Vec<VarId>,
    pub base: BaseType
}

impl Value {
    pub fn apply_bounds(&mut self, bounds: &mut Bounds) {
        for binding in self.bindings {
            if let Some(poly_type) = bounds.remove(&binding) {
                self.base.replace_var(binding, poly_type);
            }
        }
    }

    pub fn flow(&self, other: &Self) -> Result<Bounds, ()> {
        let bounds = self.t.flow(&other.t)?;

        let mut bindings = self.bindings.clone();
        bindings.append(&mut other.bindings);

        if bounds.check_bindings(&bindings) {
            Ok(bounds)
        } else {
            Err(())?
        }
    }

    // pub fn merge(mut self, mut other: Self, globe: &mut Value) -> (Self, Bounds) {
    //     self.bindings.append(&mut other.bindings);
    //     Self {
    //         bindings: self.bindings,
    //         t: self.t.merge(other, globe)
    //     }
    // }

    pub fn vars_cloned(&self, globe: &mut Value) -> Self {
        let mut new = self.clone();
        for id in new.bindings {
            new.t.replace_var(id, Type::Var(globe.new_type_var()));
        }
        if let Type::Cons(_, args) = &mut new.t {
            for arg in args {
                *arg = arg.vars_cloned(globe)
            }
        }
        new
    }

    pub fn merge(&self, other: &Self, globe: &mut Value) -> (Self, Bounds) {
        let v = match (self, other) {
            (
                Self { bindings, t: Type::Cons(id, args) },
                Self { bindings: other_bindings, t: Type::Cons(other_id, other_args) }
            ) => {
                if id == other_id {
                    let mut new_args = Vec::new();
                    let mut bounds = Bounds::default();
                    let mut valid = true;

                    for (arg, other_arg) in zip(args, other_args) {
                        let (arg, arg_bounds) = arg.merge(other_arg, globe);

                        if !arg_bounds.check_bindings(&arg.bindings) {
                            valid = false;
                            break
                        }

                        new_args.push(arg);
                        bounds.merge(arg_bounds, globe);
                    }
                    
                    if valid {
                        Some((Type::Cons(*id, new_args), bounds))
                    } else { None }
                } else {
                    None
                }
            },

            (
                Self { bindings, t: Type::Var(id) },
                Self { bindings: other_bindings, t: Type::Var(other_id) }
            ) => {
                if id == other_id {
                    assert!(!bindings.contains(id));
                    assert!(!other_bindings.contains(id));
                    Some((Type::Var(*id), Bounds::default()))
                } else {
                    let mut bounds = Bounds::default();
                    bounds.put_var_and_var(*id, *other_id, globe);
                    let mut bindings = bindings.clone();
                    bindings.append(&mut other_bindings.clone());
                    Some((Type::Var(*id), bounds))
                }
            },

            (Self { bindings, t: Type::Var(id) }, poly_type)
            | (poly_type, Self { bindings, t: Type::Var(id) }) => {
                let mut bounds = Bounds::default();
                bounds.put_var_and_poly_type(*id, poly_type.clone(), globe);
                Some((Self { bindings: bindings.clone(), t: Type::Var(*id) }, bounds))
            },
        };

        v
            .and_then(|(t, bounds)| {
                let mut bindings = self.bindings.clone();
                bindings.append(&mut other.bindings);
                if bounds.check_bindings(&bindings) {
                    Some((PolyType { bindings, t }, bounds))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                let id = globe.new_type_var();
                (PolyType { bindings: vec![id], t: Type::Var(id) }, Bounds::default())
            })
    }
}
