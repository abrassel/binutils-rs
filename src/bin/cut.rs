use std::path::PathBuf;

use regex::Regex;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CoreOpt {
    /// For each line in the file that matches the input regex, matching groups are outputted.
    regex: Regex,
    /// Output everything but the matched groups
    #[structopt(short, long)]
    complement: bool,
    /// Skip lines that don't have matches.
    #[structopt(short, long)]
    filter: bool,
    /// Binary which takes a sequence of strings and outputs a single string.  This will be used to map to output.
    #[structopt(short, long)]
    transform: Option<PathBuf>,
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(flatten)]
    core_opt: CoreOpt,
    /// The file to read from or STDIN if unspecified or `-`.
    input: Option<PathBuf>,
}

pub fn main() {
    todo!()
}