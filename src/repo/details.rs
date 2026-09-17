use anyhow::Result;

use crate::cli::args::DetailsArgs;
use super::Repo;

impl Repo {
    pub fn details(&self, args: DetailsArgs) -> Result<()> {
        self.check_lll()?;
        
        Ok(())
    }
}