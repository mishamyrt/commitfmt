use clap::Parser;

/// Utility to add ticket id to commit message
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// The lower boundary of the commit range to be checked.
    /// If the `--to` parameter is not set, it will check commits up to the current one
    #[arg(long)]
    pub from: Option<String>,

    /// The upper boundary of the commits range to be checked.
    /// Can be used only together with the `--from` parameter
    #[arg(long)]
    pub to: Option<String>,

    /// Check the message and return an error if any problem is found
    #[arg(short, long)]
    pub lint: bool,
}
