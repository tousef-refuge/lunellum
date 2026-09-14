use anyhow::bail;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::objects::file_edit::FileEditType;
use crate::paths::{decompress_read, display_path};
use super::Repo;

impl Repo {
    pub fn check_lll(&self) -> anyhow::Result<()> {
        if !self.lll.exists() {
            bail!("There is no repository on {}. Run `lll init` to make one", display_path(&self.root));
        }
        Ok(())
    }

    pub fn get_all_files(&self) -> Vec<PathBuf> {
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

            match path.strip_prefix(&self.files) {
                Ok(relative_path) => {
                    let path_buf: PathBuf = relative_path.to_path_buf();
                    let root_path = &self.root.join(&path_buf);
                    if !root_path.exists() {
                        changed_files.insert(path_buf, FileEditType::IsDeleted);
                        continue
                    }
                }

                Err(_) => continue,
            }
        }

        changed_files
    }

    // pub fn get_latest_commit(&self) -> Option<PathBuf> {
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