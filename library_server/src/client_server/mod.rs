use std::sync::Arc;
use crate::store::Lock as StoreLock;

mod store;

pub struct Value {
    store: Arc<StoreLock>,
}

impl Value {
    pub fn new(store: Arc<StoreLock>) -> Self {
        Self { store }
    }
}