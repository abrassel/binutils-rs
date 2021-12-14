//! Tail prints the last part (default 10 lines) of each file or stdin passed to
//! it.
use std::path::PathBuf;

use binutils::commands::tail::{self, CoreOpt, TailError};
use structopt::StructOpt;

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

    tail::tail(files, core_opt);

}
