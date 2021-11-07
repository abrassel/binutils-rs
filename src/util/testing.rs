// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
//
use std::{
    fmt,
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

use pretty_assertions::assert_eq;
use structopt::StructOpt;

/// Wrapper around string slice that makes debug output `{:?}` to print string
/// same way as `{}`. Used in different `assert*!` macros in combination with
/// `pretty_assertions` crate to make test failures to show nice diffs.
#[derive(PartialEq, Eq)]
struct PrettyString<'a>(pub &'a str);

/// Make diff to display string as multi-line string
impl<'a> fmt::Debug for PrettyString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

#[doc(hidden)]
pub trait Command<R, W, Opt, CommandError> =
    FnOnce(&mut R, &mut W, Opt) -> Result<(), CommandError>
    where
        R: Read,
        W: Write,
        CommandError: std::error::Error;

/// Helper method to run a command and compare it against our tests given some
/// more information.
#[doc(hidden)]
pub fn assert_command<Opt: StructOpt, Cmd, CommandError: Send + Sync + 'static>(
    input_path: &Path,
    command: Cmd,
    command_name: &str,
) -> anyhow::Result<()>
where
    Cmd: Command<BufReader<File>, Vec<u8>, Opt, CommandError>,
{
    let mut input = BufReader::new(File::open(&input_path).expect(&format!(
        "Failed to open test input: {:?}",
        std::fs::canonicalize(&input_path)
    )));
    // first line is the arguments
    let args: Opt = {
        let mut arg_buf = String::new();
        input.read_line(&mut arg_buf)?;
        Opt::from_iter(arg_buf.split_whitespace())
    };

    let mut buf = vec![];
    command(&mut input, &mut buf, args)?;
    let actual = String::from_utf8(buf)?;
    let expected_path = input_path
        .parent()
        .unwrap()
        .join("..")
        .join("..")
        .join("expected")
        .join(command_name)
        .join(input_path.file_name().unwrap());
    let expected = fs::read_to_string(&expected_path).expect(&format!(
        "Failed to open test output: {:?}",
        std::fs::canonicalize(&expected_path)
    ));

    assert_eq!(PrettyString(&actual), PrettyString(&expected),);
    Ok(())
}

/// Generate the unit tests for a command.
/// Read all inputs in `test_inputs/input/<cmd>/*` and generate a test for each.
/// Expect a matching file in `test_inputs/expected/<cmd>/*`.
/// The first line of the input test case should be the command line flags to
/// invoke with. Except the command line flags that might affect stdin / stdout
/// redirection.
#[macro_export]
macro_rules! gen_unit_tests {
    ($cmd:ident, $name:literal) => {
        use std::path::Path;

        use test_generator::test_resources;
        #[test_resources($name)]
        fn verify_resources(resource: &str) {
            crate::util::testing::assert_command(Path::new(resource), $cmd, stringify!($cmd))
                .expect("Test returned error");
        }
    };
}
