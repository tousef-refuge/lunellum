use clap::Args;

#[derive(Args)]
pub struct CommitArgs {
    info: String,
}

#[derive(Args)]
pub struct InitArgs {}

#[derive(Args)]
pub struct StatusArgs {}