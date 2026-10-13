use anyhow::Result;
use colored::Colorize;
use std::fs;

use crate::cli::args::TagArgs;
use super::Repo;

impl Repo {
    pub fn tag(&self, args: TagArgs) -> Result<()> {
        self.check_lll()?;

        let current = self.get_commit_from_hash(&args.hash)?;
        for entry in fs::read_dir(&self.tags)? {
            let tag = entry?.path();
            if current.hash == tag.file_name().unwrap().to_string_lossy() {
                let old_tag = fs::read_to_string(&tag)?;
                println!("{} ({})", "A tag already exists for this commit".blue().bold(), tag_str(&old_tag).green().bold());
                return Ok(())
            }
        }

        fs::write(&self.tags.join(&current.hash), &args.name)?;
        println!("{} {} {} {} [{}] ", "Added the tag".blue().bold(), tag_str(&args.name).green().bold(), "to".blue().bold(), current.info.bold(), &current.hash[..20]);

        Ok(())
    }
}

fn tag_str(name: &str) -> String {
    format!("\"{}\"", name)
}