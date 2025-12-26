use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub use clap;
use clap::{ArgAction, ArgMatches, Command, Parser, ValueHint};

/// A single, lazily-initialized instance of the user's [CLI input][Args].
static CLI_ARGS: LazyLock<Args> = LazyLock::new(|| Args::parse());

/// Struct containing both an input file's contents and its path.
///
/// A wrapper struct is used to make working with clap's automatic value parsing a little easier.
#[derive(Debug, Clone)]
struct Input {
    path: PathBuf,
    text: String,
}

/// General command-line format for all Advent of Code puzzle solutions.
#[derive(Parser, Debug)]
struct Args {
    /// The name of the file to read puzzle input from.
    #[arg(
        required = true,
        value_name = "FILE",
        value_hint = ValueHint::FilePath,
        value_parser = load_input,
    )]
    input: Input,

    /// Set to activate printing. Specify multiple times for increased verbosity.
    ///
    /// This value is not necessarily used by all puzzles.
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    /// Any per-program arguments to pass down to each puzzle.
    ///
    /// Not all puzzles make use of additional arguments.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    puzzle_args: Vec<OsString>,
}

/// Value-parser for use with clap.
fn load_input(path: &str) -> std::io::Result<Input> {
    let mut text = fs::read_to_string(path)?;

    // Trim possible trailing newline at the end of a file
    if text.ends_with("\r\n") {
        text.truncate(text.len() - 2);
    } else if text.ends_with("\n") {
        text.truncate(text.len() - 1);
    }

    let path = path.parse::<PathBuf>().unwrap(); // PathBuf parse is infallible
    Ok(Input { path, text })
}

/// Returns the path to the file that was specified as puzzle input on the command line.
///
/// This path is guaranteed to be a [file name][Path::is_file].
pub fn puzzle_input_filename() -> &'static Path {
    CLI_ARGS.input.path.as_path()
}

/// Returns the contents of the file specified as puzzle input on the command line.
///
/// This implementation takes a stance and makes the upfront trade-off of favouring simplicity (reading all input into a
/// static buffer) instead of performance (operating on puzzle input as it is read from the file directly).
pub fn puzzle_input() -> &'static str {
    CLI_ARGS.input.text.as_str()
}

/// Checks program arguments on the command line for verbosity.
pub fn verbosity() -> u8 {
    CLI_ARGS.verbose
}

/// Parses trailing arguments provided on the command line into a custom format that may be used by each individual
/// puzzle.
///
/// Note that deriving [`clap::Parser`] generates code that refers to the `clap` crate by name. Therefore, when deriving
/// `Parser` from this crate's re-export, it is also required that `clap` itself be brought into scope:
///
/// ```rs
/// use aoc_utils::clap::{self, Parser};
///
/// #[derive(Parser)]
/// struct Args {
///     // ...
/// }
/// ```
pub fn parse_puzzle_args<A: Parser>() -> A {
    let matches = match_puzzle_args(A::command());
    match A::from_arg_matches(&matches) {
        Ok(args) => args,
        Err(err) => err.exit(),
    }
}

/// Matches trailing arguments provided on the command line into a [`clap::ArgMatches`] for further handling or parsing.
///
/// Note that the provided [`Command`] will automatically have [`no_binary_name`][Command::no_binary_name] set to
/// `true`.
pub fn match_puzzle_args(cmd: Command) -> ArgMatches {
    // We need to ensure that provided command doesn't expect to see a binary name at the start of its arguments.
    let args = CLI_ARGS.puzzle_args.iter();
    cmd.no_binary_name(true).get_matches_from(args)
}
