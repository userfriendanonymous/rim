
use std::{collections::{BTreeMap, BTreeSet}, iter::zip};
use super::{Id, Val};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValueId(Id);

type TypeVarId = u64;

pub struct Value {
    vals: BTreeMap<ValueId, (Val, ValType)>,
    type_vars: TypeVarId
}

impl Value {
    pub fn new_type_var(&mut self) -> TypeVarId {
        let id = self.type_vars;
        self.type_vars += 1;
        id
    }
}

#[derive(Clone)]
pub enum Type {
    Cons(Id, Vec<PolyType>),
    Var(TypeVarId)
}

impl Type {
    pub fn replace_var(&mut self, id: TypeVarId, with: Type) {
        match self {
            Self::Var(id) => *self = with,
            Self::Cons(_, args) => {
                for arg in args {
                    arg.t.replace_var(id, with.clone());
                }
            }
        }
    }

    pub fn flow(&self, other: &Self) -> Result<Bounds, ()> {
        match (self, other) {
            (Type::Cons(id, args), Type::Cons(other_id, other_args)) => {
                if id == other_id {
                    let mut bounds = Bounds::default();
                    for (arg, other_arg) in zip(args, other_args) {
                        let arg_bounds = arg.flow(&other_arg)?;
                        bounds = bounds.merge(arg_bounds);
                    }
                    Ok(bounds)
                } else {
                    Err(())
                }
            },
            (Type::Var(id), other) => {
                let mut bounds = Bounds::default();
                bounds.put(Type::Var(*id), other.clone());
                Ok(bounds)
            },
            (self_, Type::Var(other_id)) => {
                let mut bounds = Bounds::default();
                bounds.put(Type::Var(*other_id), self_.clone());
                Ok(bounds)
            }
        }
    }

    pub fn merge(&self, other: &Self, globe: &mut Value) -> Option<(Type, Bounds)> {
        match (self, other) {
            (Type::Cons(id, args), Type::Cons(other_id, other_args)) => {
                if id == other_id {
                    let mut new_args = Vec::new();
                    let mut bounds = Bounds::default();

                    for (arg, other_arg) in zip(args, other_args) {
                        let (arg, arg_bounds) = arg.merge(other_arg, globe);

                        if !arg_bounds.check_bindings(&arg.bindings) {
                            return None
                        }

                        new_args.push(arg);
                        bounds = bounds.merge(arg_bounds);
                    }
                    
                    Some((Type::Cons(*id, new_args), bounds))
                } else {
                    None
                }
            },

            (Type::Var(id), Type::Var(other_id)) => {
                if id == other_id {
                    // assert!(!bindings.contains(id));
                    // assert!(!other_bindings.contains(id));
                    Some((Type::Var(*id), Bounds::default()))
                } else {
                    let mut bounds = Bounds::default();
                    bounds.put_var_and_var(*id, *other_id, globe);
                    Some((Type::Var(*id), bounds))
                }
            },

            (Type::Var(id), poly_type)
            | (poly_type, Type::Var(id)) => {
                let mut bounds = Bounds::default();
                bounds.put_var_and_poly_type(*id, poly_type.clone(), globe);
                Some((Type::Var(*id), bounds))
            },
        };
    }
}

#[derive(Clone)]
pub struct PolyType {
    bindings: Vec<TypeVarId>,
    t: Type
}

impl PolyType {
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

pub struct PolyCons {
    bindings: Vec<TypeVarId>,
    value: (Id, Vec<PolyType>),
}

#[derive(Default)]
pub struct Bounds {
    rows: Vec<(BTreeSet<TypeVarId>, Option<PolyCons>)>
}

impl Bounds {
    pub fn check_bindings(&self, bindings: &[TypeVarId]) -> bool {
        for id in bindings {
            if let Some((ids, poly_type)) = self.row_by_type_id(id) {
                if poly_type.is_some() {
                    return false
                }
                for id in ids {
                    if !bindings.contains(id) {
                        return false
                    }
                }
            }
        }
        true
    }

    pub fn merge(&mut self, other: Self, globe: &mut Value) {
        for row in other.rows {
            self.put_row(row, globe);
        }
    }

    pub fn put_row(&mut self, row: (BTreeSet<TypeVarId>, Option<PolyType>), globe: &mut Value) {
        self.put_vars(row.0, globe);
        if let Some(poly_type) = row.1 {
            if let Some(id) = row.0.first() {
                self.put_var_and_poly_type(*id, poly_type, globe);
            }
        }
    }

