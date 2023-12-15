use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use compiler::project;
use fs_utils::create_string_file;
use tokio::{fs::{File, create_dir_all}, io};

mod fs_utils;

const CONFIG: &'static str = r#"
{
    "dependencies": {
        "builtIn": { "BuiltIn": null }
    },
    "targets": {
        "js": ["jsMain"]
    }
}
"#;

const MAIN_MODULE: &'static str = r#"
let
    mod console = builtIn.js.effect.console
in
    val jsMain = console.log "Hello world!"
"#;

#[derive(Parser, Debug)]
pub enum Args {
    Init,
    New {
        at: PathBuf
    },
    Build,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args {
        Args::Init => {
            init_project(".".into()).await.unwrap();
        },
        Args::New { at } => {
            init_project(at).await.unwrap();
        },
        Args::Build => {
            let library_server = Arc::new(project::LibraryServer::new());
            project::Pointer::new(".".into(), "".into(), library_server)
                .compile().await.unwrap();
        }
    }
}

async fn init_project(path: PathBuf) -> Result<(), io::Error> {
    create_string_file(path.join("config.json"), CONFIG.into()).await?;
    create_string_file(path.join("main.rim"), MAIN_MODULE.into()).await?;
    create_dir_all(path.join("main")).await?;
    Ok(())
}
