#![allow(dead_code)]

use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;

use super::*;

/// A builder for a [DLX Matrix][Matrix].
#[derive(Debug, Clone)]
pub struct MatrixBuilder<R, C, State = Header<C>> {
    // We could have done this instead by having `Header<C>` and `Body<R, C>` be separate types. `Header::<C>::finish()`
    // could have then just been generic over `R` to return the right type of `Body<R, C>`. But that would decouple the
    // `R` on the `Body<R, C>` from the one on the call to `Matrix::<R, C>::builder()` method! That'd be kinda awkward.
    // This struct keeps all the type parameters together.
    state: State,
    _mark: PhantomData<(R, C)>,
}

/// [`MatrixBuilder`] state representing an incomplete list of column header specifications.
#[derive(Debug, Clone)]
pub struct Header<C> {
    specs: Vec<ColSpec<C>>,
}

/// [`MatrixBuilder`] state representing a matrix with a full header, whose rows have not yet completed construction.
#[derive(Debug, Clone)]
pub struct Body<R, C> {
    col_headers: Box<[ColHeader<C>]>,
    row_headers: Vec<RowHeader<R>>,
    nodes: Vec<Node>,
    /// Keeps track of which node indices were the most recent one in each column.
    stack: Box<[NodeIndex]>,
}

impl<C> Header<C> {
    pub fn new() -> Self {
        Self { specs: Vec::new() }
    }
}

impl<R, C, State> MatrixBuilder<R, C, State> {
    /// Constructs a new [`MatrixBuilder`] starting from the given state.
    pub fn from_state(state: State) -> Self {
        Self { state, _mark: PhantomData }
    }
}

impl<R, C> MatrixBuilder<R, C, Header<C>> {
    /// Creates a brand-new [`MatrixBuilder`] in its initial state: ready to build the header (columns) of the matrix.
    pub fn new() -> Self {
        Self {
            state: Header::new(),
            _mark: PhantomData,
        }
    }

    /// Gets a list of the column specifications that are already inside
    pub fn col_specs(&self) -> &[ColSpec<C>] {
        &self.state.specs[..]
    }

    pub fn col_specs_mut(&mut self) -> &mut [ColSpec<C>] {
        &mut self.state.specs[..]
    }

    /// Creates a new column with the given identifier. Use builder-style method chaining to further configure the
    /// column specification.
    pub fn add_column_named(&mut self, name: C) -> &mut ColSpec<C> {
        self.state.specs.push_mut(ColSpec::named(name))
    }

    /// Pushes an existing column specification into the list of columns.
    pub fn push_column(&mut self, spec: ColSpec<C>) -> &mut ColSpec<C> {
        self.state.specs.push_mut(spec)
    }

    /// Extends the list of column specifications with all of those in `specs`.
    ///
    /// A mutable slice is returned to allow further modifications of each entry.
    pub fn extend_columns(&mut self, specs: impl IntoIterator<Item = ColSpec<C>>) -> &mut [ColSpec<C>] {
        let i = self.state.specs.len();
        self.state.specs.extend(specs);
        let j = self.state.specs.len();
        &mut self.state.specs[i..j]
    }

    /// Complete the header of the matrix and move on to constructing the body.
    pub fn finish_header(self) -> MatrixBuilder<R, C, Body<R, C>> {
        MatrixBuilder {
            _mark: PhantomData,
            state: Body::from_header(self.state.specs),
        }
    }
}

impl<R, C: Default> MatrixBuilder<R, C, Header<C>> {
    /// Creates a new column with the given empty identifier.
    pub fn add_column(&mut self) -> &mut ColSpec<C> {
        self.add_column_named(C::default())
    }
}

/// Specifies a single column in a [DLX Matrix][Matrix].
///
/// This struct provides a builder-style API to customize columns after they have been inserted into a
/// <code>[MatrixBuilder]<R, C, [Header<C>]></code>.
#[derive(Debug, Clone)]
pub struct ColSpec<C> {
    /// The identifier for this column.
    pub name: C,
    /// The number of times this column's criteria must be covered for the column to be considered **completely**
    /// covered.
    pub count: usize,
    /// Whether or not this column must be complete for the overall matrix to be considered solved.
    pub required: bool,
}

impl<C: Default> Default for ColSpec<C> {
    /// Creates a new column specifier with the default identifier.
    fn default() -> Self {
        Self {
            name: C::default(),
            count: 1,
            required: true,
        }
    }
}

impl<C> ColSpec<C> {
    /// Creates a new column specifier with the given identifier.
    pub fn named(name: C) -> Self {
        Self { name, count: 1, required: true }
    }

