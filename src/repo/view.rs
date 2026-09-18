use anyhow::Result;
use colored::Colorize;
use std::fs;
use walkdir::WalkDir;

use crate::cli::args::ViewArgs;
use crate::objects::commit::Commit;
use super::Repo;

// MISERY
impl Repo {
    pub fn view(&self, args: ViewArgs) -> Result<()> {
        self.check_lll()?;

        // find the right commit
        let mut commits : Vec<Commit> = self.get_commits()?.into_values().collect();
        commits.sort_by_key(|commit| commit.timestamp);
        let current = self.get_commit_from_hash(&args.hash)?;

        // get commit change data
        let files = self.build_files(&current);

        // actually write and delete the necessary files
        let lllignore = &self.build_gitignore(&self.lllignore);

        for entry in WalkDir::new(&self.root)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            if lllignore.matched(path, false).is_ignore() {
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
