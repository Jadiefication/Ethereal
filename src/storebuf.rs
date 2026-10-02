use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

pub struct StoreBuf {
    pub path: PathBuf,
    text: String,
}

impl StoreBuf {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let buf = path.as_ref().to_path_buf();
        let contents = fs::read_to_string(&buf)?;
        Ok(StoreBuf {
            path: buf,
            text: contents,
        })
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