    /// Sets how many times this column's criteria must be covered for the column is considered **complete.**
    ///
    /// The default value is 1. Columns with a count of 0 are ignored during matrix construction.
    pub fn count(&mut self, count: usize) -> &mut Self {
        self.count = count;
        self
    }

    /// Sets this column to be **optional:** if it is covered, conflicting rows will be removed, but it has no impact on
    /// whether or not the overall matrix is considered complete.
    pub fn optional(&mut self) -> &mut Self {
        self.required = false;
        self
    }

    /// Sets this column to be required: it must be covered exactly [`Self::count`] times before it is considered
    /// complete.
    pub fn required(&mut self) -> &mut Self {
        self.required = true;
        self
    }
}

impl<R, C> Body<R, C> {
    /// Begins the construction of a matrix's main body (its rows), given a list of column specifications.
    ///
    /// # Panics
    ///
    /// This function will panic if `columns` contains more columns than are supported.
    pub fn from_header(columns: impl Into<Box<[ColSpec<C>]>>) -> Self {
        Self::try_from_header(columns).unwrap()
    }

    /// Like [`Self::from_header`], but with the opportunity to catch potential integer overflows during matrix
    /// construction.
    pub fn try_from_header(columns: impl Into<Box<[ColSpec<C>]>>) -> Result<Self, IndexOverflowError> {
        const INITIAL_ROW_CAP: usize = 8;

        // Hopefully by using `Into<Box<[_]>>` we can just straight-up re-use the memory of the `Vec` from the header.
        let columns = columns.into();

        let mut col_headers = Vec::with_capacity(columns.len());
        let mut nodes = Vec::with_capacity(columns.len() * INITIAL_ROW_CAP);
        let mut stack = Vec::with_capacity(columns.len());

        // Create the root node and push it in:
        nodes.push(Node {
            column: ColIndex::NONE,
            row: RowIndex::NONE,
            up: NodeIndex::ROOT,
            down: NodeIndex::ROOT,
            left: NodeIndex::ROOT,
            right: NodeIndex::ROOT,
        });

        // Keep track of the index of the last (non-optional) node so that we can make the chain
        let mut prev_idx = NodeIndex::ROOT;
        for ColSpec { name, count, required } in columns {
            let row_idx: RowIndex = RowIndex::NONE;
            let col_idx: ColIndex = col_headers.len().try_into()?;
            let node_idx: NodeIndex = nodes.len().try_into()?;

            col_headers.push(ColHeader {
                name,
                count,
                choices: 0,
                head: node_idx,
            });

            // Create a node for an optional column first, then update its left/right if it's required.
            let node = nodes.push_mut(Node {
                column: col_idx,
                row: row_idx,
                up: node_idx,
                down: node_idx,
                left: node_idx,
                right: node_idx,
            });

            if required {
                // We point backwards at the last required node; it points back at us.
                node.left = prev_idx;
                nodes[prev_idx.as_usize()].right = node_idx;
                prev_idx = node_idx;
            }

            stack.push(node_idx);
        }

        // To finish up, grab the last required node and make it point to the root. Also make the root point back to it.
        nodes[prev_idx.as_usize()].right = NodeIndex::ROOT;
        nodes[0].left = prev_idx;

        Ok(Body {
            col_headers: col_headers.into_boxed_slice(),
            row_headers: Vec::with_capacity(INITIAL_ROW_CAP),
            nodes,
            stack: stack.into_boxed_slice(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexOverflowError {
    kind: IndexOverflowKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexOverflowKind {
    Nodes,
    Cols,
    Rows,
}

impl Display for IndexOverflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self.kind {
            IndexOverflowKind::Nodes => "nodes",
            IndexOverflowKind::Rows => "rows",
            IndexOverflowKind::Cols => "columns",
        };
        write!(f, "matrix construction failed: too many {kind} to fit into index type")
    }
}

impl Error for IndexOverflowError {}

macro_rules! index_conversions {
    ($wrapper:ident, $max_val:expr, $error_kind:expr) => {
        impl TryFrom<usize> for $wrapper {
            type Error = IndexOverflowError;

            fn try_from(n: usize) -> Result<Self, Self::Error> {
                if n > ($max_val as usize) {
                    Err(IndexOverflowError { kind: $error_kind })
                } else {
                    Ok($wrapper(n as u32))
                }
            }
        }

        impl $wrapper {
            fn as_usize(self) -> usize {
                self.0 as usize
            }
        }
    };
}

index_conversions!(NodeIndex, u32::MAX, IndexOverflowKind::Nodes);
index_conversions!(RowIndex, u32::MAX - 1, IndexOverflowKind::Rows);
index_conversions!(ColIndex, u32::MAX - 1, IndexOverflowKind::Cols);
