
use std::sync::Arc;

use shared::{Ident, ident};
use client_server::Value as ClientServer;
use tokio::sync::RwLock;

mod http;
mod store;
mod client_server;

#[tokio::main]
async fn main() {
    let store_lock = Arc::new(RwLock::new(store::Pointer::new("store".into())));
    let client_server = Arc::new(ClientServer::new(store_lock.clone()));
    http::run(client_server).await;
}
