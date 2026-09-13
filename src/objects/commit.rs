use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;
use super::fileedit::FileEdit;

pub struct Commit {
    pub info: String,
    pub timestamp: i64,
    pub changes: HashMap<PathBuf, BTreeSet<FileEdit>>,
}

pub struct FileInfo {
    pub path: PathBuf,
    pub data: String,
}
