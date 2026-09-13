#![allow(unused_variables)]

use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};
use crate::cli::args::*;

pub struct Repo {
    root: PathBuf,
    lll: PathBuf,

    files: PathBuf,
    commits: PathBuf,
    head: PathBuf,
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
        let files = lll.join("files");
        let commits = lll.join("commits");
        let head = lll.join("HEAD");

        Ok(Self { root, lll, files, commits, head })
    }

    // cli commands
    pub fn init(&self, args: InitArgs) -> Result<()> {
        if self.lll.exists() {
            bail!("A repository already exists on {}", self.display_root());
        }

        fs::create_dir_all(&self.lll)?;
        fs::create_dir_all(&self.files)?;
        fs::create_dir_all(&self.commits)?;
        let head = fs::File::create(&self.head)?;

        println!("Created new repository on {}", self.display_root());
        Ok(())
    }
    
    pub fn status(&self, args: StatusArgs) -> Result<()> {
        self.check_lll()?;
        Ok(())
    }

    // util
    fn display_root(&self) -> String {
        self.root
            .to_string_lossy()
            .strip_prefix(r"\\?\")
            .unwrap_or(&self.root.to_string_lossy())
            .to_string()
    }

    fn check_lll(&self) -> Result<()> {
        if !self.lll.exists() {
            bail!("There is no repository on {}. Run `lll init` to make one", self.display_root());
        }
        Ok(())
    }
}