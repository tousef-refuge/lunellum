use colored::Colorize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::cli::args::StatusArgs;
use crate::objects::file_edit::FileEditType;
use crate::paths::display_path;
use super::Repo;

#[allow(unused_variables)]
impl Repo {
    pub fn status(&self, args: StatusArgs) -> anyhow::Result<()> {
        self.check_lll()?;

        let changed_files = self.get_changed_files();
        if changed_files.is_empty() {
            println!("{}", "No files are changed".blue().bold());
            return Ok(())
        }

        let mut edit_types : HashMap<FileEditType, Vec<PathBuf>> = HashMap::from([
            (FileEditType::IsEdited, Vec::new()),
            (FileEditType::IsInserted, Vec::new()),
            (FileEditType::IsDeleted, Vec::new()),
        ]);

        for (path, edit_type) in changed_files {
            edit_types.get_mut(&edit_type).unwrap().push(path);
        }

        for (edit_type, paths) in &edit_types {
            let title = match edit_type {
                FileEditType::IsEdited => "Changed files:",
                FileEditType::IsInserted => "New files:",
                FileEditType::IsDeleted => "Deleted files:",
            };
            println!("{}", title.blue().bold());
            for path in paths { println!("   {}", display_path(&path)); }
            println!();
        }

        Ok(())
    }
}