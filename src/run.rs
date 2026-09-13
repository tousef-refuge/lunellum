use anyhow::Result;
use crate::cli::commands::Command;
use crate::repo::Repo;

#[allow(unreachable_patterns)]
pub fn run_command(command: Command) -> Result<()> {
    let repo = Repo::new(".")?;
    match command {
        Command::Init(args) => repo.init(args),
        Command::Status(args) => repo.status(args),
        
        _ => unimplemented!(),
    }
}