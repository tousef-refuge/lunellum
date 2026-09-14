use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::args::CommitArgs;
use crate::objects::commit::Commit;
use crate::objects::file_edit::{FileEdit, FileEditType};
use crate::objects::myers_diff::myers_diff;
use crate::objects::Serializable;
use super::Repo;

impl Repo {
    // cli commands
    // TODO: better commit file names
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
            let old_data = fs::read(&file_path).unwrap_or_default();
            let new_data = fs::read(self.root.join(&path)).unwrap_or_default();

            match edit_type {
                FileEditType::IsEdited => {
                    let diff = myers_diff(&old_data, &new_data);
                    changes.insert(path, diff);
                }

                FileEditType::IsInserted => {
                    changes.insert(path, vec![FileEdit::InsertFile { data : new_data.clone() }]);
                }

                FileEditType::IsDeleted => {
                    changes.insert(path, vec![FileEdit::DeleteFile { data : old_data.clone() }]);
                }
            }

            // if the commit kills itself this might be a problem but eh
            fs::write(&file_path, new_data)?;
        }

        let commit = Commit { info: args.info, changes };
        fs::write(&self.commits.join(timestamp.to_string()), commit.serialize()?)?;
        fs::write(&self.head, timestamp.to_string())?;
        println!("{} {}", "Committed:".bold().green(), commit.info);

        Ok(())
    }
}