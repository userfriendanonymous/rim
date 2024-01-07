use std::io::Write;
use axum::{Router, extract, Json, routing::get, http::StatusCode, response::IntoResponse, body::Body};
use crate::{PackageId, library::store::package, fs_utils::extract_zip, tokio_fs_utils};
use tempfile::{tempfile, tempdir};
use super::State;
use futures_util::StreamExt;

pub fn router() -> Router<State> {
    Router::new()
        .route("/package_meta/:path", get(package_meta))
        .route("/package_code/:path", get(package_code))
}

pub async fn package_meta(
    extract::Path(path): extract::Path<package::Path>,
    extract::State(state): extract::State<State>
) -> Json<Result<package::Meta, package::MetaError>> {
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

#[axum::debug_handler]
pub async fn add_package(
    extract::Path(path): extract::Path<package::Path>,
    extract::State(state): extract::State<State>,
    request: extract::Request<Body>,
) -> Json<Result<PackageId, package::AddError>> {
    type E = package::AddError;
    Json((|| async {
        let mut file = tempfile().map_err(|_| E::Internal)?;
        let mut body_stream = request.into_body().into_data_stream();
        while let Some(Ok(chunk)) = body_stream.next().await {
            file.write_all(&chunk).map_err(|_| E::Internal)?;
        }
        let dir = tempdir().map_err(|_| E::Internal)?;
        extract_zip(file, dir.path()).map_err(|_| E::Internal)?;

        let meta = tokio_fs_utils::read_json::<package::AddMeta>(dir.path().join("meta.json")).await.map_err(|_| E::Internal)?;
        let code = tokio_fs_utils::read_to_end(dir.path().join("code.zip")).await.map_err(|_| E::Internal)?;

        state.client_server.add_package(path, &meta, &code).await
    })().await)
}
