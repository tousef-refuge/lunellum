use anyhow::Result;

use crate::cli::args::ViewArgs;
use super::Repo;

impl Repo {
    pub fn view(&self, args: ViewArgs) -> Result<()> {
        self.check_lll()?;

        Ok(())
    }
}