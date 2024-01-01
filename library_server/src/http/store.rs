use std::io::Write;

use axum::{Router, extract, Json, routing::get, http::{self, StatusCode, status}, response::IntoResponse, body::Body};
use shared::library::store::{package, PackageMetaError};
use tempfile::{tempfile, tempdir};
use tokio::{fs::File, io::AsyncReadExt};
use zip::ZipArchive;
use super::State;
use shared::PackageId;

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

#[axum::debug_handler]
pub async fn add_package(
    extract::Path(path): extract::Path<package::Path>,
    extract::State(state): extract::State<State>,
    request: extract::Request<Body>,
) -> Json<Result<PackageId, package::AddError>> {
    type E = package::AddError;
    Json((|| async {
        let mut file = tempfile().map_err(|_| E::Internal)?;
        while let Some(chunk) = request.body_mut() {
            file.write_all(&chunk).map_err(|_| E::Internal)?;
        }
        let dir = tempdir().map_err(|_| E::Internal)?;
        ZipArchive::new(file).map_err(|_| E::Internal)?.extract(dir.path()).map_err(|_| E::Internal)?;

        let meta: package::AddMeta = serde_json::from_reader(File::open(dir.path().join("meta.json")).await.map_err(|_| E::Internal)?).map_err(|_| E::Internal)?;
        let mut code = Vec::new();
        File::open(dir.path().join("meta.json")).await.map_err(|_| E::Internal)?.read_to_end(&mut code).await.map_err(|_| E::Internal)?;

        state.client_server.add_package(path, &meta, &code).await
    })().await)
}
