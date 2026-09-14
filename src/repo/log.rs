use anyhow::Result;
use chrono::{Local, TimeZone};
use colored::Colorize;
use std::cmp::Reverse;

use crate::cli::args::LogArgs;
use super::Repo;

#[allow(unused_variables)]
impl Repo {
    pub fn log(&self, args: LogArgs) -> Result<()> {
        self.check_lll()?;

        let mut commits = self.get_commits()?;
        commits.sort_by_key(|commit| Reverse(commit.timestamp));
        if commits.is_empty() {
            println!("{}", "No commits exist on this repository yet".blue().bold());
        }

        for commit in commits {
            let timestamp = commit.timestamp;
            let seconds = (timestamp / 1_000_000_000) as i64;
            let nanos = (timestamp % 1_000_000_000) as u32;
            let datetime = Local.timestamp_opt(seconds, nanos).unwrap();
            println!("{} : {}", datetime.format("%a %b %d %H:%M:%S %Y").to_string().blue().bold(), commit.info);
        }

        Ok(())
    }
}