use anyhow::Result;

use crate::cli::args::RewordArgs;
use super::Repo;

impl Repo {
    pub fn reword(&self, args: RewordArgs) -> Result<()> {
        self.check_lll()?;

        Ok(())
    }
}