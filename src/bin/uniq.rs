//! This crate is a port of `binutil`'s [`uniq`](https://man7.org/linux/man-pages/man1/uniq.1.html) tool.
//! It is similar, but not quite identical in functionality.
//!
//! In particular, where the old `uniq` only compares adjacent lines, `uniq-rs`
//! offers non-adjacent comparisons. `uniq-rs` reads from `STDIN` and outputs to
//! `STDOUT` by default, although input and output files may be specified.
//! See the command line documentation for more details.

#![feature(box_syntax)]

use std::{fs::{File, OpenOptions}, io::{self, BufReader, LineWriter, Read, Write}, path::{Path, PathBuf}};

use structopt::StructOpt;

#[derive(StructOpt)]
/// The `uniq` utility provides facilities for evaluating uniqueness of lines.
struct Opt {
    #[structopt(flatten)]
    core_opt: CoreOpt,
    /// "Input file - defaults to `stdin` if not provided, or if filename is \"-\""
    input: Option<PathBuf>,
    /// Output file - defaults to `stdout` if not provided
    output: Option<PathBuf>,
}

#[derive(StructOpt)]
struct CoreOpt {
    /// Precede each line with the number of times it occured in the input, followed by a
    /// space
    #[structopt(
        short,
        long,
    )]
    count: bool,
    /// Only output repeated lines
    #[structopt(
        short,
        long,
        conflicts_with = "unique",
    )]
    repeated: bool,
    /// Ignore the first <words> words. This is one based.
    #[structopt(
        short,
        long = "skip-words",
    )]
    words: Option<usize>,
    /// Ignore the first <graphemes> graphemes. This is one based.
    /// If used in conjunction with \"-d\", applies second.
    #[structopt(
        short,
        long = "skip-graphemes",
    )]
    graphemes: Option<usize>,
    /// Perform case insensitive line comparisons.
    #[structopt(short, long)]
    insensitive: bool,
    /// Only output lines that are not repeated in the input.
    #[structopt(
        short,
        long,
    )]
    unique: bool,
    /// Consider non-adjacent lines for uniqueness
    #[structopt(long)]
    non_adjacent: bool,
}

fn exec<R: Read, W: Write>(read: &mut R, write: &mut W, core_opt: CoreOpt) -> anyhow::Result<()> {
    // use these for buffered reads and writes - much more efficient
    let read = BufReader::new(read);
    let write = LineWriter::new(write);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let Opt {input, output, core_opt} = Opt::from_args();
    let (mut input, mut output) = binutils::convert_streams(input, output)?;
    exec(&mut input, &mut output, core_opt)
}
