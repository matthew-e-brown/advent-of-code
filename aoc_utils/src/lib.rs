//! A small collection of utility functions and re-exports of commonly used third-party crates.

pub mod grid;
mod input;

pub use {arrayvec, regex, scoped_threadpool};

pub use self::grid::Grid;
pub use self::input::*;

/// Creates a new threadpool (see [`scoped_threadpool`]).
///
/// Consults [`std::thread::available_parallelism`] to see how many threads to create.
pub fn threadpool() -> scoped_threadpool::Pool {
    let n = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8);
    scoped_threadpool::Pool::new(n as u32)
}
