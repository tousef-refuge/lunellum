use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

use crate::cli::args::DetailsArgs;
use crate::objects::file_edit::FileEdit;
use crate::paths::display_path;
use super::Repo;

impl Repo {
    pub fn details(&self, args: DetailsArgs) -> Result<()> {
        self.check_lll()?;
        let current = self.get_commit_from_hash(&args.hash)?;
        println!("{} {} [{}]", "All changes in:".blue().bold(), current.info.bold(), &current.hash[..20]);

        let new_files = self.build_files(&current);
        let mut old_files = new_files.clone();
        let mut seen_paths : Vec<PathBuf> = Vec::new();

        // Heh. To me, this is just regular file building.
        for (path, edits) in current.changes {
            seen_paths.push(path.clone());
            let mut old_data = old_files.get(&path).cloned().unwrap_or_default();
            let mut deletefile = false;

            for edit in edits.iter().rev() {
                match edit {
                    FileEdit::InsertData { pos, data } => {
                        old_data.drain(*pos..(*pos + data.len()));
                    }

                    FileEdit::DeleteData { pos, data } => {
                        old_data.splice(*pos..*pos, data.iter().copied());
                    }

                    FileEdit::InsertFile { .. } => {
                        deletefile = true;
                    }

                    FileEdit::DeleteFile { data } => {
                        old_data = data.clone();
                    }
                }
            }

            if deletefile {
                old_files.remove(&path);
            } else {
                old_files.insert(path, old_data);
            }
        }

        // TODO: make this part not look horrendous
        for path in seen_paths {
            let old_data = old_files.get(&path).cloned().unwrap_or_default();
            let new_data = new_files.get(&path).cloned().unwrap_or_default();

            println!("{}", display_path(&path));
            println!("{}", String::from_utf8_lossy(&old_data).to_string());
            println!("{}", String::from_utf8_lossy(&new_data).to_string());
        }
        
        Ok(())
    }
}