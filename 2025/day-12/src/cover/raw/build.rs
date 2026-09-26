use std::borrow::Borrow;

use super::error::MatrixOverflowError;
use super::*;

/// The first stage in building a DLX [`Matrix`]. Builds the list of columns in the matrix.
///
/// [TODO] Similar type-level docs as down below; consolidate docs from methods.
#[derive(Clone)]
pub struct HeaderBuilder {
    col_headers: Vec<ColHeader>,
    nodes: Vec<Node>,
    prev_required_idx: NodeIndex,
}

/// The second stage in building a DLX [`Matrix`]. Created by [`HeaderBuilder::finish_header`].
///
/// # Adding rows
///
/// Rows are specified by listing the indices of the columns within which they contain a node. Columns are indexed in
/// the order they were originally inserted into the [`HeaderBuilder`]. If the same column index appears more than once
/// in the same row, all but the first instance is ignored.
///
/// There are several options for adding rows to the matrix:
///
/// * [`try_push_row`] and [`try_push_rows`] are fallible. They return a `Result` in the event of an overflow.
/// * [`push_row`] and [`push_rows`] do not return a result.
/// * [`add_row`] and [`add_rows`] are builder-style versions of `push_row` and `push_rows` that take and return `self`
///   to allow for method chaining.
///
/// The `rows` (plural) methods accept a list of lists of column indices (a nested iterator of `usize`).
///
/// ## Overflows
///
/// Like columns, the current implementation indexes rows with [`u32`], and uses [`u32::MAX`] as a sentinel value. This
/// makes `u32::MAX - 1` the maximum allowed number of rows; an error will occur when attempting to push more than this
/// many rows. This number my change in the future.
///
/// Note however that the practical limit of rows is much lower. In addition to rows and columns, each **node** in the
/// DLX matrix is also indexed with a [`u32`] (and for nodes, `0` is used as a sentinel value for the root node). There
/// is one node for every row/column intersection. That is, the number of nodes will almost certainly overflow before
/// the number of rows.
///
/// [`try_push_row`]: Self::try_push_row
/// [`try_push_rows`]: Self::try_push_rows
/// [`push_row`]: Self::push_row
/// [`push_rows`]: Self::push_rows
/// [`add_row`]: Self::add_row
/// [`add_rows`]: Self::add_rows
#[derive(Clone)]
pub struct MatrixBuilder {
    col_headers: Box<[ColHeader]>,
    row_headers: Vec<RowHeader>,
    col_stack: Box<[NodeIndex]>,
    nodes: Vec<Node>,
}

impl Matrix {
    /// Creates a new [`HeaderBuilder`] to start constructing a [`Matrix`].
    pub fn builder() -> HeaderBuilder {
        HeaderBuilder::new()
    }
}

/// Specification for a column in a DLX [`Matrix`].
///
/// This is an unlabelled version of the higher-level [`Constraint`] type. See its documentation for more.
///
/// [`Constraint`]: crate::cover::build::Constraint
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Column {
    /// The number of times this column may/must be "covered" before it is removed from consideration.
    pub count: usize,

    /// Whether or not this column represents an optional or a required constraint.
    ///
    /// In a given problem, optional constraints **may** be met **at most** [`count`] times; required constraints
    /// **must** be met **exactly** [`count`] times.
    ///
    /// [`count`]: Self::count
    pub optional: bool,
}

#[allow(dead_code)]
impl Column {
    pub const fn required() -> Self {
        Column { count: 1, optional: false }
    }

    pub const fn optional() -> Self {
        Column { count: 1, optional: true }
    }

    pub const fn with_count(self, count: usize) -> Self {
        Column { count, ..self }
    }

    pub const fn to_required(self) -> Self {
        Column { optional: false, ..self }
    }

    pub const fn to_optional(self) -> Self {
        Column { optional: true, ..self }
    }

    /// Returns an iterator that repeats this criterion specification multiple times.
    pub fn repeat(self, n: usize) -> std::iter::RepeatN<Self> {
        std::iter::repeat_n(self, n)
    }
}

impl Default for Column {
    fn default() -> Self {
        Self::required().with_count(1)
    }
}

impl HeaderBuilder {
    /// Creates a new [`HeaderBuilder`] containing a single root node.
    pub fn new() -> Self {
        Self {
            nodes: vec![Node {
                column: ColIndex::NONE,
                row: RowIndex::NONE,
                up: NodeIndex::ROOT,
                down: NodeIndex::ROOT,
                left: NodeIndex::ROOT,
                right: NodeIndex::ROOT,
            }],
            prev_required_idx: NodeIndex::ROOT,
            col_headers: Vec::new(),
        }
    }

