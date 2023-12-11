
use crate::built_in_module;

use super::{LibraryServer, Package};
use bytes::Bytes;
use shared::{PackageId, library::store::{PackageMetaError as LibraryPackageMetaError, dependency::Value}};
use tokio::{fs::File, io::{self, AsyncWriteExt}};
use zip::{ZipArchive, result::ZipError};

pub enum ToPackageError {
    Http(reqwest::Error),
    LibraryPackageMeta(LibraryPackageMetaError),
    Zip(ZipError),
    Io(io::Error),
    Syntax(super::fs::ResolveError),
}

pub async fn to_package(value: Value, library_server: &LibraryServer) -> Result<(super::Package, PackageId), ToPackageError> {
    type E = ToPackageError;
    match value {
        Value::Library(path) => {
            let meta = library_server.package_meta(path.clone()).await.map_err(E::Http)?.map_err(E::LibraryPackageMeta)?;
            let mut code: Bytes = library_server.package_code(path).await.map_err(E::Http)?;
            let mut zip_file = File::create("src.zip").await.map_err(E::Io)?;
            zip_file.write_all_buf(&mut code).await.map_err(E::Io)?;
            ZipArchive::new(zip_file.into_std().await).map_err(E::Zip)?
                .extract("src").map_err(E::Zip)?;

            let syntax = super::fs::FileModule::new("src".into(), "main".into())
                .resolve().await.map_err(E::Syntax)?;

            Ok((
                Package {
                    dependencies: meta.dependencies,
                    syntax
                },
                meta.id,
            ))
        },
        Value::Builtin => {
            Ok((
                Package {
                    dependencies: Vec::new(),
                    syntax: built_in_module::create()
                },
                PackageId::zero()
            ))
        }
    }
}
