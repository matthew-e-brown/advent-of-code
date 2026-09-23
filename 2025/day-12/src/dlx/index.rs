use std::fmt::Debug;

#[cfg(feature = "serde-debug")]
use serde::Serialize;

use super::error::BuilderError;


/// An index that refers to a specific column in a [`Matrix`][super::Matrix].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-debug", derive(Serialize), serde(transparent))]
pub(super) struct ColIndex(pub u32);

/// An index that refers to a specific row in a [`Matrix`][super::Matrix].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-debug", derive(Serialize), serde(transparent))]
pub(super) struct RowIndex(pub u32);

/// An index into [`super::Matrix::nodes`].
///
/// These are used as the main links to create the linked-lattice between the nodes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-debug", derive(Serialize), serde(transparent))]
pub(super) struct NodeIndex(pub u32);


impl NodeIndex {
    /// The index of the root node. Always zero.
    pub const ROOT: NodeIndex = NodeIndex(0);

    /// The maximum valid node index.
    pub const MAX: NodeIndex = NodeIndex(u32::MAX);

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl ColIndex {
    /// The [`ColIndex`] used by the root node (`h`) to denote that it does not have a column header.
    pub const NONE: ColIndex = ColIndex(u32::MAX);

    /// The maximum valid column index.
    pub const MAX: ColIndex = ColIndex(u32::MAX - 1);

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl RowIndex {
    /// The [`RowIndex`] used by the nodes in the header row to denote that they do not have a row header.
    pub const NONE: RowIndex = RowIndex(u32::MAX);

    /// The maximum valid row index.
    pub const MAX: RowIndex = RowIndex(u32::MAX - 1);

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

macro_rules! index_conversions {
    ($($wrapper:ident as $inner:ty, $make_err:expr;)*) => {
        $(
            impl From<$wrapper> for usize {
                fn from(index: $wrapper) -> usize {
                    index.index()
                }
            }

            impl TryFrom<usize> for $wrapper {
                type Error = BuilderError;

                fn try_from(n: usize) -> Result<$wrapper, BuilderError> {
                    if n > (($wrapper::MAX).0 as usize) {
                        Err($make_err)
                    } else {
                        Ok($wrapper(n as $inner))
                    }
                }
            }
        )*
    };
}

index_conversions! {
    NodeIndex as u32, BuilderError::node_overflow();
    ColIndex as u32, BuilderError::col_overflow();
    RowIndex as u32, BuilderError::row_overflow();
}

// For debugging, always print indices with no indentation or any other special formatting.
impl Debug for NodeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Self::ROOT {
            write!(f, "NodeIndex::ROOT")
        } else {
            write!(f, "NodeIndex({})", self.0)
        }
    }
}

impl Debug for ColIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Self::NONE {
            write!(f, "ColIndex::NONE")
        } else {
            write!(f, "ColIndex({})", self.0)
        }
    }
}

impl Debug for RowIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == Self::NONE {
            write!(f, "RowIndex::NONE")
        } else {
            write!(f, "RowIndex({})", self.0)
        }
    }
}
