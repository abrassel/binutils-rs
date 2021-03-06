// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
use std::io::{self, Read, Write};

use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt)]
pub struct UniqOpt {
    /// Precede each line with the number of times it occured in the input,
    /// followed by a space
    #[structopt(short, long)]
    count: bool,
    /// Only output repeated lines
    #[structopt(short, long, conflicts_with = "unique")]
    repeated: bool,
    /// Ignore the first <words> words. This is one based.
    #[structopt(short, long = "skip-words", default_value = "0")]
    words: usize,
    /// Ignore the first <graphemes> graphemes. This is one based.
    /// If used in conjunction with \"-d\", applies second.
    #[structopt(short, long = "skip-graphemes", default_value = "0")]
    graphemes: usize,
    /// Perform case insensitive line comparisons.
    #[structopt(short, long)]
    insensitive: bool,
    /// Only output lines that are not repeated in the input.
    #[structopt(short, long)]
    unique: bool,
    /// Consider non-adjacent lines for uniqueness
    #[structopt(long)]
    non_adjacent: bool,
}

pub fn uniq(
    input: &mut impl Read,
    output: &mut impl Write,
    opts: &UniqOpt,
) -> Result<(), UniqError> {
    todo!()
}

#[derive(Error, Debug)]
pub enum UniqError {
    #[error(transparent)]
    IOError(#[from] io::Error),
}
