use clap::Args;

#[derive(Args)]
pub struct CommitArgs {
    // only adding this flag cause without it
    // my muscle memory keeps getting fried lmao
    /// Information about the commited changes
    #[arg(short = 'm', long = "message")]
    pub info: String,
}

#[derive(Args)]
pub struct DetailsArgs {
    /// Hash of the given commit
    #[arg(default_value = "HEAD-0")]
    pub hash: String,
}

#[derive(Args)]
pub struct InitArgs {}

#[derive(Args)]
pub struct LogArgs {}

#[derive(Args)]
pub struct ResetArgs {
    /// Hash of the given commit
    pub hash: String,
}

#[derive(Args)]
pub struct StatusArgs {}

#[derive(Args)]
pub struct ViewArgs {
    /// Hash of the given commit
    pub hash: String,
}