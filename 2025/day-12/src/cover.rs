pub mod build;
pub mod raw;

/// A cover problem with **constraints** `C` and **subsets** `S`.
pub struct CoverProblem<C, S> {
    col_labels: indexmap::IndexSet<C>,
    row_labels: indexmap::IndexSet<S>,
    matrix: raw::DLXMatrix,
}
