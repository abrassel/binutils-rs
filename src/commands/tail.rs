// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
use std::{
    collections::VecDeque,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt)]
pub struct CoreOpt {
    /// Tail from beginning of file instead
    #[structopt(long)]
    start: bool,
    /// Output the last (first with `--start`) bytes
    #[structopt(short = "c")]
    bytes: Option<usize>, // TODO: use custom number parser here
    /// Loop forever trying to read more.
    #[structopt(short, long)]
    follow: bool,
    /// "Wait until the file exists, periodically retrying.  Optional sleep
    /// duration, defaults to 1 sec.
    #[structopt(short, long)]
    retry: Option<Option<f64>>,
    /// Output the last (first with `--start`), lines
    #[structopt(short, long, conflicts_with = "bytes")]
    lines: Option<usize>, // TODO: use custom number parser here
    /// Don't print out file headers
    #[structopt(short, long)]
    quiet: bool,
    /// Instead of printing lines, print without any terminators.
    #[structopt(short, long)]
    zero_terminated: bool,
}

pub fn tail(paths: Vec<PathBuf>, opts: CoreOpt) -> Result<(), TailError> {
    let mut files: Vec<anyhow::Result<(Box<dyn Read>, Box<dyn Write>)>> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut retry: bool = false;
    let mut retry_time: Duration = Duration::from_secs_f64(1.);

    if let Some(time) = opts.retry {
        retry = true;
        if let Some(sec) = time {
            retry_time = Duration::from_secs_f64(sec);
        }
    }

    for path in paths.into_iter() {
        let name = Path::new(&path)
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        names.push(name);
        let file = crate::convert_streams(Some(path), None);
        if retry {
            while file.is_err() {
                std::thread::sleep(retry_time);
            }
        }
        files.push(file);
    }

    let mut line: bool = true;
    let mut num_units: usize = 10;

    if let Some(x) = opts.bytes {
        line = false;
        num_units = x;
    } else if let Some(x) = opts.lines {
        line = true;
        num_units = x;
    }

    for (i, input_output) in files.into_iter().enumerate() {
        if line {
            if opts.start {
                read_lines_start(
                    num_units,
                    opts.zero_terminated,
                    opts.quiet,
                    &names[i],
                    input_output,
                    opts.follow
                );
            } else {
                read_lines_end(
                    num_units,
                    opts.zero_terminated,
                    opts.quiet,
                    &names[i],
                    input_output,
                    opts.follow
                );
            }
        } else {
            if opts.start {
                read_byte_front(
                    num_units,
                    opts.zero_terminated,
                    opts.quiet,
                    &names[i],
                    input_output,
                    opts.follow
                )
            } else {
                read_byte_end(
                    num_units,
                    opts.zero_terminated,
                    opts.quiet,
                    &names[i],
                    input_output,
                    opts.follow
                )
            }
        }
    }

    Ok(())
}

fn read_lines_start(
    num_units: usize,
    zero_terminated: bool,
    quiet: bool,
    name: &str,
    input_output: anyhow::Result<(Box<dyn Read>, Box<dyn Write>)>,
    follow: bool,
) {
    if let Ok((input, mut output)) = input_output {
        if !quiet {
            output
                .write(format!("==>{}<==\n", name).as_bytes())
                .expect("could not write");
        }
        let mut lines = BufReader::new(input).lines();
        for _ in 0..num_units {
            lines.next();
        }
        for line in lines {
            if let Ok(l) = line {
                output
                    .write(
                        format!(
                            "{}",
                            if zero_terminated {
                                l
                            } else {
                                format!("{}\n", l)
                            }
                        )
                        .as_bytes(),
                    )
                    .expect("could not write");
            }
        }
    }
}

fn read_lines_end(
    num_units: usize,
    zero_terminated: bool,
    quiet: bool,
    name: &str,
    input_output: anyhow::Result<(Box<dyn Read>, Box<dyn Write>)>,
    follow: bool,
) {
    if let Ok((input, mut output)) = input_output {
        let lines = BufReader::new(input).lines();
        if !quiet {
            output
                .write(format!("==>{}<==\n", name).as_bytes())
                .expect("could not write");
        }
        let mut lines_deque = VecDeque::new();
        for line in lines {
            lines_deque.push_front(line);
        }
        lines_deque.truncate(num_units);
        for line in lines_deque.into_iter().rev() {
            if let Ok(l) = line {
                output
                    .write(
                        format!(
                            "{}",
                            if zero_terminated {
                                l
                            } else {
                                format!("{}\n", l)
                            }
                        )
                        .as_bytes(),
                    )
                    .expect("could not write");
            }
        }
    }
}

fn read_byte_front(
    num_units: usize,
    zero_terminated: bool,
    quiet: bool,
    name: &str,
    input_output: anyhow::Result<(Box<dyn Read>, Box<dyn Write>)>,
    follow: bool,
) {
    if let Ok((input, mut output)) = input_output {
        let mut bytes = BufReader::new(input).bytes();
        if !quiet {
            output
                .write(format!("==>{}<==\n", name).as_bytes())
                .expect("could not write");
        }
        for _ in 0..num_units {
            bytes.next();
        }
        let bytes: Vec<u8> = bytes
            .map(|x| {
                if let Ok(b) = x {
                    if zero_terminated && b == b'\n' {
                        0
                    } else {
                        b
                    }
                } else {
                    0
                }
            })
            .collect();
        output.write_all(&bytes).expect("could not write");
    }
}

fn read_byte_end(
    num_units: usize,
    zero_terminated: bool,
    quiet: bool,
    name: &str,
    input_output: anyhow::Result<(Box<dyn Read>, Box<dyn Write>)>,
    follow: bool,
) {
    if let Ok((input, mut output)) = input_output {
        let bytes = BufReader::new(input).bytes();
        if !quiet {
            output
                .write(format!("==>{}<==\n", name).as_bytes())
                .expect("could not write");
        }
        let mut bytes_deque = VecDeque::new();
        for byte in bytes {
            if let Ok(b) = byte {
                bytes_deque.push_front(if zero_terminated && b == b'n' { 0 } else { b });
            }
        }
        bytes_deque.truncate(num_units);
        let bytes: Vec<u8> = bytes_deque.into_iter().rev().collect();
        output.write_all(&bytes).expect("could not write");
    }
}

#[derive(Error, Debug)]
pub enum TailError {
    #[error(transparent)]
    IOError(#[from] io::Error),
}
