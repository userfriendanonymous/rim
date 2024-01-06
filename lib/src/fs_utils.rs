use zip::{ZipArchive, result::ZipResult};
use std::{io, path::Path};

pub fn extract_zip(from: impl io::Read + io::Seek, to: impl AsRef<Path>) -> ZipResult<()> {
    ZipArchive::new(from)?.extract(to)
}