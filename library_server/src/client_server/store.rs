use shared::library::store::{PackageMetaError, package, AddPackageError, PackageCodeError};

impl super::Value {
    pub async fn package_meta(&self, path: package::Path) -> Result<package::Meta, PackageMetaError> {
        self.store.read().await.package_meta(path).await.inspect_err(|e| println!("package_meta error: {e:?}")).map_err(Into::into)
    }

    pub async fn add_package(&self, path: package::Path, meta: &package::Meta, code: &[u8]) -> Result<(), AddPackageError> {
        self.store.write().await.add_package(path, meta, code).await.map_err(Into::into)
    }

    pub async fn package_code(&self, path: package::Path) -> Result<Vec<u8>, PackageCodeError> {
        self.store.read().await.package_code(path).await.inspect_err(|e| println!("package_meta error: {e:?}")).map_err(Into::into)
    }
}
