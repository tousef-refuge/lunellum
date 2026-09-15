use anyhow::{bail, Result};
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
        } else if let Some(offset) = get_head_offset(&args.hash) {
            if offset == 0 {
                bail!("What did you even achieve from doing that lmao")
            }

            let head_idx = commits.iter()
                .position(|commit| commit.timestamp == self.get_head())
                .unwrap() as i64;

            let new_idx = head_idx + offset;
            if new_idx < 0 {
                bail!("Cannot view backwards that far (max: {})", head_idx)
            }

            let commit_count = commits.len() as i64;
            if new_idx >= commit_count {
                bail!("Cannot view forwards that far (max: {})", commit_count - head_idx - 1)
            }

            current = commits.get(new_idx as usize).unwrap().clone();
        } else {
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

        fs::write(&self.head, current.timestamp.to_string())?;
        println!("{} {} [{}]", "Currently viewing:".blue().bold(), current.info.bold(), &current.hash[..20]);

        Ok(())
    }
}

fn get_head_offset(s: &str) -> Option<i64> {
    let (sign, num) = if let Some(num) = s.strip_prefix("HEAD+") {
        (1, num)
    } else if let Some(num) = s.strip_prefix("HEAD-") {
        (-1, num)
    } else {
        return None;
    };

    if num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    num.parse::<i64>().ok().map(|n| sign * n)
}
