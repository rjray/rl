use clap::ArgAction::Append;
use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(value_parser, action = Append)]
    pub paths: Vec<String>,

    #[clap(short, long)]
    pub all: bool,

    #[clap(short = 'R', long)]
    pub recursive: bool,
}
