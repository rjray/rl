use clap::ArgAction::Append;
use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(group(
    ArgGroup::new("dots").args(&["all", "almost-all"])
))]
pub struct Cli {
    /// Optional specific paths to list
    #[clap(value_parser, action = Append)]
    pub paths: Vec<String>,

    /// Do not ignore entries whose names start with .
    #[clap(short, long)]
    pub all: bool,

    /// Do not list the implied . and .. directory entries
    #[clap(short = 'A', long)]
    pub almost_all: bool,

    /// List directories themselves, not their contents
    #[clap(short, long)]
    pub directory: bool,

    /// List subdirectories recursively
    #[clap(short = 'R', long)]
    pub recursive: bool,
}