    /// Returns the number of columns currently in this builder.
    pub const fn num_columns(&self) -> usize {
        self.col_headers.len()
    }

    /// Reserves capacity for at least `additional` more columns.
    pub fn reserve(&mut self, additional: usize) {
        self.col_headers.reserve(additional);
        self.nodes.reserve(additional);
    }

    /// Attempts to add a new column to the [`Matrix`] header, returning an error if the number of columns overflows.
    ///
    /// The current implementation indexes columns with [`u32`], and uses [`u32::MAX`] as a sentinel value. This makes
    /// `u32::MAX - 1` the maximum number of columns allowed. This may change in the future.
    pub fn try_push_column(&mut self, column: Column) -> Result<(), MatrixOverflowError> {
        // Each step, we reach backwards to the last non-optional node and link it to the newest node.
        let row_idx = RowIndex::NONE;
        let col_idx = ColIndex::try_from(self.col_headers.len())?;
        let node_idx = NodeIndex::try_from(self.nodes.len()).unwrap(); // Cannot fail if `col_idx` succeeded

        // Create a node for an optional column first (i.e., it points to itself in al directions). Then, if it's
        // non-optional, update its left/right pointers.
        let mut node = Node {
            column: col_idx,
            row: row_idx,
            up: node_idx,
            down: node_idx,
            left: node_idx,
            right: node_idx,
        };

        if !column.optional {
            node.left = self.prev_required_idx;
            self.nodes[self.prev_required_idx.to_usize()].right = node_idx;
            self.prev_required_idx = node_idx;
        }

        self.nodes.push(node);
        self.col_headers.push(ColHeader {
            count: column.count,
            choices: 0,
            index: col_idx,
            node: node_idx,
        });

        Ok(())
    }

    /// Adds a new column to the [`Matrix`] header.
    ///
    /// See also [`push_columns`][Self::push_columns] to add multiple columns at once from an iterator.
    ///
    /// # Panics
    ///
    /// This function will panic if the number of columns exceeds the maximum allowed limit. The current implementation
    /// indexes columns with [`u32`], and uses [`u32::MAX`] as a sentinel value. This makes `u32::MAX - 1` the maximum
    /// number of columns allowed. This may change in the future.
    ///
    /// See [`try_push_column`][Self::try_push_column] for a version without panics.
    pub fn push_column(&mut self, column: Column) {
        match self.try_push_column(column) {
            Ok(()) => {},
            Err(err) => handle_overflow(err),
        }
    }

    /// Attempts to add multiple new columns to the [`Matrix`] header, returning an error if the number of columns
    /// overflows.
    ///
    /// The current implementation indexes columns with [`u32`], and uses [`u32::MAX`] as a sentinel value. This makes
    /// `u32::MAX - 1` the maximum number of columns allowed. This may change in the future.
    pub fn try_push_columns(&mut self, columns: impl IntoIterator<Item = Column>) -> Result<(), MatrixOverflowError> {
        let columns = columns.into_iter();
        let est_len = match columns.size_hint() {
            (_, Some(max)) => max,
            (min, None) => min,
        };

        self.reserve(est_len);
        for column in columns {
            self.try_push_column(column)?;
        }

        Ok(())
    }

    /// Adds multiple new columns to the [`Matrix`] header.
    ///
    /// See also [`push_column`][Self::push_column] to add a single new column.
    ///
    /// # Panics
    ///
    /// This function will panic if the number of columns exceeds the maximum allowed limit. The current implementation
    /// indexes columns with [`u32`], and uses [`u32::MAX`] as a sentinel value. This makes `u32::MAX - 1` the maximum
    /// number of columns allowed. This may change in the future.
    ///
    /// See [`try_push_columns`][Self::try_push_columns] for a version without panics.
    pub fn push_columns(&mut self, columns: impl IntoIterator<Item = Column>) {
        match self.try_push_columns(columns) {
            Ok(()) => {},
            Err(err) => handle_overflow(err),
        }
    }

    /// Builder-style version of [`Self::push_column`]. See there for details about panics.
    pub fn add_column(mut self, column: Column) -> Self {
        self.push_column(column);
        self
    }

    /// Builder-style version of [`Self::push_columns`]. See there for details about panics.
    pub fn add_columns(mut self, columns: impl IntoIterator<Item = Column>) -> Self {
        self.push_columns(columns);
        self
    }

