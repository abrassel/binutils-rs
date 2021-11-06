//! Tail prints the last part (default 10 lines) of each file or stdin passed to
//! it.

use std::{path::PathBuf};

use structopt::StructOpt;

#[derive(StructOpt)]
struct CoreOpt {
    /// Tail from beginning of file instead
    #[structopt(long)]
    start: bool,
    /// Output the last (first with `--start`) bytes
    #[structopt(short = "c")]
    bytes: Option<usize>, // TODO: use custom number parser here
    /// Loop forever trying to read more.
    #[structopt(short, long)]
    follow: bool,
    /// "Wait until the file exists, periodically retrying.  Optional sleep duration,
    /// defaults to 1 sec.
    #[structopt(
        short,
        long,
    )]
    retry: Option<Option<f64>>,
    /// Output the last (first with `--start`), lines
    #[structopt(
        short,
        long,
        conflicts_with = "bytes"
    )]
    lines: Option<usize>, // TODO: use custom number parser here
    /// Don't print out file headers
    #[structopt(short, long)]
    quiet: bool,
    /// Instead of printing lines, print without any terminators.
    #[structopt(
        short,
        long,
    )]
    zero_terminated: bool,
}

// TODO: challenge: https://docs.rs/inotify/0.9.6/inotify/struct.Inotify.html
#[derive(StructOpt)]
struct Opt {
    #[structopt(flatten)]
    core_opt: CoreOpt,
    #[structopt(
        help = "If no files are given, or one file consisting of `-`, read from `stdin`.  If more \
                than one file is given, each file in sequence."
    )]
    files: Vec<PathBuf>,
}
pub fn main() {
    let Opt { files, core_opt } = Opt::from_args();
    todo!(
        "See starter code in `uniq` for some ideas.  I suggest making an iterator that zips all \
         of the read files.  Make sure to delegate your call soon to an `exec` function I can test."
    )
}
