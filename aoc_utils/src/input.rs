use std::ffi::OsString;
use std::fs;
use std::sync::LazyLock;

pub use clap;
use clap::{ArgAction, ArgMatches, Command, Parser, ValueHint};

/// A single, lazily-initialized instance of the user's [CLI input][Input].
static CLI_INPUT: LazyLock<Input> = LazyLock::new(|| Input::parse());

#[derive(Parser, Debug)]
struct Input {
    /// The name of the file to read puzzle input from.
    #[arg(
        required = true,
        value_name = "FILE",
        value_hint = ValueHint::FilePath,
        value_parser = read_file,
    )]
    input: String,

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

// Clap won't accept `fs::read_to_string` directly because of a lifetime issue:
// - https://github.com/clap-rs/clap/issues/4939
// - https://www.reddit.com/r/rust/comments/ntqu68/implementation_of_fnonce_is_not_general_enough/
//
// Using a wrapper function seems to fix it.
fn read_file(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

/// Returns the contents of the file specified as puzzle input on the command line.
///
/// This implementation takes a stance and makes the upfront trade-off of favouring simplicity (reading all input into a
/// static buffer) instead of performance (operating on puzzle input as it is read from the file directly).
pub fn puzzle_input() -> &'static str {
    &CLI_INPUT.input[..]
}

/// Checks program arguments on the command line for verbosity.
pub fn verbosity() -> u8 {
    CLI_INPUT.verbose
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
pub fn parse_puzzle_args<Args: Parser>() -> Args {
    let matches = match_puzzle_args(Args::command());
    match Args::from_arg_matches(&matches) {
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
    let args = CLI_INPUT.puzzle_args.iter();
    cmd.no_binary_name(true).get_matches_from(args)
}
