use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::crypto::Serializable;
use super::file_edit::FileEdit;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Commit {
    pub info: String,
    pub hash: String,
    pub timestamp: u128,
    pub changes: HashMap<PathBuf, Vec<FileEdit>>,
}

impl Serializable for Commit {}
