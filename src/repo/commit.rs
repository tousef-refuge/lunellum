use colored::Colorize;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::args::CommitArgs;
use crate::crypto::*;
use crate::objects::commit::Commit;
use crate::objects::file_edit::{FileEdit, FileEditType};
use crate::objects::myers_diff::myers_diff;
use crate::paths::decompress_read;
use super::Repo;

impl Repo {
    pub fn commit(&self, args: CommitArgs) -> anyhow::Result<()> {
        self.check_lll()?;

        let changed_files = self.get_changed_files();
        if changed_files.is_empty() {
            println!("{}", "No files are changed".blue().bold());
            return Ok(())
        }

        let mut changes: HashMap<PathBuf, Vec<FileEdit>> = HashMap::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();

        for (path, edit_type) in changed_files {
            let file_path = self.files.join(&path);
            let old_data : Vec<u8>;
            let new_data : Vec<u8>;

            match edit_type {
                FileEditType::IsEdited => {
                    old_data = decompress_read(&file_path)?;
                    new_data = fs::read(self.root.join(&path))?;

                    let diff = myers_diff(&old_data, &new_data);
                    changes.insert(path, diff);
                    fs::write(&file_path, compress(&new_data)?)?;
                }

                FileEditType::IsInserted => {
                    new_data = fs::read(self.root.join(&path))?;

                    changes.insert(path, vec![FileEdit::InsertFile { data : new_data.clone() }]);
                    fs::write(&file_path, compress(&new_data)?)?;
                }

                FileEditType::IsDeleted => {
                    old_data = decompress_read(&file_path)?;

                    changes.insert(path, vec![FileEdit::DeleteFile { data : old_data.clone() }]);
                    fs::remove_file(&file_path)?;
                }
            }
        }

        let mut hasher = Sha1::new();
        hasher.update(timestamp.to_be_bytes());
        let hash : String = hasher.finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        let commit = Commit { info: args.info, timestamp, changes };
        fs::write(&self.commits.join(hash), commit.serialize()?)?;
        fs::write(&self.head, timestamp.to_string())?;
        println!("{} {}", "Committed:".bold().green(), commit.info);

        Ok(())
    }
}