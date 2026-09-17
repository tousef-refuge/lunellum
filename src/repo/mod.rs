pub mod commit;
pub mod init;
pub mod log;
pub mod status;
pub mod util;
pub mod view;
pub mod reset;

use anyhow::{bail, Result};
use std::path::{Path, PathBuf};

pub struct Repo {
    pub root: PathBuf,
    pub lll: PathBuf,
    
    pub lllinclude: PathBuf,
    pub lllignore: PathBuf,

    pub files: PathBuf,
    pub commits: PathBuf,
    pub head: PathBuf,
}

impl Repo {
    // self.__init__() sorta
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

        let lll = root.join(".lll");
        
        let lllinclude = root.join(".lllinclude");
        let lllignore = root.join(".lllignore");
        
        let files = lll.join("files");
        let commits = lll.join("commits");
        let head = lll.join("HEAD");

        Ok(Self { root, lll, lllinclude, lllignore, files, commits, head })
    }
}
