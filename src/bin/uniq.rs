//! This crate is a port of `binutil`'s [`uniq`](https://man7.org/linux/man-pages/man1/uniq.1.html) tool.
//! It is similar, but not quite identical in functionality.
//!
//! In particular, where the old `uniq` only compares adjacent lines, `uniq-rs`
//! offers non-adjacent comparisons. `uniq-rs` reads from `STDIN` and outputs to
//! `STDOUT` by default, although input and output files may be specified.
//! See the command line documentation for more details.

use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, LineWriter, Read, Write},
    path::{Path, PathBuf},
};

use binutils::commands::uniq::{self, UniqError, UniqOpt};
use structopt::StructOpt;

#[derive(StructOpt)]
/// The `uniq` utility provides facilities for evaluating uniqueness of lines.
struct Opt {
    #[structopt(flatten)]
    core_opt: UniqOpt,
    /// "Input file - defaults to `stdin` if not provided, or if filename is
    /// \"-\""
    input: Option<PathBuf>,
    /// Output file - defaults to `stdout` if not provided
    output: Option<PathBuf>,
}

fn main() -> Result<(), UniqError> {
    let Opt {
        input,
        output,
        core_opt,
    } = Opt::from_args();
    let mut input = convert_input(input.as_ref())?;
    let mut output = convert_output(output.as_ref())?;

    uniq::uniq(&mut input, &mut output, &core_opt)
}