    /// Finish adding columns to the [`Matrix`] header and move on to adding rows.
    pub fn finish_columns(self) -> MatrixBuilder {
        let Self {
            mut nodes,
            col_headers,
            prev_required_idx,
        } = self;

        // To finish up, connect the end of the chain of headers back to the start:
        nodes[prev_required_idx.to_usize()].right = NodeIndex::ROOT;
        nodes[0].left = prev_required_idx; // 0 = root node

        // To add rows, the MatrixBuilder maintains a list of which node was the most recently pushed for any given
        // column. Initially, this is simply the list of column header nodes: AKA, all the nodes we just pushed, but
        // excluding the root (from 1 to n).
        let col_stack = (0..col_headers.len())
            .into_iter()
            .map(|i| NodeIndex(i as u32 + 1))
            .collect::<Box<[NodeIndex]>>();

        MatrixBuilder {
            col_headers: col_headers.into_boxed_slice(),
            row_headers: Vec::with_capacity(8),
            col_stack,
            nodes,
        }
    }
}

impl MatrixBuilder {
    /// Returns the number of columns in this builder.
    pub const fn num_columns(&self) -> usize {
        self.col_headers.len()
    }

    /// Returns the number of rows currently in this builder.
    pub const fn num_rows(&self) -> usize {
        self.row_headers.len()
    }

    /// Reserves capacity for at least `additional` more rows.
    pub fn reserve(&mut self, additional: usize) {
        self.row_headers.reserve(additional);
        // Sadly, there isn't really a good way to determine how many nodes to reserve space for... just about any
        // heuristic we pick here will probably just get in the way of the built-in geometric growth.
        /* self.nodes.reserve(additional); */
    }

    /// Attempts to add a new row to the [`Matrix`], returning an error of an overflow occurs.
    ///
    /// See the [type-level documentation][MatrixBuilder] for information about overflows.
    ///
    /// # Panics
    ///
    /// This function will panic if one of the specified column indices is out of bounds (greater than
    /// [`Self::num_columns`]).
    pub fn try_push_row<R: Row>(&mut self, row: R) -> Result<(), MatrixOverflowError> {
        let new_row_idx = RowIndex::try_from(self.row_headers.len())?;
        self.row_headers.push(RowHeader {
            index: new_row_idx,
            #[cfg(debug_assertions)]
            chosen: false,
        });

        // Once we get to the end of the row, we will need to point the `right` pointer of the last node back around to
        // the first one, and vice versa.
        let mut first_idx = None;
        let mut prev_idx = None;
        for col_idx in row.column_indices() {
            // Check what the most recent node at the bottom of this column was.
            let prev_in_col = match self.col_stack.get(col_idx) {
                // Does the most recent node in this column already belong to this newest row? If so, a previous
                // iteration of this loop added it; this index is a duplicate.
                Some(&idx) if self.nodes[idx.to_usize()].row == new_row_idx => continue,
                Some(&idx) => idx,
                // Index out of bounds!
                None => panic!(
                    "matrix row specified a column index out of bounds (index was {col_idx} for {} columns)",
                    self.num_columns(),
                ),
            };

            let col_head = &mut self.col_headers[col_idx];
            let new_node_idx = NodeIndex::try_from(self.nodes.len())?;

            // - If we're the first thing in the row, then our left pointer is ourselves; otherwise, the previous node.
            //   - This line sets `first_idx` for later.
            // - Our right pointer is the tentative index of the next node after us; if we turn out to be the last thing
            //   in the row, this pointer will get fixed after the loop.
            let prev_left = prev_idx.unwrap_or_else(|| *first_idx.get_or_insert(new_node_idx));
            let next_right = NodeIndex::try_from(self.nodes.len() + 1).ok().or(first_idx).unwrap();

            self.nodes.push(Node {
                column: ColIndex(col_idx as u32), // Known to be in-bounds
                row: new_row_idx,
                up: prev_in_col,
                down: col_head.node, // Wrap around back up to top of column
                left: prev_left,
                right: next_right,
            });

            col_head.choices += 1;

            self.nodes[prev_in_col.to_usize()].down = new_node_idx;
            self.col_stack[col_idx] = new_node_idx;

            prev_idx = Some(new_node_idx);
        }

        // If we inserted at least one node, then the last node of the row needs to now point back around to the first
        // node, and it needs to point left to the last one.
        if let Some((first, last)) = first_idx.zip(prev_idx) {
            self.nodes[last.to_usize()].right = first;
            self.nodes[first.to_usize()].left = last;
        } else {
            // If we didn't put anything in the row, we can technically pop the header off: it will never be needed.
            // however, since we're going to allow accessing things using indices, we want to avoid silently removing an
            // index the user may think got added.
            /* row_headers.pop(); */
        }

        Ok(())
    }

