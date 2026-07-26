use std::error::Error;
use std::fmt::Display;

// Allows configuring the inner size of each of the new-type structs.
type ColIdxInner = u32;
type RowIdxInner = u32;
type NodeIdxInner = u32;

/// An index that refers to a specific column in a [`Matrix`][super::Matrix].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColIndex(pub(super) ColIdxInner);

/// An index that refers to a specific row in a [`Matrix`][super::Matrix].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowIndex(pub(super) RowIdxInner);

/// An index into [`super::Matrix::nodes`].
///
/// These are used as the main links to create the linked-lattice between the nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct NodeIndex(pub(super) NodeIdxInner);

impl NodeIndex {
    /// The index of the root node.
    pub(super) const ROOT: NodeIndex = NodeIndex(0);

    /// The maximum valid node index.
    pub const MAX: NodeIndex = NodeIndex(NodeIdxInner::MAX);
}

impl ColIndex {
    /// The [`ColIndex`] used by the root node (`h`) to denote that it does not have a column header.
    pub(super) const NONE: ColIndex = ColIndex(ColIdxInner::MAX);

    /// The maximum valid column index.
    pub const MAX: ColIndex = ColIndex(ColIdxInner::MAX - 1);
}

impl RowIndex {
    /// The [`RowIndex`] used by the nodes in the header row to denote that they do not have a row header.
    pub(super) const NONE: RowIndex = RowIndex(RowIdxInner::MAX);

    /// The maximum valid row index.
    pub const MAX: RowIndex = RowIndex(RowIdxInner::MAX - 1);
}

/// An error that occurs when attempting to convert too large of a [`usize`] into a [`ColIndex`] or [`RowIndex`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexOverflowError {
    pub(super) kind: IndexOverflowKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum IndexOverflowKind {
    Nodes,
    Cols,
    Rows,
}

impl Display for IndexOverflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.kind {
            IndexOverflowKind::Nodes => "node",
            IndexOverflowKind::Rows => "row",
            IndexOverflowKind::Cols => "column",
        };
        write!(f, "index overflow occurred: {kind} index too large")
    }
}

impl Error for IndexOverflowError {}

macro_rules! index_conversions {
    ($wrapper:ident, $inner:ident, $error_kind:expr) => {
        impl $wrapper {
            pub const fn to_usize(self) -> usize {
                self.0 as usize
            }
        }

        impl TryFrom<usize> for $wrapper {
            type Error = IndexOverflowError;

            fn try_from(n: usize) -> Result<Self, Self::Error> {
                if n > (($wrapper::MAX).0 as usize) {
                    Err(IndexOverflowError { kind: $error_kind })
                } else {
                    Ok($wrapper(n as $inner))
                }
            }
        }

        impl From<$wrapper> for usize {
            fn from(index: $wrapper) -> usize {
                index.to_usize()
            }
        }
    };
}

index_conversions!(NodeIndex, NodeIdxInner, IndexOverflowKind::Nodes);
index_conversions!(RowIndex, RowIdxInner, IndexOverflowKind::Rows);
index_conversions!(ColIndex, ColIdxInner, IndexOverflowKind::Cols);
