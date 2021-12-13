// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
use std::io::{self, Read, Write};
use std::fs::File;
use itertools::Itertools;
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
    input: &mut impl Read,
    output: &mut impl Write,
    opts: UniqOpt,
) -> Result<(), UniqError> {
    
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;

    if buf.is_empty(){
        return Ok(());
    }

    //do any skipping necessary first
    buf = buf[opts.graphemes..].to_owned();

    if opts.words > 0{
        let words = buf.split(' ').skip(opts.words).collect_vec();
        buf = words.join(" ");
    }


    if opts.insensitive{
        //if we want to perform case-insensitive comparisons, turn all chars to lowercase
        // this has a side effect of the output lines being different from the origionals, TODO is that ok?
        buf = buf.to_lowercase();
    }

    let mut lines: Vec<&str> = buf.split('\n').into_iter().collect_vec();

    if opts.non_adjacent{
        // this has a side effect of losing the origional order of lines, TODO is that ok?
        lines.sort();
    }

    let mut cur_line = lines[0]; 
    //we're guaranteed that theres at least one line by the check on line 48
    lines.remove(0); 
    //lines condensed keeps track of each unique line and the number of times it occured
    let mut lines_condensed: Vec<(&str,usize)> = vec![(cur_line, 1)];

    for next_line in lines{
        if cur_line == next_line{
            let i  = lines_condensed.len()-1;
            let (_, n ) = lines_condensed[i];
            lines_condensed[i] = (cur_line, n+1);
        } else {
            lines_condensed.push((next_line, 1));
        }
        cur_line = next_line;
    }
    
    // writing to the output
    if opts.unique{
        for (line, num_occurences) in lines_condensed{
            if num_occurences == 1 {
                let out = match opts.count {
                    true => format!("{} {}", num_occurences, line),
                    false => line.to_owned()
                };
                output.write(out.as_bytes())?;
            }
        }
    }
    else if opts.repeated{
        for (line, num_occurences) in lines_condensed{
            if num_occurences > 1 {
                let out = match opts.count {
                    true => format!("{} {}", num_occurences, line),
                    false => line.to_owned()
                };
                output.write(out.as_bytes())?;
            }
        }
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum UniqError {
    #[error(transparent)]
    IOError(#[from] io::Error),
}

#[cfg(test)]
crate::gen_unit_tests!(uniq, "test_inputs/input/uniq/*");
