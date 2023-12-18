use std::path::PathBuf;
use tokio::{io::{self, AsyncWriteExt}, fs::File};


pub async fn create_string_file(path: PathBuf, content: String) -> Result<(), io::Error> {
    let mut file = File::create(path).await?;
    file.write_all(content.as_bytes()).await?;
    Ok(())
}