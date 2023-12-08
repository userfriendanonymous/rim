use axum::{Router, extract, Json, routing::get, http::{self, StatusCode}};
use shared::library::store::{package, PackageMetaError};
use super::State;


pub fn router() -> Router<State> {
    Router::new()
        .route("/package_meta", get(package_meta))
        .route("/package_code", get(package_code))
}

pub async fn package_meta(
    extract::Path(path): extract::Path<package::Path>,
    extract::State(state): extract::State<State>
) -> Json<Result<package::Meta, PackageMetaError>> {
    Json(state.client_server.package_meta(path))
}

pub async fn package_code(
    extract::Path(path): extract::Path<package::Path>,
    extract::State(state): extract::State<State>
) -> http::Response<Vec<u8>> {
    match state.client_server.package_code(path) {
        Ok(data) => http::Response::new(data),
        Err(error) => http::Response::builder().status(StatusCode::BAD_REQUEST).body(Vec::new()).unwrap()
    }
}
