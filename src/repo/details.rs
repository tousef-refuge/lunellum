use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

use crate::cli::args::DetailsArgs;
use crate::crypto::is_binary;
use crate::objects::file_edit::FileEdit;
use crate::paths::display_path;
use super::Repo;

impl Repo {
    //noinspection DuplicatedCode
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
            let mut old_data = old_files.get(&path).unwrap_or(&Vec::new()).to_vec();
            let mut deletefile = false;

            for edit in edits.iter() {
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
            let old_exists = old_files.contains_key(&path);
            let new_exists = new_files.contains_key(&path);

            let old_data = old_files.get(&path).cloned().unwrap_or_default();
            let new_data = new_files.get(&path).cloned().unwrap_or_default();

            println!("\n{} {} :", "*".bold(), display_path(&path).bold());

            if old_exists {
                println!("{}", "BEFORE:".blue().bold());
                print_data(&old_data);
            } else {
                println!("{}", "BEFORE:".blue().bold());
                println!("{}", "File does not exist yet".yellow());
            }

            if new_exists {
                println!("{}", "\nAFTER:".blue().bold());
                print_data(&new_data);
            } else {
                println!("{}", "\nAFTER:".blue().bold());
                println!("{}", "File does not exist anymore".yellow());
            }
        }
        
        Ok(())
    }
}

fn print_data(data : &[u8]) {
    if is_binary(data) {
        println!("{}", "Binary file".yellow());
    } else {
        println!("{}", String::from_utf8_lossy(data).to_string());
    }
}
