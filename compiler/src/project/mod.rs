use std::{path::{Path, PathBuf}, collections::BTreeMap, sync::Arc};
use crate::{syntax::{Ident, Value as Syntax, self, module::Module}, parsing, target, resolution};
use async_recursion::async_recursion;
use chumsky::Parser;
use tokio::{fs::{File, read_to_string, OpenOptions}, io::{self, AsyncWriteExt}};
use kdl::{KdlDocument, KdlError};
use shared::{PackageId, library::store::Dependency};
use config::Value as Config;
use packages_map::Value as PackagesMap;
pub use library_server::Value as LibraryServer;

mod packages_map;
mod dependency;
mod config;
pub mod library_server;
mod file_module;

pub struct Pointer {
    packages_cache_path: PathBuf,
    path: PathBuf,
    library_server: Arc<LibraryServer>,
}

impl Pointer {
    pub fn new(path: PathBuf, packages_cache_path: PathBuf, library_server: Arc<LibraryServer>) -> Self {
        Self {
            path,
            packages_cache_path,
            library_server,
        }
    }

    async fn config(&self) -> Result<Config, ConfigError> {
        type E = ConfigError;
        let config_str = read_to_string(self.path.join("config.json")).await.map_err(E::Io)?;
        let config = serde_json::from_str(&config_str).map_err(E::Deserialize)?;
        Ok(config)
    }

    pub async fn compile(&self) -> Result<(), CompileError> {
        type E = CompileError;
        use crate::syntax::module;

        let config = self.config().await.map_err(E::Config)?;
        let syntax = file_module::Ptr::new(self.path.clone(), "main".into())
            .resolve().await.map_err(E::FileModule)?;
        let (dependencies, packages_map) = dependency::resolve_many(config.dependencies, &self.library_server).await.unwrap();
        let map_item = packages_map::Item {
            dependencies,
            syntax
        };

        File::create("packages_map").await.unwrap().write_all(format!("{:#?}", &packages_map).as_bytes()).await.unwrap();
        
        let packages_syntax = packages_map.to_syntax();
        let syntax = vec![module::Item::LetIn(
            packages_syntax,
            map_item.to_syntax()
        )];

        File::create("syntax").await.unwrap().write_all(format!("{:#?}", &syntax).as_bytes()).await.unwrap();

        let mut globe = resolution::Globe::new();
        let env = resolution::value(&syntax, resolution::Env::default(), &mut globe).map_err(E::Resolution)?;

        let dir = self.path.join("output").join("js");
        tokio::fs::create_dir_all(dir.clone()).await.map_err(E::Io)?;

        for name in config.targets.js {
            let val_id = env.val_id(&name).ok_or(E::ValNotFound(name.clone()))?.clone();
            let string = target::Type::Js.compile(&env, &mut globe, val_id);
            let mut file = File::create(dir.join(format!("{name}.js"))).await.map_err(E::Io)?;
            file.write_all(string.as_bytes()).await.map_err(E::Io)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum ToSyntaxError {
    Resolve(file_module::ResolveError),
    Packages(PackagesError),
}

#[derive(Debug)]
pub enum PackagesError {
    Config(ConfigError),
    ResolveDependencies(dependency::ResolveMapError)
}

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Deserialize(serde_json::Error),
}

#[derive(Debug)]
pub enum CompileError {
    Config(ConfigError),
    ValNotFound(Ident),
    Io(io::Error),
    Resolution(resolution::module::Error),
    FileModule(file_module::ResolveError),
}