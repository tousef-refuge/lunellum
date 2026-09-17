use anyhow::Result;
use colored::Colorize;
use std::fs;

use crate::cli::args::{ResetArgs, ViewArgs};
use crate::objects::commit::Commit;
use super::Repo;

// MISERY
impl Repo {
    pub fn reset(&self, args: ResetArgs) -> Result<()> {
        self.check_lll()?;

        // find the right commit
        let mut commits : Vec<Commit> = self.get_commits()?.into_values().collect();
        commits.sort_by_key(|commit| commit.timestamp);
        let current = self.get_commit_from_hash(&args.hash)?;

        // delete all commits with the right hash
        let mut go = false;
        let mut count = 0;
        for commit in commits {
            let hash = commit.hash;
            if !go {
                go = hash == current.hash;
                continue;
            }

            let commit_path = &self.commits.join(hash);
            fs::remove_file(commit_path)?;
            count += 1;
        }

        // man just steal the rest of the code from somewhere else i dont care lmao
        println!("{} {} {}", "Deleted".blue().bold(), count.to_string().bold(), "commits".blue().bold());
        self.view(ViewArgs { hash : args.hash })?;
        Ok(())
    }
}
