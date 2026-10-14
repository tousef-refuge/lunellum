use anyhow::Result;
use colored::Colorize;
use std::fs;

use crate::cli::args::RewordArgs;
use crate::crypto::Serializable;
use super::Repo;

impl Repo {
    pub fn reword(&self, args: RewordArgs) -> Result<()> {
        self.check_lll()?;

        let mut current = self.get_commit_from_hash(&args.hash)?;
        current.info = args.message;
        fs::write(&self.commits.join(&current.hash), current.serialize()?)?;
        println!(
            "[{}] {} {}",
            &current.hash[..20],
            "Changed the message to:".blue().bold(),
            current.info
        );

        Ok(())
    }
}