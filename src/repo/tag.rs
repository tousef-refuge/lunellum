use anyhow::Result;

use crate::cli::args::TagArgs;
use super::Repo;

impl Repo {
    pub fn tag(&self, args: TagArgs) -> Result<()> {
        self.check_lll()?;
        
        Ok(())
    }
}