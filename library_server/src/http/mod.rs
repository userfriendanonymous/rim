use std::{sync::{Arc, RwLock}, path::PathBuf};
use axum::Router;
use shared::library::store::package::Path as PackagePath;
use tokio::net::TcpListener;
use crate::{store, ClientServer};

pub mod store;

#[derive(Clone)]
struct State {
    client_server: Arc<ClientServer>,
}

pub fn router() -> Router<State> {
    Router::new()
        .nest("/store", store::router())
}

async fn run(client_server: Arc<ClientServer>) {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
