use Language::storebuf::StoreBuf;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

struct TempTestFile {
    path: PathBuf,
}

impl TempTestFile {
    fn new(name_prefix: &str, content: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let pid = std::process::id();
        let filename = format!("{}_{}_{}.tmp", name_prefix, pid, timestamp);
        let path = std::env::temp_dir().join(filename);

        let mut file = File::create(&path).expect("failed to create temporary test file");
        file.write_all(content.as_bytes())
            .expect("failed to write content to temporary test file");
        file.flush().expect("failed to flush test file");

        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempTestFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[test]
fn test_storebuf_open_happy_path() {
    let content = "val x: u32 = 42;\nval y = 100;";
    let temp_file = TempTestFile::new("storebuf_happy", content);

    let store_buf = StoreBuf::open(temp_file.path()).expect("StoreBuf::open should succeed");
    assert_eq!(store_buf.text(), content);
    assert_eq!(store_buf.path, temp_file.path());
}

#[test]
fn test_storebuf_open_empty_file() {
    let temp_file = TempTestFile::new("storebuf_empty", "");

    let store_buf =
        StoreBuf::open(temp_file.path()).expect("StoreBuf::open on empty file should succeed");
    assert_eq!(store_buf.text(), "");
    assert_eq!(store_buf.path, temp_file.path());
}

#[test]
fn test_storebuf_open_multiline_unicode_content() {
    let content = "fn greet() {\n    \"Hello, 世界! 🚀\"\n}\n";
    let temp_file = TempTestFile::new("storebuf_unicode", content);

    let store_buf =
        StoreBuf::open(temp_file.path()).expect("StoreBuf::open should succeed with unicode");
    assert_eq!(store_buf.text(), content);
}

#[test]
fn test_storebuf_open_nonexistent_file_returns_error() {
    let non_existent = std::env::temp_dir().join("definitely_non_existent_file_12345.xyz");
    let result = StoreBuf::open(&non_existent);
    assert!(
        result.is_err(),
        "StoreBuf::open should return Err for non-existent file"
    );
}

#[test]
fn test_storebuf_open_directory_returns_error() {
    let dir = std::env::temp_dir();
    let result = StoreBuf::open(&dir);
    assert!(
        result.is_err(),
        "StoreBuf::open should return Err when path is a directory"
    );
}
