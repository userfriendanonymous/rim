use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};
use tokio::{fs::File, io::{self, AsyncReadExt, AsyncWriteExt}};

#[derive(Debug)]
pub enum ReadJsonError {
    Io(io::Error),
    Json(serde_json::Error),
}

#[derive(Debug)]
pub enum CreateJsonError {
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

pub async fn create(path: impl AsRef<Path>, data: &[u8]) -> io::Result<()> {
    File::create(path).await?.write_all(data).await
}

pub async fn create_json<T: Serialize>(path: impl AsRef<Path>, data: &T) -> io::Result<()> {
    type E = CreateJsonError;
    let mut file = File::create(path).await?;
    file.write_all(&serde_json::to_vec(data).unwrap()).await?;
    Ok(())
}