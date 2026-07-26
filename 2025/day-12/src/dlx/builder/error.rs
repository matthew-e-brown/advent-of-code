// use std::error::Error;
use std::fmt::Display;

use super::index::{IndexOverflowError, IndexOverflowKind};

/// An error that could occur during construction of a [DLX Matrix][super::Matrix].
#[derive(Debug, Clone)]
pub struct BuilderError {
    kind: BuilderErrorKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuilderErrorKind {
    TooManyColumns,
    TooManyNodes,
    TooManyRows,
}

impl BuilderError {
    pub fn kind(&self) -> BuilderErrorKind {
        self.kind
    }
}

impl Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.kind {
            BuilderErrorKind::TooManyColumns => "column",
            BuilderErrorKind::TooManyNodes => "node",
            BuilderErrorKind::TooManyRows => "row",
        };
        write!(f, "matrix construction failed: number of {name}s overflowed maximum allowed {name} index")
    }
}

impl std::error::Error for BuilderError {}

impl From<IndexOverflowError> for BuilderError {
    fn from(inner: IndexOverflowError) -> Self {
        let kind = match inner.kind {
            IndexOverflowKind::Nodes => BuilderErrorKind::TooManyNodes,
            IndexOverflowKind::Cols => BuilderErrorKind::TooManyColumns,
            IndexOverflowKind::Rows => BuilderErrorKind::TooManyRows,
        };

        Self { kind }
    }
}

impl From<std::convert::Infallible> for BuilderError {
    fn from(err: std::convert::Infallible) -> BuilderError {
        match err {}
    }
}
