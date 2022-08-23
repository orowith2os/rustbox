use clap::Args;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, long_about = None)]

struct Args {
    /// Print Rustbox's version
    #[clap(short = 'v', long = "version", value_parser)]
    version: bool,
}