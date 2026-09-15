use anyhow::Result;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::cli::args::ViewArgs;
use crate::objects::commit::Commit;
use crate::objects::file_edit::FileEdit;
use super::Repo;

// MISERY
impl Repo {
    pub fn view(&self, args: ViewArgs) -> Result<()> {
        self.check_lll()?;

        // find the right commit
        let mut commits : Vec<Commit> = self.get_commits()?.into_values().collect();
        commits.sort_by_key(|commit| commit.timestamp);
        if commits.is_empty() {
            println!("{}", "No commits exist on this repository yet".blue().bold());
            return Ok(());
        }

        let current: Commit;
        if args.hash == "LATEST" {
            current = commits.last().unwrap().clone();
        }
        else {
            current = self.get_commit_from_hash(&args.hash)?;
        }

        // store commit change data
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

        // actually write and delete the necessary files
        for entry in WalkDir::new(&self.root)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let relative_path = match path.strip_prefix(&self.root) {
                Ok(path) => path,
                Err(_) => continue,
            };
            if relative_path.starts_with(".lll") {
                continue;
            }

            if !files.contains_key(relative_path) {
                fs::remove_file(path)?;
            }
        }

        for (path, data) in files {
            let final_path = self.root.join(&path);
            if let Some(parent) = final_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(final_path, data)?;
        }

        // TODO: add proper println!
        println!("{}", current.info);

        Ok(())
    }
}