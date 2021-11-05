// Expose some methods to handle shared behavior.

#![feature(box_syntax)]

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

pub fn convert_streams(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
) -> anyhow::Result<(Box<dyn Read>, Box<dyn Write>)> {
    let input = match input {
        Some(input) if input != Path::new("-") => box File::open(input)? as Box<dyn Read>,
        _ => box io::stdin() as Box<dyn Read>,
    };
    let output = match output {
        None => box io::stdout() as Box<dyn Write>,
        // important not to truncate what is already in the file.
        Some(output) => box OpenOptions::new().write(true).open(output)? as Box<dyn Write>,
    };

    Ok((input, output))
}
