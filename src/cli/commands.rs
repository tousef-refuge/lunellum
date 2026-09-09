use clap::Subcommand;
use super::args::*;

#[derive(Subcommand)]
pub enum Command {
    /// Create a new Lunellum repository
    Init(InitArgs)
}