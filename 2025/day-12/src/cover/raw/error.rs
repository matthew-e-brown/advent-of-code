use std::fmt::Display;

use super::{ColOverflowError, NodeOverflowError, RowOverflowError};

#[derive(Debug, Clone)]
pub struct MatrixOverflowError {
    kind: MatrixOverflowKind,
}

impl MatrixOverflowError {
    pub const fn kind(&self) -> MatrixOverflowKind {
        self.kind
    }
}

impl Display for MatrixOverflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.kind {
            MatrixOverflowKind::Rows => "number of rows too high: overflow occurred",
            MatrixOverflowKind::Cols => "number of columns too high: overflow occurred",
            MatrixOverflowKind::Nodes => "number of nodes too high: overflow occurred",
        })
    }
}

impl std::error::Error for MatrixOverflowError {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MatrixOverflowKind {
    Rows,
    Cols,
    Nodes,
}

impl From<RowOverflowError> for MatrixOverflowError {
    fn from(_: RowOverflowError) -> Self {
        Self { kind: MatrixOverflowKind::Rows }
    }
}

impl From<ColOverflowError> for MatrixOverflowError {
    fn from(_: ColOverflowError) -> Self {
        Self { kind: MatrixOverflowKind::Cols }
    }
}

impl From<NodeOverflowError> for MatrixOverflowError {
    fn from(_: NodeOverflowError) -> Self {
        Self { kind: MatrixOverflowKind::Nodes }
    }
}
