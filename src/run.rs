use anyhow::Result;
use crate::cli::commands::Command;
use crate::repo::Repo;

#[allow(unreachable_patterns)]
pub fn run_command(command: Command) -> Result<()> {
    let repo = Repo::new(".")?;
    match command {
        Command::Commit(args) => repo.commit(args),
        Command::Init(args) => repo.init(args),
        Command::Log(args) => repo.log(args),
        Command::Reset(args) => repo.reset(args),
        Command::Status(args) => repo.status(args),
        Command::View(args) => repo.view(args),
        
        _ => unimplemented!(),
    }
}