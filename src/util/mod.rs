use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

#[allow(unused)]
pub fn convert_input(input: Option<PathBuf>) -> io::Result<Box<dyn Read>> {
    Ok(match input {
        Some(input) if input != Path::new("-") => box File::open(input)?,
        _ => box io::stdin(),
    })
}

#[allow(unused)]
pub fn convert_output(output: Option<PathBuf>) -> io::Result<Box<dyn Write>> {
    Ok(match output {
        None => box io::stdout(),
        Some(output) => box OpenOptions::new().write(true).open(output)?,
    })
}
