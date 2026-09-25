use std::hash::Hash;

use indexmap::IndexSet;

use super::error::BuildError;
use super::raw::build as raw;

pub struct ProblemBuilder<C, S> {
    col_labels: IndexSet<C>,
    row_labels: IndexSet<S>,
    inner: raw::MatrixBuilder,
}

/// Specification for a constraint during the creation of a [`CoverProblem`].
///
/// Every constraint has a "cover count" associated with it. This count determines how many times subsets ...[TODO]
///
/// [`CoverProblem`]: super::CoverProblem
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Constraint<C> {
    pub label: C,
    pub count: usize,
    pub optional: bool,
}

impl<C> Constraint<C> {
    /// Creates a [`raw`] version of this constraint.
    pub const fn as_raw(&self) -> raw::Constraint {
        raw::Constraint {
            count: self.count,
            optional: self.optional,
        }
    }

    pub const fn required(label: C) -> Self {
        Constraint { label, count: 1, optional: false }
    }

    pub const fn optional(label: C) -> Self {
        Constraint { label, count: 1, optional: true }
    }

    pub const fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    pub const fn to_required(mut self) -> Self {
        self.optional = false;
        self
    }

    pub const fn to_optional(mut self) -> Self {
        self.optional = true;
        self
    }
}

impl<C: Clone> Constraint<C> {
    /// Returns an iterator that repeats this criterion specification multiple times.
    pub fn repeat(self, n: usize) -> std::iter::RepeatN<Self> {
        std::iter::repeat_n(self, n)
    }
}

impl<C, S> ProblemBuilder<C, S>
where
    C: Hash + Eq,
{
    pub fn new(constraints: impl IntoIterator<Item = Constraint<C>>) -> Result<Self, BuildError> {
        let mut col_labels = IndexSet::new();
        let mut raw_columns = Vec::new();

        // [TODO]
        //
        // It's not nice how we have to iterate through this list twice (we do it once so we can stop at an error, and
        // then the inner builder loops through the columns again). Now that I am looking at this again, what should
        // happen is more clear:
        //
        // - The inner really should have two states: one for columns, and one for rows
        // - Call them `dlx::HeaderBuilder` and `dlx::MatrixBuilder`.
        // - The inner `try_from_constraints` function then gets broken up:
        //   - Pushing the root node in happens when the `HeaderBuilder` is created
        //   - The main loop is then a single `push_column` method
        //   - The part after the main loop happens when converting from `HeaderBuilder` to `MatrixBuilder`.
        // - (also I wanna rename `raw` to `dlx` and `raw::Constraint` to `dlx::Column`, then `Constraint::<C>::as_raw`
        //   can become `as_raw_column` or something more descriptive).

        for constraint in constraints {
            raw_columns.push(constraint.as_raw());
            if !col_labels.insert(constraint.label) {
                return Err(BuildError::duplicate_constraint());
            }
        }

        let inner = raw::MatrixBuilder::try_from_constraints(raw_columns)?;
        Ok(ProblemBuilder {
            col_labels,
            row_labels: IndexSet::new(),
            inner,
        })
    }
}

impl<C, S> ProblemBuilder<C, S>
where
    C: Hash + Eq,
    S: Hash + Eq,
{
    pub fn try_push_subset<'a, Q>(
        &mut self,
        label: S,
        constraints: impl IntoIterator<Item = &'a Q>,
    ) -> Result<(), BuildError>
    where
        Q: ?Sized + Hash + indexmap::Equivalent<C> + 'a,
    {
        if !self.row_labels.insert(label) {
            return Err(BuildError::duplicate_subset());
        }

        self.inner
            .try_push_row(constraints.into_iter().map(|label| match self.col_labels.get_index_of(label) {
                Some(index) => index,
                None => panic!("subset contains unknown constraint label"),
            }))?;

        Ok(())
    }
}
