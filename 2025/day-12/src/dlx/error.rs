use std::convert::Infallible;
use std::error::Error;
use std::fmt::Display;


/// An error that could occur during construction of a [DLX Matrix][super::Matrix].
#[derive(Debug, Clone)]
pub struct BuilderError {
    kind: BuilderErrorKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuilderErrorKind {
    NodeOverflow,
    ColOverflow,
    RowOverflow,
}

impl BuilderError {
    pub fn kind(&self) -> BuilderErrorKind {
        self.kind
    }

    pub(super) const fn col_overflow() -> Self {
        Self { kind: BuilderErrorKind::ColOverflow }
    }

    pub(super) const fn row_overflow() -> Self {
        Self { kind: BuilderErrorKind::RowOverflow }
    }

    pub(super) const fn node_overflow() -> Self {
        Self { kind: BuilderErrorKind::NodeOverflow }
    }
}

impl Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = match self.kind {
            BuilderErrorKind::ColOverflow => "number of columns overflowed maximum allowed column index",
            BuilderErrorKind::NodeOverflow => "number of nodes overflowed maximum allowed node index",
            BuilderErrorKind::RowOverflow => "number of rows overflowed maximum allowed row index",
        };

        write!(f, "matrix construction failed: {reason}")
    }
}

impl Error for BuilderError {}

// Providing conversions from `Infallible` makes the types more versatile in generics:
impl From<Infallible> for BuilderError {
    fn from(err: Infallible) -> BuilderError {
        match err {}
    }
}
