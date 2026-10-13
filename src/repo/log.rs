use anyhow::Result;
use chrono::{Local, TimeZone};
use colored::Colorize;
use std::cmp::Reverse;

use crate::cli::args::LogArgs;
use crate::objects::commit::Commit;
use super::Repo;
use super::tag::tag_str;

#[allow(unused_variables)]
impl Repo {
    pub fn log(&self, args: LogArgs) -> Result<()> {
        self.check_lll()?;

        let mut commits : Vec<Commit> = self.get_commits()?.into_values().collect();
        commits.sort_by_key(|commit| Reverse(commit.timestamp));
        if commits.is_empty() {
            println!("{}", "No commits exist on this repository yet".blue().bold());
        }
        let tags = self.get_tags();

        for commit in commits {
            let timestamp = commit.timestamp;
            let seconds = (timestamp / 1_000_000_000) as i64;
            let nanos = (timestamp % 1_000_000_000) as u32;
            let datetime = Local.timestamp_opt(seconds, nanos).unwrap();

            let current_head = self.get_head();
            print!("{} ", if commit.timestamp == current_head {"*".blue().bold()} else {" ".bold()});
            print!("[{}] {} : {} ", &commit.hash[..20], datetime.format("%a %b-%d %Y @ %H:%M:%S").to_string().blue().bold(), commit.info.bold());

            match tags.get(&commit.hash) {
                Some(tag) => println!("({})", tag_str(tag).green().bold()),
                None => println!(),
            }
        }

        Ok(())
    }
}