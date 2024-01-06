use bytes::Bytes;
use crate::library::store::{package, PackageMetaError};

use super::{HttpFut, URL};

impl super::Value {
    pub fn package_meta(&self, path: package::Path) -> impl HttpFut<Result<package::Meta, PackageMetaError>> {
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
}