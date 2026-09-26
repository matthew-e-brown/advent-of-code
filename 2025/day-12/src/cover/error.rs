use std::fmt::Display;

use super::raw::error::{MatrixOverflowError, MatrixOverflowKind};

#[derive(Debug, Clone)]
pub struct BuildError {
    kind: BuildErrorKind,
}

#[derive(Debug, Clone)]
pub enum BuildErrorKind {
    DuplicateConstraint,
    DuplicateSubset,
    ProblemTooLarge(MatrixOverflowError),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            BuildErrorKind::DuplicateConstraint => write!(f, "encountered multiple constraints with the same label"),
            BuildErrorKind::DuplicateSubset => write!(f, "encountered multiple subsets with the same label"),
            BuildErrorKind::ProblemTooLarge(inner) => match inner.kind() {
                MatrixOverflowKind::Cols => write!(f, "cover problem overflowed: too many constraints"),
                MatrixOverflowKind::Rows => write!(f, "cover problem overflowed: too many subsets"),
                MatrixOverflowKind::Nodes => write!(f, "cover problem too large: overflow occurred"),
            },
        }
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            BuildErrorKind::DuplicateConstraint => None,
            BuildErrorKind::DuplicateSubset => None,
            BuildErrorKind::ProblemTooLarge(inner) => Some(inner),
        }
    }
}

impl BuildError {
    pub(super) fn duplicate_constraint() -> Self {
        Self {
            kind: BuildErrorKind::DuplicateConstraint,
        }
    }

    pub(super) fn duplicate_subset() -> Self {
        Self { kind: BuildErrorKind::DuplicateSubset }
    }

    pub(super) fn too_large(inner: MatrixOverflowError) -> Self {
        inner.into()
    }
}

impl From<MatrixOverflowError> for BuildError {
    fn from(inner: MatrixOverflowError) -> Self {
        Self {
            kind: BuildErrorKind::ProblemTooLarge(inner),
        }
    }
}
