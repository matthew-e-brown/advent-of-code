//! A collection of commonly used types, utility functions, and re-exports of frequently used third-party crates.

mod cli;
pub mod grid;

pub use {arrayvec, regex, scoped_threadpool};

pub use self::cli::*;
pub use self::grid::Grid;

/// Creates a new threadpool (see [`scoped_threadpool`]).
///
/// Consults [`std::thread::available_parallelism`] to see how many threads to create.
pub fn threadpool() -> scoped_threadpool::Pool {
    let n = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8);
    scoped_threadpool::Pool::new(n as u32)
}
