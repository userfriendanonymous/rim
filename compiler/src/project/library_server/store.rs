use bytes::Bytes;
use shared::library::store::{package, PackageMetaError};

use super::{HttpFut, URL};

impl super::Value {
    pub fn package_meta(&self, path: package::Path) -> impl HttpFut<Result<package::Meta, PackageMetaError>> {
        let f = self.client.get(format!("{URL}/store/package_meta/{path}")).send();
        async move {
            f.await?.json().await
        }
    }

    pub fn package_code(&self, path: package::Path) -> impl HttpFut<Bytes> {
        let f = self.client.get(format!("{URL}/store/package_code/{path}")).send();
        async move {
            f.await?.bytes().await
        }
    }
}