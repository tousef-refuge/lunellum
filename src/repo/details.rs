use anyhow::Result;
use colored::Colorize;
use similar::{ChangeTag, TextDiff};
use std::path::PathBuf;
use termimad::print_text;

use crate::cli::args::DetailsArgs;
use crate::crypto::is_binary;
use crate::objects::file_edit::FileEdit;
use crate::paths::display_path;
use super::Repo;

impl Repo {
    pub fn details(&self, args: DetailsArgs) -> Result<()> {
        self.check_lll()?;
        let current = self.get_commit_from_hash(&args.hash)?;
        println!("{} {} [{}]", "All changes in:".blue().bold(), current.info.bold(), &current.hash[..20]);

        let new_files = self.build_files(&current);
        let mut old_files = new_files.clone();
        let mut seen_paths : Vec<PathBuf> = Vec::new();

        // Heh. To me, this is just regular file building.
        for (path, edits) in current.changes {
            seen_paths.push(path.clone());
            let mut old_data = old_files.get(&path).unwrap_or(&Vec::new()).to_vec();
            let mut deletefile = false;

            for edit in edits.iter() {
                match edit {
                    FileEdit::InsertData { pos, data } => {
                        old_data.drain(*pos..(*pos + data.len()));
                    }

                    FileEdit::DeleteData { pos, data } => {
                        old_data.splice(*pos..*pos, data.iter().copied());
                    }

                    FileEdit::InsertFile { .. } => {
                        deletefile = true;
                    }

                    FileEdit::DeleteFile { data } => {
                        old_data = data.clone();
                    }
                }
            }

            if deletefile {
                old_files.remove(&path);
            } else {
                old_files.insert(path, old_data);
            }
        }

        for path in seen_paths {
            let old_exists = old_files.contains_key(&path);
            let new_exists = new_files.contains_key(&path);

            let old_data = old_files.get(&path).cloned().unwrap_or_default();
            let new_data = new_files.get(&path).cloned().unwrap_or_default();

            println!("\n{} {} :", "*".bold(), display_path(&path).bold());

            // println!("{}", "BEFORE:".blue().bold()); repeats a lot and i can
            // technically put that at the beginning of this if statement
            // daga kotowaru
            if old_exists && new_exists {
                let old_text = String::from_utf8_lossy(&old_data);
                let new_text = String::from_utf8_lossy(&new_data);

                let is_binary = is_binary(&old_data) || is_binary(&new_data);
                let is_md = path.extension().unwrap_or_default() == "md";

                if is_binary {
                    println!("{}", "BEFORE:".blue().bold());
                    println!("{}", if args.raw { old_text.white() } else { "Binary file".yellow() });
                    println!("{}", "\nAFTER:".blue().bold());
                    println!("{}", if args.raw { new_text.white() } else { "Binary file".yellow() });
                    continue
                }

                if is_md && !args.raw {
                    println!("{}", "BEFORE:".blue().bold());
                    print_text(&old_text);
                    println!("{}", "\nAFTER:".blue().bold());
                    print_text(&new_text);
                    continue
                }

                if args.no_color {
                    println!("{}", "BEFORE:".blue().bold());
                    println!("{}", old_text);
                    println!("{}", "\nAFTER:".blue().bold());
                    println!("{}", new_text);
                    continue
                }

                let diff = TextDiff::from_chars(&old_text, &new_text);

                println!("{}", "BEFORE:".blue().bold());
                for change in diff.iter_all_changes() {
                    match change.tag() {
                        ChangeTag::Equal => {
                            print!("{}", change.value().to_string())
                        }

                        ChangeTag::Delete => {
                            print!("{}", change.value().to_string().red())
                        }

                        ChangeTag::Insert => {}
                    }
                }

                // double \n cause similar crate is WEIRD bro
                println!("{}", "\n\nAFTER:".blue().bold());
                for change in diff.iter_all_changes() {
                    match change.tag() {
                        ChangeTag::Equal => {
                            print!("{}", change.value().to_string())
                        }

                        ChangeTag::Insert => {
                            print!("{}", change.value().to_string().green())
                        }

                        ChangeTag::Delete => {}
                    }
                }

                println!();
                continue
            }

            if old_exists {
                println!("{}", "BEFORE:".blue().bold());
                print_data(&old_data, &path, &args);
            } else {
                println!("{}", "BEFORE:".blue().bold());
                println!("{}", "File does not exist yet".yellow());
            }

            if new_exists {
                println!("{}", "\nAFTER:".blue().bold());
                print_data(&new_data, &path, &args);
            } else {
                println!("{}", "\nAFTER:".blue().bold());
                println!("{}", "File does not exist anymore".yellow());
            }
        }
        
        Ok(())
    }
}

fn print_data(data: &[u8], path: &PathBuf, args: &DetailsArgs) {
    let text = String::from_utf8_lossy(data);
    if args.raw {
        println!("{}", text);
        return;
    }

    if is_binary(data) {
        println!("{}", "Binary file".yellow());
    } else if path.extension().unwrap_or_default() == "md" {
        print_text(&text);
    } else {
        println!("{}", text);
    }
}
