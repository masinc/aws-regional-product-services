use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Yaml,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Text
    }
}

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
    Service(Service),
    #[clap(about = "Diff the services of the regions")]
    Diff(Diff),
}

#[derive(Parser, Debug)]
pub struct Regions {}

#[derive(Parser, Debug)]
pub struct Fetch {}

#[derive(Parser, Debug)]
pub struct Service {
    /// The region of the AWS. If not specified, all regions are listed.
    pub region: Option<String>,

    /// The output format (default: text)
    #[clap(long)]
    pub output: Option<OutputFormat>,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_output_format_default() {
        assert_eq!(OutputFormat::default(), OutputFormat::Text);
    }
}
