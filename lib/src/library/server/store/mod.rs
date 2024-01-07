use std::path::PathBuf;
use tokio::sync::RwLock;

mod package;
mod directory;
mod package_id;

pub type Lock = RwLock<Pointer>;

pub struct Pointer {
    path: PathBuf,
    // package_id_lock: RwLock<()>,
}

impl Pointer {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            // package_id_lock: RwLock::new(())
        }
    }
}