    /// Adds a new row to the [`Matrix`].
    ///
    /// # Panics
    ///
    /// This function will panic if any of the specified column indices are out of bounds (greater than
    /// [`Self::num_columns`]) or if the number of rows/nodes grows beyond the maximum allowed value. Refer to [the
    /// type-level documentation][MatrixBuilder] for more information about overflows.
    pub fn push_row<R: Row>(&mut self, row: R) {
        match self.try_push_row(row) {
            Ok(()) => {},
            Err(err) => handle_overflow(err),
        }
    }

    /// Attempts to add multiple new rows to the [`Matrix`], returning an error if overflow occurs.
    ///
    /// See the [type-level documentation][MatrixBuilder] for information about overflows.
    ///
    /// # Panics
    ///
    /// This function will panic if any of the specified column indices in any of the rows are out of bounds (greater
    /// than [`Self::num_columns`]).
    pub fn try_push_rows<R: Row>(&mut self, rows: impl IntoIterator<Item = R>) -> Result<(), MatrixOverflowError> {
        let rows = rows.into_iter();
        let est_len = match rows.size_hint() {
            (_, Some(max)) => max,
            (min, None) => min,
        };

        self.reserve(est_len);
        for row in rows {
            self.try_push_row(row)?;
        }

        Ok(())
    }

    /// Adds multiple new rows to the [`Matrix`].
    ///
    /// # Panics
    ///
    /// This function will panic if any of the specified column indices in any of the rows are out of bounds (greater
    /// than [`Self::num_columns`]), or if the number of rows/nodes grows beyond the maximum allowed value. Refer to
    /// [the type-level documentation][MatrixBuilder] for more information about overflows.
    pub fn push_rows<R: Row>(&mut self, rows: impl IntoIterator<Item = R>) {
        match self.try_push_rows(rows) {
            Ok(()) => {},
            Err(err) => handle_overflow(err),
        }
    }

    /// Builder-style version of [`Self::push_row`]. See there for details about panics.
    pub fn add_row<R: Row>(mut self, row: R) -> Self {
        self.push_row(row);
        self
    }

    /// Builder-style version of [`Self::push_rows`]. See there for details about panics.
    pub fn add_rows<R: Row>(mut self, rows: impl IntoIterator<Item = R>) -> Self {
        self.push_rows(rows);
        self
    }

    /// Finish adding rows to and build the final [`Matrix`].
    pub fn build(self) -> Matrix {
        let Self {
            col_headers,
            row_headers,
            col_stack,
            mut nodes,
        } = self;

        // Once all rows have been placed, the last step is to loop down the column headers one last time and connect
        // them up to the things at the bottom of each column.
        for node_idx in col_stack {
            let node = &mut nodes[node_idx.to_usize()];
            let head_idx = col_headers[node.column.to_usize()].node;
            node.down = head_idx;
            nodes[head_idx.to_usize()].up = node_idx;
        }

        Matrix {
            col_headers,
            row_headers: row_headers.into_boxed_slice(),
            nodes: nodes.into_boxed_slice(),
        }
    }
}

/// Common place to write down the panic messages for matrix overflow errors.
#[inline(always)]
fn handle_overflow(err: MatrixOverflowError) -> ! {
    match err.kind() {
        super::error::MatrixOverflowKind::Rows => panic!("number of rows exceeded maximum allowed value"),
        super::error::MatrixOverflowKind::Cols => panic!("number of columns exceeded maximum allowed value"),
        super::error::MatrixOverflowKind::Nodes => panic!("number of nodes exceeded maximum allowed value"),
    }
}


/// Items that can specify a row in a DLX [`Matrix`].
///
/// This trait is automatically implemented for any iterator over `usize` or `&usize` (or anything else that implements
/// [`Borrow<usize>`]).
/* [TODO]: Rename to DLXRow? RowSpec? */
/* [TODO]: Could we make a trait abstracts "one row or multiple rows" and replace both `add_row` and `add_rows`? */
pub trait Row {
    /// Gets an iterator over the indices this row specifies.
    fn column_indices(self) -> impl Iterator<Item = usize>;
}

impl<I, T> Row for I
where
    I: IntoIterator<Item = T>,
    T: Borrow<usize>,
{
    fn column_indices(self) -> impl Iterator<Item = usize> {
        self.into_iter().map(|x| *x.borrow())
    }
}
