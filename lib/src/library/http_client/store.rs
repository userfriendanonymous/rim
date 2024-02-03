use std::io::{Read, Seek, Write};

use bytes::Bytes;
use tempfile::{tempdir, tempfile};
use tokio::{io, fs::File};
use zip::{ZipWriter, write::FileOptions, result::ZipError};
use crate::{library::store::{package, family}, tokio_fs, PackageId};
use super::{HttpFut, URL, Fut};

#[derive(Debug)]
pub enum AddPackageError {
    StdIo(std::io::Error),
    Io(io::Error),
    Zip(ZipError),
    Http(reqwest::Error),
    Server(package::AddError),
}

type AddPackageOutput = Result<(PackageId, package::Version), AddPackageError>;

impl super::Value {
    pub fn package_meta(&self, path: family::Path, version: package::Version) -> impl HttpFut<Result<package::Meta, package::MetaError>> {
        let f = self.client.get(format!("{URL}/store/package_meta/{path}/{version}")).send();
        async move {
            let res = f.await?;
            let text = res.text().await?;
            println!("package_meta response: {}", &text);
            Ok(serde_json::from_str(&text).unwrap())
        }
    }

    pub fn package_code(&self, path: family::Path, version: package::Version) -> impl HttpFut<Bytes> {
        let f = self.client.get(format!("{URL}/store/package_code/{path}/{version}")).send();
        async move {
            f.await?.bytes().await
        }
    }
    
    pub fn add_package(&self, path: family::Path, meta: package::AddMeta, code: Vec<u8>) -> impl Fut<AddPackageOutput> {
        type E = AddPackageError;
        async fn inner(client: reqwest::Client, path: family::Path, meta: package::AddMeta, code: Vec<u8>) -> AddPackageOutput {
            drop(std::fs::File::create("./stuff").map_err(E::StdIo)?);
            let file = tempfile().map_err(E::StdIo)?;
            let mut zip_w = ZipWriter::new(file);
            zip_w.start_file("meta.json", FileOptions::default()).map_err(E::Zip)?;
            zip_w.write_all(&serde_json::to_vec(&meta).unwrap()).map_err(E::StdIo)?;
            zip_w.start_file("code.zip", FileOptions::default()).map_err(E::Zip)?;
            zip_w.write_all(&code).map_err(E::StdIo)?;
            let mut file = zip_w.finish().map_err(E::Zip)?;
            drop(zip_w);
            file.flush().map_err(E::StdIo)?;
            file.rewind().map_err(E::StdIo)?;
            let r: Result<(PackageId, package::Version), package::AddError> = client.post(format!("{URL}/store/add_package/{path}"))
                .body(file.bytes().collect::<Result<Vec<u8>, _>>().unwrap())
                .send().await.map_err(E::Http)?.json().await.map_err(E::Http)?;
            let r = r.map_err(E::Server)?;
            Ok(r)
        }
        inner(self.client.clone(), path, meta, code)
    }
}