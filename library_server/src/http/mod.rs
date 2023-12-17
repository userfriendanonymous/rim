use std::sync::Arc;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use crate::ClientServer;

mod store;

#[derive(Clone)]
struct State {
    client_server: Arc<ClientServer>,
}

fn router() -> Router<State> {
    Router::new()
        .nest("/store", store::router())
}

pub async fn run(client_server: Arc<ClientServer>) {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router()
            .layer(TraceLayer::new_for_http())
            .with_state(State {
                client_server
            })
            .into_make_service()
    ).await.unwrap();
}
