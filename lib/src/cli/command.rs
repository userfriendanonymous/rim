use std::{path::PathBuf, sync::Arc, process::Command, io::stdout};

use clap::Parser;
use colored::{ColoredString, Colorize};
use crate::{project, library};
use super::fs_utils::create_string_file;
use tokio::{fs::{File, create_dir_all, create_dir, read_dir}, io};

const CONFIG: &'static str = r#"
{
    "dependencies": {
        "builtIn": { "BuiltIn": null }
    },
    "targets": {
        "js": {
            "node": {
                "main": ["nodeMain", "Lazy"]
            },
            "browser": {
                
            }
        }
    }
}
"#;

const MAIN_MODULE: &'static str = r#"
let mod
    console = builtIn.js.console
    node = builtIn.js.node
in
    val nodeMain = node.fromSuper $ console.log "Hello world!"
"#;

#[derive(Parser, Debug)]
pub enum Value {
    Init,
    New {
        at: PathBuf
    },
    Build,
    RunJs {
        name: String,
    },
}

impl Value {
    pub async fn run(self) {
        match self {
            Value::Init => {
                match init().await {
                    Ok(_) => {
                        println!("{}", "Success!".green());
                    },
                    Err(error) => match error {
                        InitError::Io(error) => println!("{} {error}", "IO Error:".red()),
                        InitError::DirectoryNotEmpty => println!("{} directory must be empty!", "Error:".red())
                    }
                }
            },
            Value::New { at } => {
                if let Err(error) = create_dir(at.clone()).await {
                    println!("{} {error}", "Error, failed to create the directory:".red());
                } else if let Err(error) = init_project(at).await {
                    println!("{} {error}", "IO Error:".red());
                } else {
                    println!("{}", "Success!".green());
                }
            },
            Value::Build => {
                println!("{}", "Compiling...".blue());
                let library_client = Arc::new(library::HttpClient::new());
                if let Err(error) = project::Pointer::new(".".into(), "".into(), library_client).compile().await {
                    println!("{} {error:?}", "Error while compiling:".red());
                }
                println!("{} {}", "Success!".green(), "Compiled programs are in `output/` directory.");
            },
            Value::RunJs { name } => {
                println!("{}", "Compiling...".blue());
                let library_client = Arc::new(library::HttpClient::new());
                if let Err(error) = project::Pointer::new(".".into(), "".into(), library_client).compile().await {
                    println!("{} {error:?}", "Error while compiling:".red());
                } else {
                    println!("{}", "Success! Running:".green());
                    if let Err(error) = Command::new("node")
                        .arg(format!("./output/js/node/{name}.js"))
                        .spawn()
                        .unwrap()
                        .wait()
                    {
                        println!("{} {error}", "Error while running:".red());
                    }
                }
            }
        }
    }
}

async fn init_project(path: PathBuf) -> Result<(), io::Error> {
    create_string_file(path.join("config.json"), CONFIG.into()).await?;
    create_string_file(path.join("main.rim"), MAIN_MODULE.into()).await?;
    create_dir_all(path.join("main")).await?;
    Ok(())
}

pub enum InitError {
    Io(io::Error),
    DirectoryNotEmpty,
}

async fn init() -> Result<(), InitError> {
    type E = InitError;
    if read_dir(".").await.map_err(E::Io)?.next_entry().await.map_err(E::Io)?.is_some() {
        Err(E::DirectoryNotEmpty)?
    }

    init_project(".".into()).await.map_err(E::Io)?;
    Ok(())
}

// ../target/debug/cli run-js jsMain