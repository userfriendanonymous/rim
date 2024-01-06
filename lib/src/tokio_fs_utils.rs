use std::path::Path;

use serde::de::DeserializeOwned;
use tokio::{fs::File, io::{self, AsyncReadExt}};

pub enum ReadJsonError {
    Io(io::Error),
    Json(serde_json::Error),
}

pub async fn read_to_end(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    File::open(path).await?.read_to_end(&mut buf).await?;
    Ok(buf)
}

pub async fn read_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, ReadJsonError> {
    type E = ReadJsonError;
    serde_json::from_reader(File::open(path).await.map_err(E::Io)?.into_std().await).map_err(E::Json)
}
