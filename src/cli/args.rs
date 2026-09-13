use clap::Args;

#[derive(Args)]
pub struct CommitArgs {
    pub info: String,
}

#[derive(Args)]
pub struct InitArgs {}

#[derive(Args)]
pub struct StatusArgs {}