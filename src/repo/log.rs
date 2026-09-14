use anyhow::Result;

use crate::cli::args::LogArgs;
use super::Repo;

#[allow(unused_variables)]
impl Repo {
    pub fn log(&self, args: LogArgs) -> Result<()> {
        Ok(())
    }
}