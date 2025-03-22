//! A small collection of utility functions and re-exports of commonly used third-party crates.

pub mod grid;

use std::fs;
use std::path::PathBuf;

use clap::{Parser, ValueHint};
pub use {arrayvec, regex, scoped_threadpool};

pub use self::grid::Grid;


#[derive(Parser, Debug)]
struct Input {
    /// The name of the file to read puzzle input from.
    #[arg(required = true, value_name = "FILE", value_hint = ValueHint::FilePath)]
    path: PathBuf,
}

/// Checks program arguments on the command line for a filename and reads it to a string.
///
/// This implementation takes a stance and makes the upfront trade-off of favouring simplicity (reading all input into a
/// buffer) instead of performance (operating on puzzle input as it is read from the file).
pub fn puzzle_input() -> String {
    let path = Input::parse().path;
    fs::read_to_string(path).expect("failed to read puzzle input from file")
}

/// Creates a new threadpool (see [`scoped_threadpool`]).
///
/// Consults [`std::thread::available_parallelism`] to see how many threads to create.
pub fn threadpool() -> scoped_threadpool::Pool {
    let n = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8);
    scoped_threadpool::Pool::new(n as u32)
}
