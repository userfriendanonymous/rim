#![feature(trait_alias)]

pub use ident::Value as Ident;
pub use package_id::Value as PackageId;

pub mod compiler;
pub mod ident;
pub mod fs_utils;
pub mod tokio_fs_utils;
pub mod library;
pub mod package_id;
pub mod project;
pub mod cli;