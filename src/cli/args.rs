use clap::Args;

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct CommitArgs {
    // only adding this flag cause without it
    // my muscle memory keeps getting fried lmao
    /// Information about the commited changes
    #[arg(short = 'm', long = "message")]
    pub message: Option<String>,

    /// Positional message if -m is omitted
    pub info: Option<String>,
}

#[derive(Args)]
pub struct DetailsArgs {
    /// Hash of the given commit
    #[arg(default_value = "HEAD-0")]
    pub hash: String,

    /// Print data exactly as it is in the file
    #[arg(short = 'm', long = "raw")]
    pub raw: bool,

    /// Print data without color-coding changes
    #[arg(long = "no-color")]
    pub no_color: bool,
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
pub struct TagArgs {
    /// Hash of the given commit
    #[arg(default_value = "HEAD-0")]
    pub hash: String,

    /// Name of the tag
    #[arg(short = 'n', long = "name")]
    pub name: Option<String>,

    /// Forcefully add a new tag if another tag already exists
    #[arg(short = 'f', long = "force")]
    pub force: bool,

    /// Delete the tag attached to the commit if it exists
    #[arg(short = 'd', long = "delete")]
    pub delete: bool,
}

#[derive(Args)]
pub struct ViewArgs {
    /// Hash of the given commit
    pub hash: String,
}