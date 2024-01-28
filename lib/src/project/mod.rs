use std::{path::PathBuf, sync::Arc};
use crate::{compiler::{syntax::Ident, target, resolution}, tokio_fs, fs_utils, library::{http_client, store::package::Version}, PackageId};
use futures_util::TryFutureExt;
use tokio::{fs::{File, read_to_string, create_dir_all}, io::{self, AsyncWriteExt}};
pub use config::Value as Config;
use packages_map::Value as PackagesMap;
pub use crate::library::HttpClient as LibraryClient;
use crate::compiler::syntax::Value as Syntax;

mod packages_map;
mod dependency;
pub mod config;
mod file_module;
mod build;

pub struct Pointer {
    store_path: PathBuf,
    path: PathBuf,
    library_client: Arc<LibraryClient>,
}

impl Pointer {
    pub fn new(path: PathBuf, store_path: PathBuf, library_client: Arc<LibraryClient>) -> Self {
        Self {
            path,
            store_path,
            library_client,
        }
    }

    async fn config(&self) -> Result<Config, ConfigError> {
        type E = ConfigError;
        let config_str = read_to_string(self.path.join("config.json")).await.map_err(E::Io)?;
        let config = serde_json::from_str(&config_str).map_err(E::Deserialize)?;
        Ok(config)
    }

    pub async fn publish(&self) -> Result<(PackageId, Version), PublishError> {
        type E = PublishError;

        let config = self.config().await.map_err(E::Config)?;
        let syntax = file_module::Ptr::new(self.path.clone(), "main".into())
            .resolve().await.map_err(E::FileModule)?;
        build_to(self.path.join("output"), config.clone(), syntax.clone(), self.library_client.clone()).await.map_err(E::BuildTo)?;

        let Some(family_path) = config.family.clone() else {
            Err(E::NoFamilyPath)?
        };
        
        let mut archiver = zip_archive::Archiver::new();
        archiver.set_destination(self.store_path.join("code"));
        archiver.push(self.path.clone());
        archiver.archive().map_err(E::Archive)?;
        let code = tokio_fs::read_to_end(self.store_path.join("code").join(self.path.with_extension("zip").file_name().unwrap())).await.map_err(E::Io)?;
        let (id, version) = self.library_client.add_package(family_path, config.clone(), code).await.map_err(E::AddPackage)?;
        let mut config = config.clone();
        config.version = Some(version);
        tokio_fs::create_json(self.path.join("config.json"), &config).await.map_err(E::Io)?;
        Ok((id, version))
    }

    pub async fn build(&self) -> Result<(), BuildError> {
        type E = BuildError;

        let config = self.config().await.map_err(E::Config)?;
        let syntax = file_module::Ptr::new(self.path.clone(), "main".into())
            .resolve().await.map_err(E::FileModule)?;
        build_to(self.path.join("output"), config, syntax, self.library_client.clone()).await.map_err(E::BuildTo)
    }
}

async fn build_to(path: PathBuf, config: Config, syntax: Syntax, library_client: Arc<LibraryClient>) -> Result<(), BuildToError> {
    type E = BuildToError;
    use crate::compiler::syntax::module;

    let (dependencies, packages_map) = dependency::resolve_many(config.dependencies, &library_client).await.map_err(E::DependencyResolution)?;
    let map_item = packages_map::Item {
        dependencies,
        syntax
    };

    // File::create("packages_map").await.unwrap().write_all(format!("{:#?}", &packages_map).as_bytes()).await.unwrap();
    
    let syntax = packages_map.to_syntax(map_item.to_syntax());

    // File::create("syntax").await.unwrap().write_all(format!("{:#?}", &syntax).as_bytes()).await.unwrap();

    let mut globe = resolution::Globe::new();
    let env = resolution::value(&syntax, resolution::Env::default(), &mut globe).map_err(E::Resolution)?;

    {
        let dir = path.join("js");
        {
            let dir = dir.join("browser");
            tokio::fs::create_dir_all(dir.clone()).await.map_err(E::Io)?;
            for (name, (path, evaluation)) in config.targets.js.browser {
                let val_id = env.val_id_by_path(&path, &globe).map_err(|_| E::ValNotFound(name.clone()))?.clone();
                let string = target::js::Type { environment: target::js::Environment::Browser, evaluation }
                    .compile(&env, &mut globe, val_id);
                let mut file = File::create(dir.join(format!("{name}.js"))).await.map_err(E::Io)?;
                file.write_all(string.as_bytes()).await.map_err(E::Io)?;
            }
        }
        {
            let dir = dir.join("node");
            tokio::fs::create_dir_all(dir.clone()).await.map_err(E::Io)?;
            for (name, (path, evaluation)) in config.targets.js.node {
                let val_id = env.val_id_by_path(&path, &globe).map_err(|_| E::ValNotFound(name.clone()))?.clone();
                let string = target::js::Type { environment: target::js::Environment::Browser, evaluation }
                    .compile(&env, &mut globe, val_id);
                let mut file = File::create(dir.join(format!("{name}.js"))).await.map_err(E::Io)?;
                file.write_all(string.as_bytes()).await.map_err(E::Io)?;
            }
        }
    }

    Ok(())
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
pub enum BuildToError {
    ValNotFound(Ident),
    Io(io::Error),
    Resolution(resolution::module::Error),
    DependencyResolution(dependency::ResolveMapError)
}

#[derive(Debug)]
pub enum BuildError {
    BuildTo(BuildToError),
    Config(ConfigError),
    FileModule(file_module::ResolveError),
}

#[derive(Debug)]
pub enum PublishError {
    BuildTo(BuildToError),
    Config(ConfigError),
    FileModule(file_module::ResolveError),
    NoFamilyPath,
    Io(io::Error),
    Archive(Box<dyn std::error::Error>),
    AddPackage(http_client::store::AddPackageError)
}