//! A small collection of utility functions and re-exports of commonly used third-party crates.

pub mod grid;

use std::fs;
use std::sync::LazyLock;

use clap::{ArgAction, Parser, ValueHint};
pub use {arrayvec, regex, scoped_threadpool};

pub use self::grid::Grid;


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
}

// Clap won't accept `fs::read_to_string` directly because of a lifetime issue:
// - https://github.com/clap-rs/clap/issues/4939
// - https://www.reddit.com/r/rust/comments/ntqu68/implementation_of_fnonce_is_not_general_enough/
//
// Using a wrapper function seems to fix it.
fn read_file(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}

static CLI_INPUT: LazyLock<Input> = LazyLock::new(|| Input::parse());

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

/// Creates a new threadpool (see [`scoped_threadpool`]).
///
/// Consults [`std::thread::available_parallelism`] to see how many threads to create.
pub fn threadpool() -> scoped_threadpool::Pool {
    let n = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8);
    scoped_threadpool::Pool::new(n as u32)
}
