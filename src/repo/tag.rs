use anyhow::Result;
use colored::Colorize;
use std::fs;

use crate::cli::args::TagArgs;
use super::Repo;

impl Repo {
    pub fn tag(&self, args: TagArgs) -> Result<()> {
        self.check_lll()?;

        let mut old_tag = "".to_string();
        let mut tag_replaced = false;

        let current = self.get_commit_from_hash(&args.hash)?;
        for entry in fs::read_dir(&self.tags)? {
            let tag = entry?.path();
            if current.hash == tag.file_name().unwrap().to_string_lossy() {
                // i didnt want old_tag to be mut but rust is STUPID
                old_tag = fs::read_to_string(&tag)?;
                tag_replaced = true;
                if !args.force {
                    println!(
                        "{} ({}){}",
                        "A tag already exists for this commit".blue().bold(),
                        tag_str(&old_tag).green().bold(),
                        ". To override this tag, run lll tag --force".blue().bold(),
                    );
                    return Ok(())
                }
                break;
            }
        }

        fs::write(&self.tags.join(&current.hash), &args.name)?;
        println!(
            "{} {} {} {} [{}] ",
            "Added the tag".blue().bold(),
            tag_str(&args.name).green().bold(),
            "to".blue().bold(),
            current.info.bold(),
            &current.hash[..20]
        );

        if tag_replaced {
            println!(
                "{} {} {}",
                "The tag".blue().bold(),
                tag_str(&old_tag).green().bold(),
                "has been replaced".blue().bold()
            );
        }

        Ok(())
    }
}

fn tag_str(name: &str) -> String {
    format!("\"{}\"", name)
}