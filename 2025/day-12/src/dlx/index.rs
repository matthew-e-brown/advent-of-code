use super::error::BuilderError;


/// An index that refers to a specific column in a [`Matrix`][super::Matrix].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct ColIndex(u32);

/// An index that refers to a specific row in a [`Matrix`][super::Matrix].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct RowIndex(u32);

/// An index into [`super::Matrix::nodes`].
///
/// These are used as the main links to create the linked-lattice between the nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct NodeIndex(u32);


impl NodeIndex {
    /// The index of the root node. Always zero.
    pub(super) const ROOT: NodeIndex = NodeIndex(0);

    /// The maximum valid node index.
    pub const MAX: NodeIndex = NodeIndex(u32::MAX);

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl ColIndex {
    /// The [`ColIndex`] used by the root node (`h`) to denote that it does not have a column header.
    pub(super) const NONE: ColIndex = ColIndex(u32::MAX);

    /// The maximum valid column index.
    pub const MAX: ColIndex = ColIndex(u32::MAX - 1);

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl RowIndex {
    /// The [`RowIndex`] used by the nodes in the header row to denote that they do not have a row header.
    pub(super) const NONE: RowIndex = RowIndex(u32::MAX);

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
