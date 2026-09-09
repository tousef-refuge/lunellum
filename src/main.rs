mod cli;
mod run;

use cli::Cli;
use clap::Parser;
use run::run_command;

fn main() {
    let cli = Cli::parse();
    run_command(cli.command).unwrap();
}
