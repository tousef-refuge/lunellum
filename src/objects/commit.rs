use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;
use super::fileedit::FileEdit;

pub struct Commit {
    info: String,
    timestamp: i64,
    changes: HashMap<PathBuf, BTreeSet<FileEdit>>,
}