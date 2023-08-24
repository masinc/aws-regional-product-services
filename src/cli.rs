use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[clap(about = "Get the regions of the AWS")]
    Regions(Regions),
    #[clap(about = "Fetch the AWS Regional Product Services data")]
    Fetch(Fetch),
    #[clap(about = "List the services of the region")]
    List(List),
    #[clap(about = "Diff the services of the regions")]
    Diff(Diff),
}

#[derive(Parser, Debug)]
pub struct Regions {}

#[derive(Parser, Debug)]
pub struct Fetch {}

#[derive(Parser, Debug)]
pub struct List {
    /// The region of the AWS
    pub region: String,
}

#[derive(Parser, Debug)]
pub struct Diff {
    /// The region of the AWS
    pub region1: String,
    /// The region of the AWS
    pub region2: String,

    /// if true, no color output (default: false)
    #[clap(long, default_value = "false")]
    pub no_color: bool,

    /// if true, only show the diff (default: false)
    #[clap(long, default_value = "false")]
    pub diff_only: bool,
}
