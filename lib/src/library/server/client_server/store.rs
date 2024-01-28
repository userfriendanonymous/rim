use crate::library::store::{package, family};
use crate::PackageId;

impl super::Value {
    pub async fn package_meta(&self, path: family::Path, version: package::Version) -> Result<package::Meta, package::MetaError> {
        self.store.read().await.package_meta(path, version).await.inspect_err(|e| println!("package_meta error: {e:?}")).map_err(Into::into)
    }

    pub async fn add_package(&self, path: family::Path, meta: &package::AddMeta, code: &[u8]) -> Result<(PackageId, package::Version), package::AddError> {
        self.store.write().await.add_package(path, meta, code).await
            .inspect_err(|e| println!("add_package error: {e:?}"))
            .map_err(Into::into)
    }

    pub async fn package_code(&self, path: family::Path, version: package::Version) -> Result<Vec<u8>, package::CodeError> {
        self.store.read().await.package_code(path, version).await.inspect_err(|e| println!("package_meta error: {e:?}")).map_err(Into::into)
    }
}
