use anyhow::Result;

use crate::cli::args::ResetArgs;
use super::Repo;

// MISERY
impl Repo {
    pub fn reset(&self, args: ResetArgs) -> Result<()> {
        self.check_lll()?;
        Ok(())
    }
}
