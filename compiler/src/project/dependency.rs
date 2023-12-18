
use std::{collections::BTreeMap, io::Write};
use crate::built_in_module;
use super::{LibraryServer, PackagesMap, packages_map};
use tempfile::NamedTempFile;
use bytes::Bytes;
use shared::{PackageId, Ident, library::store::{PackageMetaError as LibraryPackageMetaError, dependency::Value}};
use tokio::{fs::{File, OpenOptions}, io::{self, AsyncWriteExt, AsyncReadExt}};
use zip::{ZipArchive, result::ZipError};
use crate::syntax::Value as Syntax;
use async_recursion::async_recursion;

#[derive(Clone, Debug)]
pub struct Resolved {
    pub dependencies: BTreeMap<Ident, Value>,
    pub syntax: Syntax,
    pub id: PackageId,
}

pub async fn resolve(value: Value, library_server: &LibraryServer) -> Result<Resolved, ResolveError> {
    type E = ResolveError;
    match value {
        Value::Library(path) => {
            let meta = library_server.package_meta(path.clone()).await.map_err(E::Http)?.map_err(E::LibraryPackageMeta)?;
            let mut code: Bytes = library_server.package_code(path).await.map_err(E::Http)?;
            
            let mut src_file = NamedTempFile::new().map_err(E::StdIo)?;
            src_file.write_all(&mut code).map_err(E::StdIo)?;
            ZipArchive::new(src_file).map_err(E::Zip)?
                .extract("src").map_err(E::Zip)?;

            let syntax = super::file_module::Ptr::new("src".into(), "main".into())
                .resolve().await.map_err(E::Syntax)?;
            
            Ok(Resolved {
                dependencies: meta.dependencies,
                syntax,
                id: meta.id
            })
        },
        Value::BuiltIn => Ok(Resolved {
            dependencies: Default::default(),
            syntax: built_in_module::create(),
            id: PackageId::zero()
        })
    }
}

#[derive(Debug)]
pub enum ResolveError {
    Http(reqwest::Error),
    LibraryPackageMeta(LibraryPackageMetaError),
    Zip(ZipError),
    Io(io::Error),
    Syntax(super::file_module::ResolveError),
    StdIo(std::io::Error)
}

#[derive(Debug)]
pub enum ResolveMapError {
    Single(ResolveError),
}

#[async_recursion]
pub async fn resolve_many(values: BTreeMap<Ident, Value>, library_server: &LibraryServer)
-> Result<(BTreeMap<Ident, PackageId>, PackagesMap), ResolveMapError> {
    type E = ResolveMapError;
    let mut dependencies = BTreeMap::new();
    let mut packages_map = PackagesMap::default();
    for (name, value) in values {
        let child = resolve(value, library_server).await.map_err(E::Single)?;
        dependencies.insert(name, child.id);
        let (child_dependencies, mut child_packages) = resolve_many(child.dependencies, library_server).await?;
        packages_map.append(&mut child_packages);
        packages_map.insert(child.id, packages_map::Item { syntax: child.syntax, dependencies: child_dependencies });
    }

    Ok((dependencies, packages_map))
}