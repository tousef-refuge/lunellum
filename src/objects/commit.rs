use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use super::fileedit::FileEdit;
use super::Serializable;

#[derive(Deserialize, Serialize)]
pub struct Commit {
    pub info: String,
    pub timestamp: i64,
    pub changes: HashMap<PathBuf, BTreeSet<FileEdit>>,
}

#[derive(Deserialize, Serialize)]
pub struct FileInfo {
    pub path: PathBuf,
    pub data: String,
}

impl Serializable for Commit {}
impl Serializable for FileInfo {}
