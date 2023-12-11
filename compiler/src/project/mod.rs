use std::{path::{Path, PathBuf}, collections::BTreeMap, sync::Arc};
use crate::{syntax::{Ident, Value as Syntax, self, module::Module}, parsing, target};
use async_recursion::async_recursion;
use chumsky::Parser;
use tokio::{fs::{File, read_to_string}, io};
use kdl::{KdlDocument, KdlError};
use shared::{PackageId, library::store::Dependency};
use config::Value as Config;
use package::Value as Package;
use library_server::Value as LibraryServer;

mod package;
mod dependency;
mod config;
mod library_server;
mod fs;

pub type Dependencies = BTreeMap<Ident, Dependency>;

pub struct Packages(BTreeMap<PackageId, syntax::Value>);



pub struct Manager {
    packages_cache_path: PathBuf,
    path: PathBuf,
    library_server: Arc<LibraryServer>,
}

impl Manager {
    async fn packages(&self) -> Result<Packages, PackagesError> {
        type E = PackagesError;
        let config = self.config().await.map_err(E::Config)?;
        for (name, dependency) in config.dependencies {
            dependency::to_package(dependency, &self.library_server);
        }
        todo!()
    }

    async fn config(&self) -> Result<Config, ConfigError> {
        type E = ConfigError;
        let config_str = read_to_string(self.path.join("config.json")).await.map_err(E::Io)?;
        let config = serde_json::from_str(&config_str).map_err(E::Deserialize)?;
        Ok(config)
    }
}

pub enum PackagesError {
    Config(ConfigError)
}

pub enum ConfigError {
    Io(io::Error),
    Deserialize(serde_json::Error),
}

impl Value {
    pub async fn resolve(&self, path: PathBuf) -> Result<Syntax, ResolveError> {
        type E = ResolveError;
        let config_content = read_to_string(path.join("config.json")).await.map_err(E::Io)?;
        let config = serde_json::from_str::<Config>(&config_content).map_err(E::ParseConfig)?;
        config.dependencies
    }
}

pub async fn resolve(path: PathBuf) -> Result<Syntax, ResolveFileModuleError> {

    let config_content = read_to_string(path.join("config.kdl")).await.map_err(E::ReadFile)?;
    let config_doc = config_content.parse::<KdlDocument>().map_err(E::ParseConfig)?;
    let config = resolve_config(config_doc).map_err(E::Config)?;

    let ptr = FileModule { path, name: "src".into() };
    resolve_file_module(ptr).await
}

pub fn resolve_config(doc: KdlDocument) -> Result<(), ConfigError> {
    type E = ConfigError;
    let imports = doc.get("imports").ok_or(E::ImportsNotFound)?;
    let imports_nodes = imports.children().ok_or(E::ImportsChildrenNotFound)?.nodes();
    for node in imports_nodes {
        let name = node.name().to_string();
        let mut entries = node.entries().into_iter();
        match entries.next().ok_or(E::ImportTypeNotFound)?.value().as_string().ok_or(E::ImportTypeNotFound)? {
            "package" => {
                let name = entries.next().ok_or(E::ImportPackageNameMissing)?.value().as_string().ok_or(E::ImportPackageNameMissing)?;
                let version = entries.next().ok_or(E::ImportPackageVersionMissing)?.value().as_i64().ok_or(E::ImportPackageVersionMissing)?;
                Dependency::Package(name.into(), version as _)
            },
            "builtin" => {
                let version = entries.next().ok_or(E::ImportBuiltinVersionMissing)?.value().as_i64().ok_or(E::ImportBuiltinVersionMissing)?;
                Dependency::Builtin(version as _)
            }
        }
    }
    Ok(())
}
