pub mod commands;
pub mod args;

use clap::Parser;
use commands::Command;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}