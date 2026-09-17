use clap::Subcommand;
use super::args::*;

#[derive(Subcommand)]
pub enum Command {
    /// Record changes to the repository
    Commit(CommitArgs),

    /// Get a list of changes of a given commit
    Details(DetailsArgs),

    /// Create a new Lunellum repository
    Init(InitArgs),
    
    /// Get a list of every commit so far
    Log(LogArgs),
    
    /// Reset the repository back to a given commit
    Reset(ResetArgs),
    
    /// Show the status of the current branch
    Status(StatusArgs),

    /// View the file structure of a specific commit
    View(ViewArgs),
}