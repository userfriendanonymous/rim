use std::io::Write;
use axum::{Router, extract, Json, routing::{get, post}, http::StatusCode, response::IntoResponse, body::Body};
use zip::result::ZipError;
use crate::{PackageId, library::store::{package, family}, fs_utils::extract_zip, tokio_fs};
use tempfile::{tempfile, tempdir, NamedTempFile};
use super::State;
use futures_util::StreamExt;
use zip_extensions::zip_extract;

pub fn router() -> Router<State> {
    Router::new()
        .route("/package_meta/:path/:version", get(package_meta))
        .route("/package_code/:path/:version", get(package_code))
        .route("/add_package/:path", post(add_package))
}

pub async fn package_meta(
    extract::Path((path, version)): extract::Path<(family::Path, package::Version)>,
    extract::State(state): extract::State<State>
) -> Json<Result<package::Meta, package::MetaError>> {
    Json(state.client_server.package_meta(path, version).await)
}

#[axum::debug_handler]
pub async fn package_code(
    extract::Path((path, version)): extract::Path<(family::Path, package::Version)>,
    extract::State(state): extract::State<State>
) -> impl IntoResponse {
    match state.client_server.package_code(path, version).await {
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
    extract::Path(path): extract::Path<family::Path>,
    extract::State(state): extract::State<State>,
    request: extract::Request<Body>,
) -> Json<Result<(PackageId, package::Version), package::AddError>> {
    type E = package::AddError;
    Json((|| async {
        type E = InternalAddPackageError;
        let mut file = NamedTempFile::new().map_err(E::CreateTempFile)?;
        let mut body_stream = request.into_body().into_data_stream();
        while let Some(Ok(chunk)) = body_stream.next().await {
            file.write_all(&chunk).map_err(E::WriteToFile)?;
        }
        file.flush().map_err(E::FlushFile)?;
        let dir = tempdir().map_err(E::CreateTempDir)?;
        zip_extract(&file.path().to_owned(), &dir.path().to_owned()).map_err(E::ExtractZip)?;
        let meta = tokio_fs::read_json::<package::AddMeta>(dir.path().join("meta.json")).await.map_err(E::ReadMeta)?;
        let code = tokio_fs::read_to_end(dir.path().join("code.zip")).await.map_err(E::ReadCode)?;

        state.client_server.add_package(path, &meta, &code).await.map_err(E::AddPackage)
    })().await
        .inspect_err(|e| println!("Error adding package: {e:?}"))
        .map_err(Into::into))
}

#[derive(Debug)]
enum InternalAddPackageError {
    CreateTempFile(std::io::Error),
    WriteToFile(std::io::Error),
    CreateTempDir(std::io::Error),
    ExtractZip(ZipError),
    ReadMeta(tokio_fs::ReadJsonError),
    ReadCode(tokio::io::Error),
    AddPackage(package::AddError),
    FlushFile(std::io::Error)
}

impl From<InternalAddPackageError> for package::AddError {
    fn from(value: InternalAddPackageError) -> Self {
        match value {
            InternalAddPackageError::AddPackage(e) => e,
            _ => Self::Internal
        }
    }
}