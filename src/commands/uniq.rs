use std::io::{self, Read, Write};

use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt)]
pub struct UniqOpt {
    /// Precede each line with the number of times it occured in the input,
    /// followed by a space
    #[structopt(short, long)]
    pub count: bool,
    /// Only output repeated lines
    #[structopt(short, long, conflicts_with = "unique")]
    pub repeated: bool,
    /// Ignore the first <words> words. This is one based.
    #[structopt(short, long = "skip-words", default_value = "0")]
    pub words: usize,
    /// Ignore the first <graphemes> graphemes. This is one based.
    /// If used in conjunction with \"-d\", applies second.
    #[structopt(short, long = "skip-graphemes", default_value = "0")]
    pub graphemes: usize,
    /// Perform case insensitive line comparisons.
    #[structopt(short, long)]
    pub insensitive: bool,
    /// Only output lines that are not repeated in the input.
    #[structopt(short, long)]
    pub unique: bool,
    /// Consider non-adjacent lines for uniqueness
    #[structopt(long)]
    pub non_adjacent: bool,
}

pub fn uniq(
    _input: &mut impl Read,
    _output: &mut impl Write,
    _opts: UniqOpt,
) -> Result<(), UniqError> {
    todo!()
}

#[derive(Error, Debug)]
pub enum UniqError {
    #[error(transparent)]
    IOError(#[from] io::Error),
}

#[cfg(test)]
crate::gen_unit_tests!(uniq, "test_inputs/input/uniq/*");
