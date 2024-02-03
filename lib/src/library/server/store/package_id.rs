
use serde::{Serialize, Deserialize};
use crate::PackageId as Value;
use tokio::{fs::File, io::{self, AsyncReadExt, AsyncSeekExt, AsyncWriteExt}};

#[derive(Debug)]
pub enum NewError {
    Io(io::Error),
    Serde(serde_json::Error)
}

impl super::Pointer {
    pub(super) async fn new_package_id(&mut self) -> Result<Value, NewError> {
        type E = NewError;
        // self.package_id_lock.write().await;
        println!("{}", self.path.clone().to_str().unwrap());
        let mut file = File::options().read(true).write(true).create(true).open(self.path.join("package_id")).await.map_err(E::Io)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).await.map_err(E::Io)?;
        let value = serde_json::from_str::<Value>(&file_content).unwrap_or(Value::zero());
        file.rewind().await.map_err(E::Io)?;
        file.write_all(serde_json::to_string(&value.inc()).unwrap().as_bytes()).await.map_err(E::Io)?;
        Ok(value)
    }
}