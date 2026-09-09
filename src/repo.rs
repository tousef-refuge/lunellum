use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

pub struct Repo {
    root: PathBuf,
    lll: PathBuf,
}

impl Repo {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let mut root = match path.as_ref().canonicalize() {
            Ok(path) => path,
            Err(_) => bail!("The specified path does not exist"),
        };

        if root.is_file() {
            let parent = match root.parent() {
                Some(parent) => parent,
                None => bail!("Could not determine parent directory"),
            };
            root = parent.to_path_buf();
        }

        let lll = root.join("lll");

        Ok(Self { root, lll })
    }
}