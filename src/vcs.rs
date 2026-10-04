use anyhow::Result;
use colored::Colorize;
use self_update::cargo_crate_version;
use std::env;

pub fn update_bin() -> Result<()> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner(env::var("UPDATER_USER")?)
        .repo_name(env::var("UPDATER_REPO")?)
        .bin_name(env::var("UPDATER_BIN")?)
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    if status.is_updated() { println!("Updated to version {}", status.version().green()); }
    else { println!("Already up to date"); }
    Ok(())
}