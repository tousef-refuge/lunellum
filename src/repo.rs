#![allow(unused_variables)]

use anyhow::{bail, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
    pub fn commit(&self, args: CommitArgs) -> Result<()> {
        self.check_lll()?;

        Ok(())
    }

    pub fn init(&self, args: InitArgs) -> Result<()> {
        if self.lll.exists() {
            bail!("A repository already exists on {}", display_path(&self.root));
        }

        fs::create_dir_all(&self.lll)?;
        fs::create_dir_all(&self.files)?;
        fs::create_dir_all(&self.commits)?;
        let head = fs::File::create(&self.head)?;

        println!("Created new repository on {}", display_path(&self.root));
        Ok(())
    }
    
    pub fn status(&self, args: StatusArgs) -> Result<()> {
        self.check_lll()?;

        // TODO: make status checks actually work
        let paths = self.walk_root();
        let mut changed_files = Vec::new();
        for path in paths {
            let file_path = self.files.join(&path);
            if !file_path.exists() {
                changed_files.push(path);
            }
        }

        println!("{}", "Changed files:".blue().bold());
        for path in changed_files {
            println!("   {}", display_path(&path));
        }

        Ok(())
    }

    // util
    fn check_lll(&self) -> Result<()> {
        if !self.lll.exists() {
            bail!("There is no repository on {}. Run `lll init` to make one", display_path(&self.root));
        }
        Ok(())
    }

    fn walk_root(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // TODO: implement gitignore thingy here
        for entry in WalkDir::new(&self.root)
            .into_iter()
            .filter_entry(|e| e.path() != &self.lll)
            .filter_map(|e| e.ok()) {
            let path = entry.path();
            if path == self.root { continue; }

            match path.strip_prefix(&self.root) {
                Ok(relative_path) => {
                    let path_buf: PathBuf = relative_path.to_path_buf();
                    paths.push(path_buf);
                }
                Err(_) => continue,
            }
        }

        paths
    }
}

fn display_path(path: &PathBuf) -> String {
    path.to_string_lossy()
        .strip_prefix(r"\\?\")
        .unwrap_or(&path.to_string_lossy())
        .to_string()
}
