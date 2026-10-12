pub mod commands;
pub mod args;

use clap::Parser;
use commands::Command;

const ABOUT: &str = concat!(
    env!("CARGO_PKG_DESCRIPTION"),
    "\nFor an in-depth guide on how to use lunellum, go to https://github.com/tousef-refuge/lunellum/tree/master/docs");

#[derive(Parser)]
#[command(author, version, about = ABOUT, name = "lll")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}