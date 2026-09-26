pub mod build;
pub mod error;
pub mod raw;

// [TODO] Come up with a better name/term than "subsets". Or at least find a way to make the method names less awful. Do
// I just refer to them as "rows" and "columns" in all cases? Be honest about the fact that this is DLX; implemented
// with a matrix? Would probably make my life way easier.

/// A cover problem with **constraints** `C` and **subsets** `S`.
pub struct CoverProblem<C, S> {
    matrix: raw::Matrix,
    col_labels: indexmap::IndexSet<C>,
    row_labels: indexmap::IndexSet<S>,
}
