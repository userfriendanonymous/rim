use std::io::{Write, Read};

use bytes::Bytes;
use tempfile::{tempdir, tempfile};
use tokio::io;
use zip::{ZipWriter, write::FileOptions, result::ZipError};
use crate::{library::store::package, tokio_fs, PackageId};
use super::{HttpFut, URL, Fut};

#[derive(Debug)]
pub enum AddPackageError {
    StdIo(std::io::Error),
    Io(io::Error),
    Zip(ZipError),
    Http(reqwest::Error),
}

impl super::Value {
    pub fn package_meta(&self, path: package::Path) -> impl HttpFut<Result<package::Meta, package::MetaError>> {
        let f = self.client.get(format!("{URL}/store/package_meta/{path}")).send();
        async move {
            let res = f.await?;
            let text = res.text().await?;
            println!("package_meta response: {}", &text);
            Ok(serde_json::from_str(&text).unwrap())
        }
    }

    pub fn package_code(&self, path: package::Path) -> impl HttpFut<Bytes> {
        let f = self.client.get(format!("{URL}/store/package_code/{path}")).send();
        async move {
            f.await?.bytes().await
        }
    }
    
    pub fn add_package(&self, path: package::Path, meta: package::AddMeta, code: Vec<u8>) -> impl Fut<Result<PackageId, AddPackageError>> {
        type E = AddPackageError;
        async fn inner(client: reqwest::Client, path: package::Path, meta: package::AddMeta, code: Vec<u8>) -> Result<PackageId, AddPackageError> {
            let mut zip_w = ZipWriter::new(tempfile().map_err(E::StdIo)?);
            zip_w.start_file("meta.json", FileOptions::default()).map_err(E::Zip)?;
            zip_w.write_all(&serde_json::to_vec(&meta).unwrap()).map_err(E::StdIo)?;
            zip_w.start_file("code.zip", FileOptions::default()).map_err(E::Zip)?;
            zip_w.write_all(&serde_json::to_vec(&code).unwrap()).map_err(E::StdIo)?;
            let mut file = zip_w.finish().map_err(E::Zip)?;
            let mut content = Vec::new();
            file.read_to_end(&mut content).map_err(E::Io)?;
            let r = client.post(format!("{URL}/store/add_package/{path}"))
                .body(content)
                .send().await.map_err(E::Http)?.json().await.map_err(E::Http)?;
            Ok(r)
        }
        inner(self.client.clone(), path, meta, code)
    }
}