use std::{path::PathBuf, fmt::Display};
use tokio::{fs::{File, self, create_dir_all}, io::{self, AsyncRead, AsyncWrite, AsyncWriteExt, AsyncReadExt}};
use crate::{Ident, PackagePath, DirectoryPath};
use tokio::sync::RwLock;
use zip::ZipWriter;
use shared::library::store::{directory, package};
use shared::library::store::{PackageMetaError as SharedPackageMetaError, AddPackageError as SharedAddPackageError};

pub enum PackageCodeError {
    Io(io::Error),
}

pub enum PackageMetaError {
    Io(io::Error),
    Decoding(serde_json::Error)
}

impl Into<SharedPackageMetaError> for PackageMetaError {
    fn into(self) -> SharedPackageMetaError {
        match self {
            _ => SharedPackageMetaError::Internal
        }
    }
}

pub enum DirectoryError {
    Io(io::Error),
    Decoding(serde_json::Error)
}

pub enum AddPackageError {
    PathExists,
    Io(io::Error)
}

impl Into<SharedAddPackageError> for AddPackageError {
    fn into(self) -> SharedAddPackageError {
        match self {
            Self::PathExists => SharedAddPackageError::PathExists,
            _ => SharedAddPackageError::Internal
        }
    }
}

pub enum AddDirectoryError {
    Io(io::Error),
    PathExists,
}

pub type Lock = RwLock<Pointer>;

pub struct Pointer {
    path: PathBuf,
}

impl Pointer {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
        }
    }

    fn directory_path(&self, path: directory::Path) -> PathBuf {
        self.path.join("directories").join(path)
    }

    fn package_path(&self, path: package::Path) -> PathBuf {
        self.path.join("packages").join(path.to_string())
    }

    pub async fn add_package(&mut self, path: package::Path, meta: &package::Meta, code: &[u8]) -> Result<(), AddPackageError> {
        type E = AddPackageError;
        let path_to_package = self.package_path(path);
        if path_to_package.exists() {
            Err(E::PathExists)
        } else {
            create_dir_all(path_to_package).await.map_err(E::Io)?;
            File::create(path_to_package.join("meta.json")).await.map_err(E::Io)?
                .write_all(serde_json::to_string(meta).unwrap().as_bytes()).await.map_err(E::Io)?;
            File::create(path_to_package.join("code.zip")).await.map_err(E::Io)?
                .write_all(code).await.map_err(E::Io)?;
            Ok(())
        }

    }

    pub async fn add_directory(&mut self, path: directory::Path, value: directory::Value) -> Result<(), AddDirectoryError> {
        type E = AddDirectoryError;
        let path_to_directory = self.directory_path(path);
        if path_to_directory.exists() {
            Err(E::PathExists)
        } else {
            create_dir_all(path_to_directory).await.map_err(E::Io)?;
            File::create(path_to_directory.join("meta.json")).await.map_err(E::Io)?
                .write_all(serde_json::to_string(meta).unwrap().as_bytes()).await.map_err(E::Io)?;
            Ok(())
        }
    }

    pub async fn package_code(&self, path: package::Path) -> Result<Vec<u8>, PackageCodeError> {
        type E = PackageCodeError;
        let mut content = Vec::new();
        let mut file = File::open(self.package_path(path).join("code.zip")).await.map_err(E::OpeningFile)?;
        file.read_to_end(&mut content).await.map_err(E::Io)?;
        Ok(content)
    }

    pub async fn package_meta(&self, path: package::Path) -> Result<package::Meta, PackageMetaError> {
        type E = PackageMetaError;
        let mut file = File::open(self.package_path(path).join("meta.json")).await.map_err(E::Io)?;
        let mut content = String::new();
        file.read_to_string(&mut content).await.map_err(E::Io)?;
        serde_json::from_str(&content).map_err(E::Decoding)
    }

    pub async fn directory_meta(&self, path: directory::Path) -> Result<directory::Meta, DirectoryError> {
        type E = DirectoryError;
        let mut file = File::open(self.directory_path(path).join("meta.json")).await.map_err(E::Io)?;
        let mut content = String::new();
        file.read_to_string(&mut content).await.map_err(E::Io)?;
        serde_json::from_str(&content).map_err(E::Decoding)
    }
}