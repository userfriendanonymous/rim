
use std::path::PathBuf;
use crate::library::server::store::directory;
pub use crate::{Ident, library::store::directory::Path};
use serde::{Serialize, Deserialize};
use tokio::{io::{self, AsyncReadExt as _, AsyncWriteExt as _}, fs::{create_dir_all, File}};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Meta {
    packages: Vec<Ident>,
    directories: Vec<Ident>,
}

pub enum AddError {
    Io(io::Error),
    PathExists,
}

#[derive(Debug)]
pub enum MetaError {
    Io(io::Error),
    Decoding(serde_json::Error)
}

impl super::Pointer {
    fn directory_path(&self, path: Path) -> PathBuf {
        self.path.join("directories").join(path.to_string())
    }

    pub async fn add_directory(&mut self, path: Path) -> Result<(), directory::AddError> {
        type E = directory::AddError;
        let path_to_directory = self.directory_path(path);
        if path_to_directory.exists() {
            Err(E::PathExists)
        } else {
            create_dir_all(path_to_directory.clone()).await.map_err(E::Io)?;
            File::create(path_to_directory.join("meta.json")).await.map_err(E::Io)?
                .write_all(serde_json::to_string(
                    &directory::Meta::default()
                ).unwrap().as_bytes()).await.map_err(E::Io)?;
            Ok(())
        }
    }

    pub(super) async fn directory_meta(&self, path: Path) -> Result<Meta, MetaError> {
        type E = MetaError;
        let mut file = File::open(self.directory_path(path).join("meta.json")).await.map_err(E::Io)?;
        let mut content = String::new();
        file.read_to_string(&mut content).await.map_err(E::Io)?;
        serde_json::from_str(&content).map_err(E::Decoding)
    }
}