use clap::ArgAction::Append;
use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(groups(
    &[
        ArgGroup::new("dots").args(&["all", "almost-all"]),
        ArgGroup::new("longs").args(&["long", "long-o", "long-g"])
    ]
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

    /// Append a one-character indicator to files based on type
    #[clap(short = 'F', long)]
    pub classify: bool,

    /// Produce a long listing
    #[clap(short = 'l')]
    pub long: bool,

    /// Like -l, but do not show group information
    #[clap(short = 'o')]
    pub long_o: bool,

    /// Like -l, but do not show owner information
    #[clap(short = 'g')]
    pub long_g: bool,

    /// In a long listing, do not print group names
    #[clap(short = 'G', long)]
    pub no_group: bool,

    /// In a long listing, do not print owner names
    #[clap(short = 'O', long)]
    pub no_owner: bool,

    /// Always quote names
    #[clap(short = 'Q', long)]
    pub quote_name: bool,

    /// Set output width (0 means no limit)
    #[clap(short, long, value_parser)]
    pub width: Option<usize>,

    /// Output names one per line, regardless of width
    #[clap(short = '1')]
    pub one: bool,

    /// Output names with null characters (\0) between them
    #[clap(short = '0')]
    pub zero: bool,
}
