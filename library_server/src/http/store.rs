use axum::{Router, extract, Json, routing::get, http::{self, StatusCode, status}, response::IntoResponse};
use shared::library::store::{package, PackageMetaError};
use super::State;


pub fn router() -> Router<State> {
    Router::new()
        .route("/package_meta/:path", get(package_meta))
        .route("/package_code/:path", get(package_code))
}

pub async fn package_meta(
    extract::Path(path): extract::Path<package::Path>,
    extract::State(state): extract::State<State>
) -> Json<Result<package::Meta, PackageMetaError>> {
    Json(state.client_server.package_meta(path).await)
}

#[axum::debug_handler]
pub async fn package_code(
    extract::Path(path): extract::Path<package::Path>,
    extract::State(state): extract::State<State>
) -> impl IntoResponse {
    match state.client_server.package_code(path).await {
        Ok(data) => (
            StatusCode::OK,
            data
        ),
        Err(_error) => (
            StatusCode::FORBIDDEN,
            Vec::new()
        )
    }
}
