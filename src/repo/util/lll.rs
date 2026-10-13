use anyhow::{bail, Result};
use std::collections::HashMap;
use std::fs;

use crate::paths::display_path;
use super::Repo;

impl Repo {
    pub fn check_lll(&self) -> Result<()> {
        if !self.lll.exists() {
            bail!("There is no repository on {}. Run `lll init` to make one", display_path(&self.root));
        }
        Ok(())
    }

    // checks if the head is on the latest commit
    // might kill this later if branches are real
    pub fn check_head(&self) -> Result<()> {
        let commits = self.get_commits()?;
        if !commits.is_empty() {
            let latest = commits.values()
                .max_by_key(|commit| commit.timestamp)
                .unwrap().timestamp;

            let current_head = self.get_head();
            if latest != current_head {
                bail!("Cannot run this command as you are currently not on the latest commit. Run `lll view LATEST` and try again")
            }
        }
        Ok(())
    }

    pub fn get_head(&self) -> u128 {
        fs::read_to_string(&self.head)
            .ok()
            .and_then(|s| s.trim().parse::<u128>().ok())
            .unwrap_or(0)
    }

    pub fn get_tags(&self) -> HashMap<String, String> {
        let mut tags = HashMap::new();
        for entry in fs::read_dir(&self.tags).unwrap() {
            let tag = entry.unwrap().path();
            let name = fs::read_to_string(&tag).unwrap().trim().to_string();
            let hash = tag.file_name().unwrap().to_str().unwrap().to_string();
            tags.insert(hash, name);
        };
        tags
    }
}