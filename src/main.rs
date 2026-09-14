mod cli;
mod objects;

mod repo;
mod run;
mod paths;

use cli::Cli;
use clap::Parser;
use colored::Colorize;
use run::run_command;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run_command(cli.command) {
        eprintln!("{} {}", "ERROR:".bold().red(), e.to_string().red());
        std::process::exit(1);
    }
}
