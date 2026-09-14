use anyhow::{bail, Result};
use std::fs;

use crate::cli::args::InitArgs;
use crate::paths::display_path;
use super::Repo;

#[allow(unused_variables)]
impl Repo {
    pub fn init(&self, args: InitArgs) -> Result<()> {
        if self.lll.exists() {
            bail!("A repository already exists on {}", display_path(&self.root));
        }

        fs::write(&self.lllinclude, "*.txt")?;

        fs::create_dir_all(&self.lll)?;
        fs::create_dir_all(&self.files)?;
        fs::create_dir_all(&self.commits)?;
        let head = fs::File::create(&self.head)?;

        println!("Created new repository on {}", display_path(&self.root));
        Ok(())
    }
}