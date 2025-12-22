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

/// Counts the number of true boolean expressions.
///
/// By default, this macro returns the number as a `usize`. This can be configured by passing a type at the end of the
/// list, after a semicolon, with `as <type>`. The type can be anything that implements [`From<bool>`].
///
/// # Examples
///
/// ```
/// # use aoc_utils::count_bools;
/// let a = 5;
/// let b = 10;
///
/// assert_eq!(count_bools!(a == b, a < b, b < 20), 2);
/// assert_eq!(count_bools!(a != b, b > a, 20 > b; as u8), 3u8);
/// ```
#[macro_export]
macro_rules! count_bools {
    ($bool:expr, $($others:expr),+) => {
        count_bools!($bool, $($others),+; as usize)
    };
    ($bool:expr$(,)?) => {
        count_bools!($bool; as usize)
    };
    ($bool:expr, $($others:expr),+; as $type:ty) => {
        count_bools!($bool; as $type) + count_bools!($($others),+; as $type)
    };
    ($bool:expr; as $type:ty) => {
        (<$type as ::std::convert::From<bool>>::from($bool))
    };
}

/// Exactly like the vanilla [`println!`] macro, but only prints when [verbosity] is at least a certain level (i.e.,
/// based on the `-v` flag passed on the command-line).
///
/// # Example
///
/// ```
/// # let x = 10;
/// // This:
/// println_v!(2, "x = {x}.");
///
/// // Is equivalent to:
/// if aoc_utils::verbosity() >= 2 {
///     println!("x = {x}.");
/// }
/// ```
///
/// An `==` sign can be used to print only when verbosity is _exactly_ equal to the given value:
///
/// ```
/// # let y = 5;
/// println_v!(== 1, "-v passed: y = {y}.")
///
/// if aoc_utils::verbosity() == 1 {
///     println!("-v passed: y = {y}.");
/// }
/// ```
#[macro_export]
macro_rules! println_v {
    ($v:expr) => {
        println_v!($v,);
    };
    ($v:expr, $($args:tt)*) => {
        if $crate::verbosity() >= $v {
            println!($($args)*);
        }
    };
    (== $v:expr) => {
        println_v!(== $v,);
    };
    (== $v:expr, $(args:tt)*) => {
        if $crate::verbosity() == $v {
            println!($($args)*);
        }
    };
}

/// Exactly like the vanilla [`print!`] macro, but only prints when [verbosity] is at least a certain level.
///
/// See [`println_v!`] for examples.
#[macro_export]
macro_rules! print_v {
    ($v:expr) => {
        print_v!($v,);
    };
    ($v:expr, $($args:tt)*) => {
        if $crate::verbosity() >= $v {
            print!($($args)*);
        }
    };
    (== $v:expr) => {
        print_v!(== $v,);
    };
    (== $v:expr, $(args:tt)*) => {
        if $crate::verbosity() == $v {
            print!($($args)*);
        }
    };
}
