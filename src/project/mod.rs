use std::path::{Path, PathBuf};
use crate::{syntax::{Ident, Value as Syntax, self, module::Module}, parsing};
use async_recursion::async_recursion;
use chumsky::Parser;
use tokio::{fs::{File, read_to_string}, io};
use kdl::{KdlDocument, KdlError};

pub async fn resolve(path: PathBuf) -> Result<Syntax, ResolveFileModuleError> {
    type E = ResolveFileModuleError;
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
            "package" => ,
            "builtin" => 
        }
    }
    Ok(())
}

pub enum ConfigError {
    ImportsNotFound,
    ImportsChildrenNotFound,
    ImportTypeNotFound,
}

pub enum ResolveFileModuleError {
    ReadFile(io::Error),
    Parsing(Vec<chumsky::error::Simple<char>>),
    Child(Ident, Box<Self>),
    ParseConfig(KdlError),
    Config(ConfigError)
}

#[derive(Clone)]
pub struct FileModule {
    path: PathBuf,
    name: Ident,
}

impl FileModule {
    pub fn file_path(&self) -> PathBuf {
        self.path.join(self.name.to_string() + ".rim")
    }

    pub fn child(&self, name: Ident) -> Self {
        Self {
            path: self.path.join(self.name.to_string()),
            name
        }
    }
}

pub async fn resolve_file_module(ptr: FileModule) -> Result<Syntax, ResolveFileModuleError> {
    type E = ResolveFileModuleError;
    let content = read_to_string(ptr.file_path()).await.map_err(E::ReadFile)?;
    let syntax = parsing::value(Default::default()).parse(content).map_err(E::Parsing)?;
    resolve_syntax(syntax, ptr).await
}

#[async_recursion]
pub async fn resolve_syntax(syntax: Syntax, ptr: FileModule) -> Result<Syntax, ResolveFileModuleError> {
    type Item = syntax::module::Item;
    type E = ResolveFileModuleError;
    let mut output_items = Vec::new();
    for item in syntax {
        output_items.push(match item {
            Item::Module(name, v) => Item::Module(name, match v {
                syntax::module::Module::File(file_name) => Module::Where(
                    resolve_file_module(ptr.clone().child(file_name.clone()))
                        .await
                        .map_err(|e| E::Child(file_name, Box::new(e)))?
                ),
                v => v
            }),
            Item::From(path, syntax) => Item::From(path, resolve_syntax(syntax, ptr.clone()).await?),
            Item::LetIn(input, output) => Item::LetIn(resolve_syntax(input, ptr.clone()).await?, resolve_syntax(output, ptr.clone()).await?),
            Item::Val(name, value) => Item::Val(name, resolve_val(value, ptr.clone()).await?),
            v => v
        })
    }
    Ok(output_items)
}

#[async_recursion]
pub async fn resolve_val(value: syntax::Val, ptr: FileModule) -> Result<syntax::Val, ResolveFileModuleError> {
    use syntax::Val;
    Ok(match value {
        Val::Apply(f, input) => Val::Apply(Box::new(resolve_val(*f, ptr.clone()).await?), Box::new(resolve_val(*input, ptr.clone()).await?)),
        Val::Function(f) => Val::Function(Box::new(syntax::Function { input: f.input, output: resolve_val(f.output, ptr.clone()).await? })),
        Val::If(cond, then, otherwise) => Val::If(
            Box::new(resolve_val(*cond, ptr.clone()).await?),
            Box::new(resolve_val(*then, ptr.clone()).await?),
            Box::new(resolve_val(*otherwise, ptr.clone()).await?)
        ),
        Val::InfixApply(f, left, right) => Val::InfixApply(
            Box::new(resolve_val(*f, ptr.clone()).await?),
            if let Some(left) = left { Some(Box::new(resolve_val(*left, ptr.clone()).await?)) } else { None },
            if let Some(right) = right { Some(Box::new(resolve_val(*right, ptr.clone()).await?)) } else { None }
        ),
        Val::LetIn(input, output) => Val::LetIn(resolve_syntax(input, ptr.clone()).await?, Box::new(resolve_val(*output, ptr.clone()).await?)),
        v => v,
    })
}