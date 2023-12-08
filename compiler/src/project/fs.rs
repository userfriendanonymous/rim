use std::path::PathBuf;
use chumsky::Parser;
use shared::Ident;
use async_recursion::async_recursion;
use tokio::{io, fs};
use crate::{syntax::{Value as Syntax, self, module::Module}, parsing};

#[derive(Clone)]
pub struct FileModule {
    path: PathBuf,
    name: Ident,
}

impl FileModule {
    pub fn new(path: PathBuf, name: Ident) -> Self {
        Self {
            path,
            name,
        }
    }

    fn file_path(&self) -> PathBuf {
        self.path.join(self.name.to_string() + ".rim")
    }

    fn child(&self, name: Ident) -> Self {
        Self {
            path: self.path.join(self.name.to_string()),
            name
        }
    }

    pub async fn resolve(self) -> Result<Syntax, ResolveError> {
        type E = ResolveError;
        let content = fs::read_to_string(self.file_path()).await.map_err(E::Io)?;
        let syntax = parsing::value(Default::default()).parse(content).map_err(E::Parsing)?;
        self.resolve_module_where(syntax).await
    }

    #[async_recursion]
    async fn resolve_module_where(self, items: Syntax) -> Result<Syntax, ResolveError> {
        type Item = syntax::module::Item;
        type E = ResolveError;
        let mut output_items = Vec::new();
        for item in items {
            output_items.push(match item {
                Item::Module(name, v) => Item::Module(name, match v {
                    syntax::module::Module::File(file_name) => Module::Where(
                        self.clone().child(file_name.clone()).resolve()
                            .await
                            .map_err(|e| E::Child(file_name, Box::new(e)))?
                    ),
                    v => v
                }),
                Item::From(path, syntax) => Item::From(path, self.clone().resolve_module_where(syntax).await?),
                Item::LetIn(input, output) => Item::LetIn(self.clone().resolve_module_where(input).await?, self.clone().resolve_module_where(output).await?),
                Item::Val(name, value) => Item::Val(name, self.clone().resolve_val(value).await?),
                v => v
            })
        }
        Ok(output_items)
    }

    #[async_recursion]
    async fn resolve_val(self, value: syntax::Val) -> Result<syntax::Val, ResolveError> {
        use syntax::Val;
        Ok(match value {
            Val::Apply(f, input) => Val::Apply(Box::new(self.clone().resolve_val(*f).await?), Box::new(self.clone().resolve_val(*input).await?)),
            Val::Function(f) => Val::Function(Box::new(syntax::Function { input: f.input, output: self.clone().resolve_val(f.output).await? })),
            Val::If(cond, then, otherwise) => Val::If(
                Box::new(self.clone().resolve_val(*cond).await?),
                Box::new(self.clone().resolve_val(*then).await?),
                Box::new(self.clone().resolve_val(*otherwise).await?)
            ),
            Val::InfixApply(f, left, right) => Val::InfixApply(
                Box::new(self.clone().resolve_val(*f).await?),
                if let Some(left) = left { Some(Box::new(self.clone().resolve_val(*left).await?)) } else { None },
                if let Some(right) = right { Some(Box::new(self.clone().resolve_val(*right).await?)) } else { None }
            ),
            Val::LetIn(input, output) => Val::LetIn(self.resolve_module_where(input).await?, Box::new(self.clone().resolve_val(*output).await?)),
            v => v,
        })
    }
}

pub enum ResolveError {
    Io(io::Error),
    Parsing(Vec<chumsky::error::Simple<char>>),
    Child(Ident, Box<Self>),
}
