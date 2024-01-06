
use std::collections::BTreeMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tokio::io;
pub use crate::library::store::package::Path;
use crate::library::store::{package as sharedSelf, Dependency};
use crate::{PackageId as Id, Ident};

#[derive(Debug)]
pub enum CodeError {
    Io(io::Error),
}

impl Into<sharedSelf::CodeError> for CodeError {
    fn into(self) -> sharedSelf::CodeError {
        match self {
            _ => sharedSelf::CodeError::Internal
        }
    }
}

#[derive(Debug)]
pub enum MetaError {
    Io(io::Error),
    Decoding(serde_json::Error)
}

impl Into<sharedSelf::MetaError> for MetaError {
    fn into(self) -> sharedSelf::MetaError {
        match self {
            _ => sharedSelf::MetaError::Internal
        }
    }
}

#[derive(Debug)]
pub enum AddError {
    PathExists,
    Io(io::Error),
    NewPackageId(super::package_id::NewError)
}

impl Into<sharedSelf::AddError> for AddError {
    fn into(self) -> sharedSelf::AddError {
        match self {
            Self::PathExists => sharedSelf::AddError::PathExists,
            _ => sharedSelf::AddError::Internal
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub dependencies: BTreeMap<Ident, Dependency>,
    pub id: Id,
}

impl super::Pointer {
    fn package_path(&self, path: Path) -> PathBuf {
        self.path.join("packages").join(path.to_string())
    }

    pub async fn add_package(&mut self, path: Path, meta: &sharedSelf::AddMeta, code: &[u8]) -> Result<Id, AddError> {
        type E = AddError;
        let path_to_package = self.package_path(path);
        if path_to_package.exists() {
            Err(E::PathExists)
        } else {
            let id = self.new_package_id().await.map_err(E::NewPackageId)?;
            let meta = Meta {
                dependencies: meta.dependencies,
                id,
            };
            create_dir_all(path_to_package.clone()).await.map_err(E::Io)?;
            File::create(path_to_package.join("meta.json")).await.map_err(E::Io)?
                .write_all(serde_json::to_string(&meta).unwrap().as_bytes()).await.map_err(E::Io)?;
            File::create(path_to_package.join("code.zip")).await.map_err(E::Io)?
                .write_all(code).await.map_err(E::Io)?;
            Ok(id)
        }

    }

    pub async fn package_code(&self, path: Path) -> Result<Vec<u8>, CodeError> {
        type E = CodeError;
        let mut content = Vec::new();
        let mut file = File::open(self.package_path(path).join("code.zip")).await.map_err(E::Io)?;
        file.read_to_end(&mut content).await.map_err(E::Io)?;
        Ok(content)
    }

    pub async fn package_meta(&self, path: Path) -> Result<sharedSelf::Meta, MetaError> {
        type E = MetaError;
        let mut file = File::open(self.package_path(path).join("meta.json")).await.map_err(E::Io)?;
        let mut content = String::new();
        file.read_to_string(&mut content).await.map_err(E::Io)?;
        serde_json::from_str(&content).map_err(E::Decoding)
    }
}