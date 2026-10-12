use anyhow::Result;
use colored::Colorize;
use self_update::cargo_crate_version;
use std::env;

pub const REPO_OWNER: &str = "tousef-refuge";
pub const REPO_NAME: &str = "lunellum";
pub const BIN_NAME: &str = "lll";

pub fn update_bin() -> Result<()> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(BIN_NAME)
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    if status.is_updated() { println!("Updated to version {}", status.version().green()); }
    else { println!("Already up to date"); }
    Ok(())
}