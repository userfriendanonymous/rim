use std::sync::Arc;

use crate::store::Lock as StoreLock;


mod store;

pub struct Value {
    store: Arc<StoreLock>,
}
