use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::objects::commit::Commit;
use crate::objects::file_edit::{FileEdit, FileEditType};
use crate::paths::decompress_read;
use super::Repo;

impl Repo {
    pub fn get_all_files(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        let lllinclude = &self.build_gitignore(&self.lllinclude);
        let lllignore = &self.build_gitignore(&self.lllignore);

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

            if lllignore.matched(path, false).is_ignore() {
                continue;
            }

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
        let mut changed_files: HashMap<PathBuf, FileEditType> = HashMap::new();
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
    
    pub fn build_files(&self, current : &Commit) -> HashMap<PathBuf, Vec<u8>> {
        let mut commits : Vec<Commit> = self.get_commits().unwrap().into_values().collect();
        commits.sort_by_key(|commit| commit.timestamp);

        let mut files : HashMap<PathBuf, Vec<u8>> = HashMap::new();
        let mut stop = false;
        for commit in commits {
            if stop { break }
            stop = commit.hash == current.hash;

            let changes = commit.changes;
            for (path, edits) in changes {
                let mut new_data = Vec::new();
                let mut deletefile = false;

                for edit in edits.iter().rev() {
                    match edit {
                        FileEdit::InsertData { pos, data } => {
                            new_data.splice(*pos..*pos, data.iter().copied());
                        }

                        FileEdit::DeleteData { pos, data } => {
                            new_data.drain(*pos..(*pos + data.len()));
                        }

                        FileEdit::InsertFile { data } => {
                            new_data = data.clone();
                        }

                        FileEdit::DeleteFile { .. } => {
                            deletefile = true;
                        }
                    }
                }

                if deletefile {
                    files.remove(&path);
                } else {
                    files.insert(path, new_data);
                }
            }
        }
        
        files
    }

    pub fn build_gitignore(&self, file : &PathBuf) -> Gitignore {
        let mut builder = GitignoreBuilder::new(&self.root);
        if file.is_file() {
            builder.add(file);
        }
        builder.build().unwrap()
    }
}