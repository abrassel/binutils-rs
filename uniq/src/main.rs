//! This crate is a port of `binutil`'s [`uniq`](https://man7.org/linux/man-pages/man1/uniq.1.html) tool.
//! It is similar, but not quite identical in functionality.
//!
//! In particular, where the old `uniq` only compares adjacent lines, `uniq-rs`
//! offers non-adjacent comparisons. `uniq-rs` reads from `STDIN` and outputs to
//! `STDOUT` by default, although input and output files may be specified.
//! See the command line documentation for more details.

use std::path::PathBuf;

use structopt::StructOpt;

#[allow(unused)]
#[derive(StructOpt)]
/// The `uniq` utility provides facilities for evaluating uniqueness of lines.
struct Opt {
    #[structopt(
        short,
        long,
        help = "Precede each line with the number of times it occured in the input, followed by a \
                space"
    )]
    count: bool,
    #[structopt(
        short,
        long,
        conflicts_with = "unique",
        help = "Only output repeated lines"
    )]
    repeated: bool,
    #[structopt(
        short,
        long = "skip-words",
        help = "Ignore the first <words> words. This is one based."
    )]
    words: Option<usize>,
    #[structopt(
        short,
        long = "skip-graphemes",
        help = "Ignore the first <graphemes> graphemes. This is one based.
                If used in conjunction with \"-d\", applies second."
    )]
    graphemes: Option<usize>,
    #[structopt(short, long, help = "Perform case insensitive line comparisons.")]
    insensitive: bool,
    #[structopt(
        short,
        long,
        help = "Only output lines that are not repeated in the input."
    )]
    unique: bool,
    #[structopt(long, help = "Consider non-adjacent lines for uniqueness")]
    non_adjacent: bool,
    #[structopt(
        help = "Input file - defaults to `stdin` if not provided, or if filename is \"-\""
    )]
    input: Option<PathBuf>,
    #[structopt(help = "Output file - defaults to `stdout` if not provided")]
    output: Option<PathBuf>,
}

fn main() {
    let _args = Opt::from_args();
}
