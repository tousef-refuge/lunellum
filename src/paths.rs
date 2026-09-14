use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::crypto::decompress;

pub fn display_path(path: &PathBuf) -> String {
    path.to_string_lossy()
        .strip_prefix(r"\\?\")
        .unwrap_or(&path.to_string_lossy())
        .to_string()
}

pub fn decompress_read(path: &PathBuf) -> Result<Vec<u8>> {
    if path.exists() {
        Ok(decompress(&fs::read(path)?)?)
    } else {
        Ok(Vec::new())
    }
}
