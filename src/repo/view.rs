use anyhow::Result;
use colored::Colorize;
use std::cmp::Reverse;

use crate::cli::args::ViewArgs;
use crate::objects::commit::Commit;
use super::Repo;

impl Repo {
    pub fn view(&self, args: ViewArgs) -> Result<()> {
        self.check_lll()?;

        let mut commits : Vec<Commit> = self.get_commits()?.into_values().collect();
        commits.sort_by_key(|commit| Reverse(commit.timestamp));
        if commits.is_empty() {
            println!("{}", "No commits exist on this repository yet".blue().bold());
        }

        let commit;

        commit = self.get_commit_from_hash(&args.hash)?;
        println!("{:?}", commit);

        Ok(())
    }
}