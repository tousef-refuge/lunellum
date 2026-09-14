use anyhow::{bail, Result};
use ignore::gitignore::GitignoreBuilder;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use crate::crypto::Serializable;
use crate::objects::commit::Commit;
use crate::objects::file_edit::FileEditType;
use crate::paths::{decompress_read, display_path};
use super::Repo;

impl Repo {
    pub fn check_lll(&self) -> Result<()> {
        if !self.lll.exists() {
            bail!("There is no repository on {}. Run `lll init` to make one", display_path(&self.root));
        }
        Ok(())
    }
    
    // checks if the head is on the latest commit
    // might kill this later if branches are real
    pub fn check_head(&self) -> Result<()> {
        let commits = self.get_commits()?;
        if !commits.is_empty() {
            let latest = commits.values()
                .max_by_key(|commit| commit.timestamp)
                .unwrap().timestamp;

            let current_head = fs::read_to_string(&self.head)
                .ok()
                .and_then(|s| s.trim().parse::<u128>().ok())
                .unwrap_or(0);

            if latest != current_head {
                bail!("Cannot run this command as you are currently not on the latest commit")
            }
        }
        Ok(())
    }

    pub fn get_all_files(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        let mut builder = GitignoreBuilder::new(&self.root);
        if self.lllinclude.is_file() {
            builder.add(&self.lllinclude);
        }
        let lllinclude = builder.build().unwrap();

        for entry in WalkDir::new(&self.root)
            .into_iter()
            .filter_entry(|e| e.path() != &self.lll)
            .filter_map(|e| e.ok()) {
            let path = entry.path();
            if path == self.root { continue; }

            let relative_path = match path.strip_prefix(&self.root) {
                Ok(path) => path,
                Err(_) => continue,
            };

            if lllinclude.matched(path, false).is_ignore() {
                paths.push(relative_path.to_path_buf());
            }
        }

        // always include .lllinclude itself
        if let Ok(relative_path) = self.lllinclude.strip_prefix(&self.root) {
            paths.push(relative_path.to_path_buf());
        }

        paths
    }

    pub fn get_changed_files(&self) -> HashMap<PathBuf, FileEditType> {
        let paths = self.get_all_files();
        let mut changed_files : HashMap<PathBuf, FileEditType> = HashMap::new();
        for path in paths {
            let file_path = self.files.join(&path);
            if !file_path.exists() {
                changed_files.insert(path, FileEditType::IsInserted);
                continue
            }

            let old_data = decompress_read(&file_path).unwrap();
            let new_data = fs::read(self.root.join(&path)).unwrap();
            if old_data != new_data {
                changed_files.insert(path, FileEditType::IsEdited);
            }
        }

        for entry in WalkDir::new(&self.files)
            .into_iter()
            .filter_map(|e| e.ok()) {
            let path = entry.path();

            let relative_path = match path.strip_prefix(&self.files) {
                Ok(path) => path,
                Err(_) => continue,
            };

            let path_buf: PathBuf = relative_path.to_path_buf();
            let root_path = &self.root.join(&path_buf);
            if !root_path.exists() {
                changed_files.insert(path_buf, FileEditType::IsDeleted);
                continue
            }
        }

        changed_files
    }

    pub fn get_commits(&self) -> Result<BTreeMap<String, Commit>> {
        let mut commits = Vec::new();

        for entry in fs::read_dir(&self.commits)? {
            let commit_file = entry?.path();
            let data = fs::read(commit_file)?;
            let commit = Commit::deserialize(&data)?;
            commits.push(commit);
        }

        // "hash map" that doesnt use a hash map how ironic
        let commit_map: BTreeMap<String, Commit> = commits
            .into_iter()
            .map(|x| (x.hash.clone(), x))
            .collect();
        Ok(commit_map)
    }
}