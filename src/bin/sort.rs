//! Sort lines of text files.

use std::path::PathBuf;

use structopt::StructOpt;


#[derive(StructOpt)]
struct CoreOpt {
    #[structopt(short = "b", help = "Strip whitespace from front and back of lines")]
    ignore_blanks: bool,
    #[structopt(short, long, help = "Only consider whitespace and dictionary words")]
    ignore_numbers: bool,
    #[structopt(short = "f", long, help = "Ignore case, but output original casing")]
    ignore_case: bool,
    #[structopt(short, long, help = "Use numeric aware sort", conflicts_with = "ignore_numbers")]
    numeric_aware: bool,
    #[structopt(short, long, help = "Use month aware sort")]
    month_aware: bool,
    #[structopt(short, long, help = "Interpret values such as 2K as 2000", conflicts_with = "ignore_numbers")]
    numeric_unit_aware: bool,
    #[structopt(short, long, help = "Reverse the ordering")]
    reverse: bool,
    #[structopt(short, long, help = "Sort version numbers")]
    version_numbers: bool,
    #[structopt(short, long, help = "merge at most `NMOST` inputs at once, temp files for more")]
    batch_size: usize,
    #[structopt(short, long, help = "Check, do not sort")]
    check: bool,
    #[structopt(short, long, help = "Executable that takes two strings and returns a standard comparison integer")]
    key: PathBuf,
    #[structopt(short, long, help = "Merge already sorted files.  Do not sort.", conflicts_with = "check")]
    merge: bool,
    #[structopt(short, long, help = "Stable sort the inputs", conflicts_with = "check")]
    stable: bool,
    #[structopt(short, long, help = "With check, additionally guarantee that the sorts are unique", requires = "check")]
    unique: bool,
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(flatten)]
    core_opt: CoreOpt,
    #[structopt(long, help = "Newline separated filenames to read from, or stdin if `-`")]
    files_from: PathBuf,
    #[structopt(help = "List of files to sort or stdin if `-` is used", conflicts_with = "files_from")]
    input: Vec<PathBuf>,
    #[structopt(short, long, help = "output file")]
    output: PathBuf,
}

pub fn main() {
    todo!()
}