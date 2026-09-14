pub mod util;

use anyhow::{bail, Result};
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cli::args::*;
use crate::objects::commit::Commit;
use crate::objects::file_edit::{FileEdit, FileEditType};
use crate::objects::myers_diff::myers_diff;
use crate::objects::Serializable;
use crate::paths::display_path;

pub struct Repo {
    pub root: PathBuf,
    pub lll: PathBuf,

    pub files: PathBuf,
    pub commits: PathBuf,
    pub head: PathBuf,
}

impl Repo {
    // self.__init__() sorta
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let mut root = match path.as_ref().canonicalize() {
            Ok(path) => path,
            Err(_) => bail!("The specified path does not exist"),
        };

        if root.is_file() {
            let parent = match root.parent() {
                Some(parent) => parent,
                None => bail!("Could not determine parent directory"),
            };
            root = parent.to_path_buf();
        }

        let lll = root.join(".lll");
        let files = lll.join("files");
        let commits = lll.join("commits");
        let head = lll.join("HEAD");

        Ok(Self { root, lll, files, commits, head })
    }

    // cli commands
    // TODO: better commit file names
    pub fn commit(&self, args: CommitArgs) -> Result<()> {
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

    pub fn init(&self, args: InitArgs) -> Result<()> {
        if self.lll.exists() {
            bail!("A repository already exists on {}", display_path(&self.root));
        }

        fs::create_dir_all(&self.lll)?;
        fs::create_dir_all(&self.files)?;
        fs::create_dir_all(&self.commits)?;
        let head = fs::File::create(&self.head)?;

        println!("Created new repository on {}", display_path(&self.root));
        Ok(())
    }

    pub fn status(&self, args: StatusArgs) -> Result<()> {
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
