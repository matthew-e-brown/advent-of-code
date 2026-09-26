use std::borrow::Borrow;
use std::hash::Hash;

use indexmap::IndexSet;

use super::CoverProblem;
use super::error::BuildError;
use super::raw::build as raw;

pub struct ConstraintsBuilder<C> {
    raw: raw::HeaderBuilder,
    col_labels: IndexSet<C>,
}

pub struct ProblemBuilder<C, S> {
    raw: raw::MatrixBuilder,
    col_labels: IndexSet<C>,
    row_labels: IndexSet<S>,
}

/// Specification of a constraint during the creation of a [`CoverProblem`].
///
/// Every constraint has a "cover count" associated with it. This count determines how many times subsets containing it
/// may be selected as part of a solution. Once a constraint has been covered by `n` subsets, it and all remaining
/// subsets which contain it are removed from consideration for the rest of the search.
///
/// Additionally, each constraint may be either _required_ or _optional:_
///
/// - A **required** constraint with count `n` **must** be covered exactly `n` times before its associated cover problem
///   is considered solved.
/// - An **optional** constraint with count `n` may be covered **at most** `n` times.
///
/// [`CoverProblem`]: super::CoverProblem
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Constraint<C> {
    pub label: C,
    pub count: usize,
    pub optional: bool,
}

impl<C> Constraint<C> {
    /// Creates a [`raw::Column`] version of this constraint.
    pub const fn as_raw_column(&self) -> raw::Column {
        raw::Column {
            count: self.count,
            optional: self.optional,
        }
    }

    /// Decomposes this constraint into its label and a [`raw::Column`].
    pub fn into_raw(self) -> (C, raw::Column) {
        let Self { label, count, optional } = self;
        (label, raw::Column { count, optional })
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

impl<C> ConstraintsBuilder<C>
where
    C: Hash + Eq,
{
    /// Creates a new [`ConstraintsBuilder`] to start constructing a [`CoverProblem`].
    pub fn new() -> Self {
        ConstraintsBuilder {
            raw: raw::HeaderBuilder::new(),
            col_labels: IndexSet::new(),
        }
    }

    /// Returns the number of constraints in this builder.
    pub const fn num_constraints(&self) -> usize {
        self.raw.num_columns()
    }

    // [TODO] doc comments

    pub fn try_push_constraint(&mut self, constraint: Constraint<C>) -> Result<(), BuildError> {
        let (label, column) = constraint.into_raw();

        if !self.col_labels.insert(label) {
            return Err(BuildError::duplicate_constraint());
        }

        self.raw.try_push_column(column).map_err(BuildError::from)
    }

    pub fn try_push_constraints(
        &mut self,
        constraints: impl IntoIterator<Item = Constraint<C>>,
    ) -> Result<(), BuildError> {
        let constraints = constraints.into_iter();
        let est_len = match constraints.size_hint() {
            (_, Some(max)) => max,
            (min, None) => min,
        };

        self.col_labels.reserve(est_len);
        self.raw.reserve(est_len);
        for constraint in constraints {
            self.try_push_constraint(constraint)?;
        }

        Ok(())
    }

    // [TODO] Add the other `*_constraint(s?)` methods

    pub fn finish_constraints<S>(self) -> ProblemBuilder<C, S> {
        ProblemBuilder {
            raw: self.raw.finish_columns(),
            col_labels: self.col_labels,
            row_labels: IndexSet::new(),
        }
    }
}

impl<C, S> ProblemBuilder<C, S>
where
    C: Hash + Eq,
    S: Hash + Eq,
{
    // [TODO] doc comments

    pub fn try_push_subset<Q: Subset<C>>(&mut self, label: S, subset: Q) -> Result<(), BuildError> {
        if !self.row_labels.insert(label) {
            return Err(BuildError::duplicate_subset());
        }

        self.raw.try_push_row(subset.constraint_labels().map(|label| {
            match self.col_labels.get_index_of(label.borrow()) {
                Some(index) => index,
                None => panic!("subset references unknown constraint label"), // should this be a proper error variant?
            }
        }))?;

        Ok(())
    }

    pub fn try_push_subsets<Q: Subset<C>>(
        &mut self,
        subsets: impl IntoIterator<Item = (S, Q)>,
    ) -> Result<(), BuildError> {
        let subsets = subsets.into_iter();
        let est_len = match subsets.size_hint() {
            (_, Some(max)) => max,
            (min, None) => min,
        };

        self.row_labels.reserve(est_len);
        self.raw.reserve(est_len);
        for (label, subset) in subsets {
            self.try_push_subset(label, subset)?;
        }

        Ok(())
    }

    // [TODO] Add the other `*_subset(s?)` methods

    pub fn build(self) -> CoverProblem<C, S> {
        let Self { raw, col_labels, row_labels } = self;
        CoverProblem {
            matrix: raw.build(),
            col_labels,
            row_labels,
        }
    }
}

/// Items that specify a subset of constraints during construction of a [`CoverProblem`].
///
/// This trait is automatically implemented for any iterator of types that implement <code>[`Borrow<C>`] + [`Hash`] +
/// [`indexmap::Equivalent<C>`]</code>.
pub trait Subset<C> {
    type Label: Borrow<C> + Hash + indexmap::Equivalent<C>;

    /// Gets an iterator over the constraints this subset contains.
    fn constraint_labels(self) -> impl Iterator<Item = Self::Label>;
}

impl<I, C, Q> Subset<C> for I
where
    I: IntoIterator<Item = Q>,
    Q: Borrow<C> + Hash + indexmap::Equivalent<C>,
{
    type Label = Q;

    fn constraint_labels(self) -> impl Iterator<Item = Self::Label> {
        self.into_iter()
    }
}
