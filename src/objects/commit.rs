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

impl Serializable for Commit {}
