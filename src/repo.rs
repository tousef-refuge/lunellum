#![allow(unused_variables)]

use anyhow::{bail, Result};
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

use crate::cli::args::*;
use crate::objects::commit::Commit;
use crate::objects::file_edit::{FileEdit, FileEditType};
use crate::objects::myers_diff::myers_diff;
use crate::objects::Serializable;

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
    // TODO: better commit file names
    pub fn commit(&self, args: CommitArgs) -> Result<()> {
        self.check_lll()?;

        let changed_files = self.get_changed_files();
        if changed_files.is_empty() {
            println!("{}", "No files are changed".blue().bold());
            return Ok(())
        }

        let mut changes: HashMap<PathBuf, Vec<FileEdit>> = HashMap::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();

        for (path, _) in changed_files {
            let file_path = self.files.join(&path);
            let old_data = fs::read(&file_path).unwrap_or_default();
            let new_data = fs::read(self.root.join(&path))?;

            let diff = myers_diff(&old_data, &new_data);
            changes.insert(path, diff);
            // if the commit kills itself this might be a problem but eh
            fs::write(&file_path, new_data)?;
        }

        let commit = Commit { info: args.info, changes };
        fs::write(&self.commits.join(timestamp.to_string()), commit.serialize()?)?;
        fs::write(&self.head, timestamp.to_string())?;
        println!("{} {}", "Committed:".bold().green(), commit.info);

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

    // TODO: separate changed files from inserted files and deleted files
    pub fn status(&self, args: StatusArgs) -> Result<()> {
        self.check_lll()?;

        let changed_files = self.get_changed_files();
        if changed_files.is_empty() {
            println!("{}", "No files are changed".blue().bold());
            return Ok(())
        }

        println!("{}", "Changed files:".blue().bold());
        for (path, _) in &changed_files {
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

    fn get_all_files(&self) -> Vec<PathBuf> {
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

    // TODO: actually check for deleted files
    fn get_changed_files(&self) -> HashMap<PathBuf, FileEditType> {
        let paths = self.get_all_files();
        let mut changed_files : HashMap<PathBuf, FileEditType> = HashMap::new();
        for path in paths {
            let file_path = self.files.join(&path);
            if !file_path.exists() {
                changed_files.insert(path, FileEditType::IsInserted);
                continue
            }

            let old_data = fs::read(&file_path).unwrap_or_default();
            let new_data = fs::read(self.root.join(&path)).unwrap();
            if old_data != new_data {
                changed_files.insert(path, FileEditType::IsEdited);
            }
        }
        changed_files
    }

    // fn get_latest_commit(&self) -> Option<PathBuf> {
    //     // straight jenga
    //     fs::read_dir(&self.commits)
    //         .ok()?
    //         .flatten()
    //         .filter(|entry| entry.file_type().is_ok_and(|t| t.is_file()))
    //         .filter_map(|entry| {
    //             let timestamp = entry.file_name().to_str()?.parse::<i64>().ok()?;
    //             Some((timestamp, entry.path()))
    //         })
    //         .max_by_key(|(timestamp, _)| *timestamp)
    //         .map(|(_, path)| path)
    // }
}

fn display_path(path: &PathBuf) -> String {
    path.to_string_lossy()
        .strip_prefix(r"\\?\")
        .unwrap_or(&path.to_string_lossy())
        .to_string()
}
