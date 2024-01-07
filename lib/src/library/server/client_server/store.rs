use crate::library::store::package;
use crate::PackageId;

impl super::Value {
    pub async fn package_meta(&self, path: package::Path) -> Result<package::Meta, package::MetaError> {
        self.store.read().await.package_meta(path).await.inspect_err(|e| println!("package_meta error: {e:?}")).map_err(Into::into)
    }

    pub async fn add_package(&self, path: package::Path, meta: &package::AddMeta, code: &[u8]) -> Result<PackageId, package::AddError> {
        self.store.write().await.add_package(path, meta, code).await.map_err(Into::into)
    }

    pub async fn package_code(&self, path: package::Path) -> Result<Vec<u8>, package::CodeError> {
        self.store.read().await.package_code(path).await.inspect_err(|e| println!("package_meta error: {e:?}")).map_err(Into::into)
    }
}
