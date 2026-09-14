use clap::Subcommand;
use super::args::*;

#[derive(Subcommand)]
pub enum Command {
    /// Record changes to the repository
    Commit(CommitArgs),

    /// Create a new Lunellum repository
    Init(InitArgs),
    
    /// Get a list of every commit so far
    Log(LogArgs),
    
    /// Show the status of the current branch
    Status(StatusArgs),
}