#![feature(trait_alias)]

use std::{fs::File, io::{Write, Read}, collections::BTreeMap, path::PathBuf, sync::Arc};
use ariadne::{Label, Source};
use chumsky::Parser;

pub mod parsing;
pub mod syntax;
pub mod target;
pub mod resolution;
pub mod project;
pub mod built_in_module;

#[tokio::test]
async fn main() {
    let library_server = Arc::new(project::LibraryServer::new());

    let project_ptr = project::Pointer::new(
        "example".into(),
        "".into(),
        library_server.clone()
    );

    project_ptr.compile().await.unwrap();
}