    pub fn put_vars(&mut self, vars: BTreeSet<TypeVarId>, globe: &mut Value) {
        let mut prev_var = None;
        for var in vars {
            if let Some(prev_var) = prev_var {
                self.put_var_and_var(var, prev_var, globe);
            }
            prev_var = Some(var);
        }
    }

    pub fn put_var_and_var(&mut self, id: TypeVarId, other_id: TypeVarId, globe: &mut Value) {
        match (self.row_idx_by_type_id(&id), self.row_idx_by_type_id(&other_id)) {
            (Some(row_idx), Some(other_row_idx)) => {
                if row_idx != other_row_idx {
                    let row = self.row_mut(row_idx);
                    let other_row = self.row_mut(other_row_idx);
                    row.0.append(&mut other_row.0);
                    match (&mut row.1, &mut other_row.1) {
                        (Some(poly_type), Some(other_poly_type)) => {
                            let (new_poly_type, bounds) = poly_type.merge(&other_poly_type, globe);
                            *poly_type = new_poly_type;
                            self.merge(bounds, globe);
                        },
                        _ => {}
                    }
                }
            },
            (Some(row_idx), None) => {
                self.row_mut(row_idx).0.insert(other_id);
            },
            (None, Some(other_row_idx)) => {
                self.row_mut(other_row_idx).0.insert(id);
            },
            (None, None) => {
                let mut keys = BTreeSet::new();
                keys.insert(id);
                keys.insert(other_id);
                self.rows.push((keys, None));
            }
        }
    }

    pub fn put_var_and_poly_type(&mut self, id: TypeVarId, poly_type: PolyType, globe: &mut Value) {
        if let Some((_, other_poly_type)) = self.row_by_type_id_mut(&id) {
            if let Some(other_poly_type) = other_poly_type {
                let (new_poly_type, bounds) = other_poly_type.clone().merge(&poly_type, globe);
                *other_poly_type = new_poly_type;
                self.merge(bounds, globe);
            } else {
                *other_poly_type = Some(poly_type);
            }
        } else {
            self.push_row(id, poly_type);
        }
    }

    pub fn push_row(&mut self, id: TypeVarId, poly_type: PolyType) {
        let mut keys = BTreeSet::new();
        keys.insert(id);
        self.rows.push((keys, Some(poly_type)));
    }

    // pub fn put(&mut self, left: Type, right: Type) {
    //     match (left, right) {
    //         (Type::Cons(left_id, left_args), Type::Cons(right_id, right_args)) => {
    //             if left_id == right_id {
    //                 zip(left_args, right_args).map(|(left_arg, right_arg)| )
    //             } else {
    //                 Err(())
    //             }
    //         }
    //         (Self::Var(id), Self::Var(other_id)) => {
    //             if id == other_id {
    //                 Ok(())
    //             } else {
    //                 match (self.row_idx_by_type_id(&id), self.row_idx_by_type_id(&other_id)) {
    //                     (Some(idx), Some(other_idx)) => {
    //                         let (other_vars, other_poly_type) = self.row(other_idx).clone();
    //                         self.rows[idx].0.append(&mut other_vars);
    //                         self.rows[idx] = ;
    //                     }
    //                 }
    //                 let row = row.unwrap_or_else(|| {
    //                     self.rows.push((BTreeSet::new(), None));
    //                     self.rows.last_mut().unwrap()
    //                 });

    //                 row.0.insert(id);
    //                 row.0.insert(other_id);
    //             }
    //         }
    //         (Self::Var(id), _) => {

    //         }
            
    //     }
    // }

    fn row_idx_by_type_id(&self, id: &TypeVarId) -> Option<usize> {
        let mut row = None;
        for (idx, item) in self.rows.iter().enumerate() {
            if item.0.contains(id) {
                row = Some(idx);
                break;
            }
        }
        row
    }

    fn row_by_type_id(&self, id: &TypeVarId) -> Option<&(BTreeSet<TypeVarId>, Option<PolyType>)> {
        Some(self.rows.get(self.row_idx_by_type_id(id)?).unwrap())
    }

    fn row_by_type_id_mut(&self, id: &TypeVarId) -> Option<&mut (BTreeSet<TypeVarId>, Option<PolyType>)> {
        Some(self.rows.get_mut(self.row_idx_by_type_id(id)?).unwrap())
    }

    fn row_mut(&mut self, idx: usize) -> &mut (BTreeSet<TypeVarId>, Option<PolyType>) {
        self.rows.get_mut(idx).unwrap()
    }

    fn row(&self, idx: usize) -> &(BTreeSet<TypeVarId>, Option<PolyType>) {
        self.rows.get(idx).unwrap()
    }
}

pub enum ValType {
    // Input bounds
    Input(Vec<Type>),
    Output(Type)
}
