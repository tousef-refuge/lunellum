use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use super::file_edit::FileEdit;
use super::Serializable;

#[derive(Debug, Deserialize, Serialize)]
pub struct Commit {
    pub info: String,
    pub timestamp: i64,
    pub changes: HashMap<PathBuf, Vec<FileEdit>>,
}

impl Serializable for Commit {}
