// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

#[allow(unused)]
pub fn convert_input<P: AsRef<Path>>(input: Option<P>) -> io::Result<Box<dyn Read>> {
    Ok(match input {
        Some(input) if input.as_ref() != Path::new("-") => box File::open(input)?,
        _ => box io::stdin(),
    })
}

#[allow(unused)]
pub fn convert_output<P: AsRef<Path>>(output: Option<P>) -> io::Result<Box<dyn Write>> {
    Ok(match output {
        None => box io::stdout(),
        Some(output) => box OpenOptions::new().write(true).open(output)?,
    })
}
