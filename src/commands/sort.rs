use std::{
    io::{self, Read, Write, BufReader, BufRead},
    path::PathBuf,
    fs::File, any,
};

use rev_lines::RevLines;
use lazy_static::lazy_static;
use regex::Regex;
use structopt::{StructOpt, clap::ArgGroup};
use thiserror::Error;

#[structopt(
    name = "tail", 
    about = "tail -- display the last part of a file"
)]
#[derive(Debug, StructOpt)]
#[structopt(group = ArgGroup::with_name("block").required(false))]
pub struct TailOpt {
    /// The location is number 512-byte blocks
    #[structopt(short = "b", group = "block")]
    num_blocks: Option<String>, 

    /// The location is number bytes
    #[structopt(short = "c", group = "block")]
    num_bytes: Option<String>,

    ///The -f option causes tail to not stop when end of file is reached, but rather
    ///to wait for additional data to be appended to the input. The -f option is
    ///ignored if the standard input is a pipe, but not if it is a FIFO.
    #[structopt(short = "f")]
    f_option: bool,

    ///The -F option implies the -f option, but tail will also check to see if the
    ///file being followed has been renamed or rotated. The file is closed and reopened
    ///when tail detects that the filename being read from has a new inode number. The
    /// -F option is ignored if reading from standard input rather than a file.
    #[structopt(short = "F")]
    f_upper: bool,

    ///The location is number lines.
    #[structopt(short = "n", group = "block")]
    num_lines: Option<String>,

    ///Suppresses printing of headers when multiple files are being examined.
    #[structopt(short = "q")]
    q_option: bool,

    ///The -r option causes the input to be displayed in reverse order, by line. Additionally,
    ///this option changes the meaning of the -b, -c, and -n options. When the -r
    ///option is specified, these options specify the number of bytes, lines or 512-byte
    ///blocks to display, instead of the bytes, lines or blocks from the beginning or end
    ///of the input from which to begin the display. The default for the -r option
    ///is to display all of the input.
    #[structopt(short = "r")]
    reverse: bool,

    #[structopt()]
    file: Vec<String>,
}

fn main() -> anyhow::Result<()> {

    Ok(())
}