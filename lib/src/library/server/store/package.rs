
use std::collections::BTreeMap;
use std::path::PathBuf;
use futures_util::TryFutureExt;
use serde::{Serialize, Deserialize};
use tokio::fs::{create_dir_all, File};
use tokio::io::{self, AsyncReadExt as _, AsyncWriteExt as _};
use crate::library::store::{package as sharedSelf, Dependency, family};
use crate::{PackageId as Id, Ident, tokio_fs};
use super::super::super::store::package::Version;
use super::package_id;

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
    NewPackageId(super::package_id::NewError),
    ReadJson(tokio_fs::ReadJsonError)
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
    fn family_path(&self, path: family::Path) -> PathBuf {
        self.path.join("packages").join(path.to_string())
    }

    fn package_path(&self, path: family::Path, version: Version) -> PathBuf {
        self.family_path(path).join(version.to_string())
    }

    pub async fn add_package(&mut self, path: family::Path, meta: &sharedSelf::AddMeta, code: &[u8]) -> Result<(Id, Version), AddError> {
        type E = AddError;
        let path_to_family = self.family_path(path);
        let (version, family_meta) = if path_to_family.exists() {
            tokio_fs::read_json::<family::Meta>(path_to_family.join("meta.json")).await.map_err(E::ReadJson)?
        } else {
            family::Meta { next_version: Default::default() }
        }.next_version();

        let id = self.new_package_id().await.map_err(E::NewPackageId)?;
        let meta = Meta {
            dependencies: meta.dependencies.clone(),
            id,
        };
        create_dir_all(path_to_family.clone()).await.map_err(E::Io)?;
        tokio_fs::create_json(path_to_family.join("meta.json"), &family_meta).await.map_err(E::Io)?;
        let path_to_package = path_to_family.join(version.to_string());
        create_dir_all(path_to_package.clone()).await.map_err(E::Io)?;
        tokio_fs::create_json(path_to_package.join("meta.json"), &meta).await.map_err(E::Io)?;
        tokio_fs::create(path_to_package.join("code.zip"), code).await.map_err(E::Io)?;
        Ok((id, version))
    }

    pub async fn package_code(&self, path: family::Path, version: Version) -> Result<Vec<u8>, CodeError> {
        type E = CodeError;
        let mut content = Vec::new();
        let mut file = File::open(self.package_path(path, version).join("code.zip")).await.map_err(E::Io)?;
        file.read_to_end(&mut content).await.map_err(E::Io)?;
        Ok(content)
    }

    pub async fn package_meta(&self, path: family::Path, version: Version) -> Result<sharedSelf::Meta, MetaError> {
        type E = MetaError;
        let mut file = File::open(self.package_path(path, version).join("meta.json")).await.map_err(E::Io)?;
        let mut content = String::new();
        file.read_to_string(&mut content).await.map_err(E::Io)?;
        serde_json::from_str(&content).map_err(E::Decoding)
    }
}