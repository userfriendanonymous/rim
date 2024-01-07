use std::collections::BTreeSet;

use super::{PolyType, VarId};

#[derive(Default)]
pub struct Value {
    rows: Vec<(BTreeSet<VarId>, PolyType)>
}

impl Value {
    pub fn get(&self, id: &VarId) -> Option<&PolyType> {
        self.row_by_var_id(id).and_then(|v| v.1.as_ref())
    }

    pub fn remove(&mut self, id: &VarId) -> Option<PolyType> {
        self.row_idx_by_var_id(&id).map(|idx| self.rows.remove(idx).1)
    }
    
    pub fn check_bindings(&self, bindings: &[VarId]) -> bool {
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

    pub fn merge(&mut self, other: Self) {
        for row in other.rows {
            self.put_row(row);
        }
    }

    pub fn put_row(&mut self, row: (BTreeSet<VarId>, Option<PolyType>)) {
        self.put_vars(row.0);
        if let Some(poly_type) = row.1 {
            if let Some(id) = row.0.first() {
                self.put_var_and_poly_type(*id, poly_type);
            }
        }
    }

    pub fn put_vars(&mut self, vars: BTreeSet<VarId>) {
        let mut prev_var = None;
        for var in vars {
            if let Some(prev_var) = prev_var {
                self.put_var_and_var(var, prev_var);
            }
            prev_var = Some(var);
        }
    }

    pub fn put_var_and_var(&mut self, id: VarId, other_id: VarId) {
        match (self.row_idx_by_type_id(&id), self.row_idx_by_type_id(&other_id)) {
            (Some(row_idx), Some(other_row_idx)) => {
                if row_idx != other_row_idx {
                    let row = self.row_mut(row_idx);
                    let other_row = self.row_mut(other_row_idx);
                    row.0.append(&mut other_row.0);
                    match (&mut row.1, &mut other_row.1) {
                        (Some(poly_type), Some(other_poly_type)) => {
                            let (new_poly_type, bounds) = poly_type.merge(&other_poly_type);
                            *poly_type = new_poly_type;
                            self.merge(bounds);
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

    pub fn put_var_and_poly_type(&mut self, id: VarId, poly_type: PolyType) {
        if let Some((_, other_poly_type)) = self.row_by_type_id_mut(&id) {
            if let Some(other_poly_type) = other_poly_type {
                let (new_poly_type, bounds) = other_poly_type.clone().merge(&poly_type);
                *other_poly_type = new_poly_type;
                self.merge(bounds);
            } else {
                *other_poly_type = Some(poly_type);
            }
        } else {
            self.push_row(id, poly_type);
        }
    }

    pub fn push_row(&mut self, id: VarId, poly_type: PolyType) {
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

    fn row_idx_by_var_id(&self, id: &VarId) -> Option<usize> {
        let mut row = None;
        for (idx, item) in self.rows.iter().enumerate() {
            if item.0.contains(id) {
                row = Some(idx);
                break;
            }
        }
        row
    }

    fn row_by_var_id(&self, id: &VarId) -> Option<&(BTreeSet<VarId>, Option<PolyType>)> {
        Some(self.rows.get(self.row_idx_by_var_id(id)?).unwrap())
    }

    fn row_by_var_id_mut(&self, id: &VarId) -> Option<&mut (BTreeSet<VarId>, Option<PolyType>)> {
        Some(self.rows.get_mut(self.row_idx_by_var_id(id)?).unwrap())
    }

    fn row_mut(&mut self, idx: usize) -> &mut (BTreeSet<VarId>, Option<PolyType>) {
        self.rows.get_mut(idx).unwrap()
    }

    fn row(&self, idx: usize) -> &(BTreeSet<VarId>, Option<PolyType>) {
        self.rows.get(idx).unwrap()
    }
}
