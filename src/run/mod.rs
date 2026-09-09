mod init;

use anyhow::Result;
use crate::cli::commands::Command;

pub fn run_command(command: Command) -> Result<()> {
    match command {
        Command::Init(args) => init::run(args),
    }
}