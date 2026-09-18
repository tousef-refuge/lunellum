use anyhow::{bail, Result};
use colored::Colorize;
use std::collections::BTreeMap;
use std::fs;

use crate::crypto::Serializable;
use crate::objects::commit::Commit;
use super::Repo;

impl Repo {
    pub fn get_commits(&self) -> Result<BTreeMap<String, Commit>> {
        let mut commits = Vec::new();

        for entry in fs::read_dir(&self.commits)? {
            let commit_file = entry?.path();
            let data = fs::read(commit_file)?;
            let commit = Commit::deserialize(&data)?;
            commits.push(commit);
        }

        // "hash map" that doesnt use a hash map how ironic
        let commit_map: BTreeMap<String, Commit> = commits
            .into_iter()
            .map(|x| (x.hash.clone(), x))
            .collect();
        Ok(commit_map)
    }

    pub fn get_commit_from_hash(&self, hash: &str) -> Result<Commit> {
        let commits : Vec<Commit> = self.get_commits()?.into_values().collect();
        if commits.is_empty() {
            println!("{}", "No commits exist on this repository yet".blue().bold());
            std::process::exit(0);
        }

        if hash == "LATEST" {
            let mut sorted = commits.clone();
            sorted.sort_by_key(|commit| commit.timestamp);
            return Ok(sorted.last().unwrap().clone())
        }

        if let Some(offset) = get_head_offset(hash) {
            let head_idx = commits.iter()
                .position(|commit| commit.timestamp == self.get_head())
                .unwrap() as i64;

            let new_idx = head_idx + offset;
            if new_idx < 0 {
                bail!("Cannot view backwards that far (max: {})", head_idx)
            }

            let commit_count = commits.len() as i64;
            if new_idx >= commit_count {
                bail!("Cannot view forwards that far (max: {})", commit_count - head_idx - 1)
            }

            return Ok(commits.get(new_idx as usize).unwrap().clone())
        }

        let commits = self.get_commits()?;

        let mut matches = commits
            .iter()
            .filter(|(key, _)| key.starts_with(hash));

        match (matches.next(), matches.next()) {
            (None, _) => bail!("No commit found with a hash starting with {hash}"),
            (Some((_, commit)), None) => Ok(commit.clone()),
            (Some(_), Some(_)) => bail!("More than one commit found with a hash starting with {hash}"),
        }
    }
}

fn get_head_offset(s: &str) -> Option<i64> {
    let (sign, num) = if let Some(num) = s.strip_prefix("HEAD+") {
        (1, num)
    } else if let Some(num) = s.strip_prefix("HEAD-") {
        (-1, num)
    } else {
        return None;
    };

    if num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    num.parse::<i64>().ok().map(|n| sign * n)
}