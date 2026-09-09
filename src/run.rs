use anyhow::Result;
use crate::cli::commands::Command;
use crate::repo::Repo;

pub fn run_command(command: Command) -> Result<()> {
    let repo = Repo::new(".")?;
    match command {
        Command::Init(args) => repo.init(args),
    }
}