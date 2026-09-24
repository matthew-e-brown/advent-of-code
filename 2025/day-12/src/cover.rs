pub mod build;
pub mod raw;

/// A cover problem with **constraints** `C` and **choices** `R`.
pub struct CoverProblem<C, R> {
    col_labels: indexmap::IndexSet<C>,
    row_labels: indexmap::IndexSet<R>,
    matrix: raw::DLXMatrix,
}
