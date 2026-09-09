use clap::Args;

#[derive(Args)]
pub struct InitArgs {
    /// Chosen directory
    #[arg(default_value = ".")]
    pub path: String,
}