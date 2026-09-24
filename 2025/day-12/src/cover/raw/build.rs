use self::error::BuildError;
use super::super::build::Constraint;
use super::*;

/// Builder for a [`DLXMatrix`].
#[derive(Clone)]
pub struct DLXBuilder {
    col_headers: Box<[ColHeader]>,
    row_headers: Vec<RowHeader>,
    col_stack: Box<[NodeIndex]>,
    nodes: Vec<Node>,
}

impl DLXBuilder {
    /// Creates a new builder for a raw [`DLXMatrix`] with the specified number of constraints.
    ///
    /// Use this method for simpler problems with _n_ required constraints. All constraints will have a cover-count of 1
    /// by default. For more control, provide an iterator to [`DLXBuilder::from_constraints`].
    ///
    /// # Panics
    ///
    /// This function will panic if too many columns (currently <code>[u32::MAX] - 1</code>) are specified. To create a
    /// new [`DLXBuilder`] fallibly, see [`DLXBuilder::try_from_constraints`].
    pub fn new(num_constraints: usize) -> Self {
        Self::from_constraints(Constraint::Required(1).repeat(num_constraints))
    }

    /// Creates a new builder for a raw [`DLXMatrix`] with the specified number of required and optional constraints.
    ///
    /// Use this method for simpler problems with _n_ required constraints followed by _m_ optional constraints. All
    /// constraints will have a cover-count of 1 by default. For more control, provide an iterator to
    /// [`DLXBuilder::from_constraints`].
    ///
    /// # Panics
    ///
    /// This function will panic if the total number of columns is too large. The limit is currently <code>[u32::MAX] -
    /// 1</code>. To create a new [`DLXBuilder`] fallibly, see [`DLXBuilder::try_from_constraints`].
    pub fn new_with_optional(num_required: usize, num_optional: usize) -> Self {
        let req = Constraint::Required(1).repeat(num_required);
        let opt = Constraint::Optional(1).repeat(num_optional);
        Self::from_constraints(req.chain(opt))
    }

    /// Creates a new builder for a raw [`DLXMatrix`] with the specified constraints in the specified order.
    ///
    /// # Panics
    ///
    /// This function will panic if too many columns are specified. Currently, the limit is <code>[u32::MAX] - 1</code>.
    ///
    /// See [`try_from_constraints`][Self::try_from_constraints] for a fallible version of this method.
    pub fn from_constraints(constraints: impl IntoIterator<Item = Constraint>) -> Self {
        Self::try_from_constraints(constraints).unwrap()
    }

    /// Creates a new builder for a raw [`DLXMatrix`] with the specified constraints in the specified order.
    ///
    /// # Errors
    ///
    /// This function will fail if too many columns are specified. Currently, the limit is <code>[u32::MAX] - 1</code>.
    pub fn try_from_constraints(constraints: impl IntoIterator<Item = Constraint>) -> Result<Self, BuildError> {
        let mut col_headers = Vec::new();
        let mut nodes = Vec::new();

        // Start the list of nodes off with the root node, which is defined to live at index zero:
        nodes.push(Node {
            column: ColIndex::NONE,
            row: RowIndex::NONE,
            up: NodeIndex::ROOT,
            down: NodeIndex::ROOT,
            left: NodeIndex::ROOT,
            right: NodeIndex::ROOT,
        });

        // Each step, we reach backwards to the last non-optional node and link it to the newest node.
        let mut prev_req = NodeIndex::ROOT; // root node = index 0
        for constraint in constraints {
            let row_idx = RowIndex::NONE;
            let col_idx = ColIndex::try_from(col_headers.len()).map_err(BuildError::too_many_cols)?;
            let node_idx = NodeIndex::try_from(nodes.len()).map_err(BuildError::too_many_nodes)?;

            // Create a node for an optional column first (points to itself in all directions). Then, if it's
            // non-optional, update its left/right.
            let mut node = Node {
                column: col_idx,
                row: row_idx,
                up: node_idx,
                down: node_idx,
                left: node_idx,
                right: node_idx,
            };

            if constraint.is_required() {
                // We point backwards (left) at the last required node; it points forwards (right) at us.
                node.left = prev_req;
                nodes[prev_req.to_usize()].right = node_idx;
                prev_req = node_idx;
            }

            nodes.push(node);

            col_headers.push(ColHeader {
                count: constraint.count(),
                choices: 0,
                index: col_idx,
                node: node_idx,
            });
        }

        // To finish up, we connect the start of the chain to the end of the chain:
        nodes[prev_req.to_usize()].right = NodeIndex::ROOT;
        nodes[0].left = prev_req; // 0 = root node

        // To add rows, the builder needs to maintain a list of which node was the most recently pushed for any given
        // column. Initially, this is simply the list of column header nodes: AKA, all the nodes we just pushed, minus
        // the root.
        let col_stack = (1..nodes.len())
            .into_iter()
            .map(|i| NodeIndex::try_from(i).expect("indices up to `nodes.len()` are known to fit into NodeIndex"))
            .collect();

        Ok(Self {
            col_headers: col_headers.into_boxed_slice(),
            row_headers: Vec::with_capacity(8),
            col_stack,
            nodes,
        })
    }

    /// Builder-style version of [`push_row`][Self::push_row].
    pub fn row(mut self, constraint_indices: impl IntoIterator<Item = usize>) -> Self {
        self.push_row(constraint_indices);
        self
    }

    /// Builder-style version of [`push_rows`][Self::push_rows].
    pub fn rows<R: IntoIterator<Item = usize>>(mut self, rows: impl IntoIterator<Item = R>) -> Self {
        self.push_rows(rows);
        self
    }

    /// Pushes a new row into the [`DLXMatrix`].
    ///
    /// See [`try_push_row`][Self::try_push_row] for details about panics and errors.
    pub fn push_row(&mut self, constraint_indices: impl IntoIterator<Item = usize>) {
        if let Err(err) = self.try_push_row(constraint_indices) {
            panic!("{err}");
        }
    }

    /// Pushes a series of row into the [`DLXMatrix`].
    ///
    /// See [`try_push_row`][Self::try_push_row] for details about panics and errors.
    pub fn push_rows<R: IntoIterator<Item = usize>>(&mut self, rows: impl IntoIterator<Item = R>) {
        for row in rows {
            self.push_row(<R as IntoIterator>::into_iter(row));
        }
    }

    /// Adds a new row to the [`DLXMatrix`].
    ///
    /// Each row represents a "choice" in the cover problem. Rows are specified by listing the indices of the
    /// constraints (columns) they contain.
    ///
    /// # Errors
    ///
    /// Columns (constraints), rows (choices), and nodes (their intersections) are indexed using `u32`s internally
    /// instead of `usize`, to reduce memory size. An error will occur if the number of rows or nodes exceeds the
    /// maximum allowed size.
    ///
    /// # Panics
    ///
    /// This function panics in the event of an ill-defined list of constraint/column indices:
    ///
    /// - The same column is specified more than once within the same row.
    /// - An index greater than or equal to [`Self::num_columns`] is specified.
    pub fn try_push_row(&mut self, constraint_indices: impl IntoIterator<Item = usize>) -> Result<(), BuildError> {
        let num_cols = self.col_headers.len();
        let Self {
            col_headers,
            row_headers,
            col_stack,
            nodes,
        } = self;

        let row_idx = RowIndex::try_from(row_headers.len()).map_err(BuildError::too_many_rows)?;
        row_headers.push(RowHeader {
            index: row_idx,
            #[cfg(debug_assertions)]
            chosen: false,
        });

        // Once we get to the end of the row, we will need to point the `right` pointer of the last node back around to
        // the first one, and vice versa.
        let mut first_idx = None;
        let mut prev_idx = None;
        for col_idx in constraint_indices {
            // 1. Is this column index in-bounds?
            // 2. Does this column already end with a node within this newly-added row?
            if col_idx >= num_cols {
                panic!("matrix row specified a column index out of bounds: index was {col_idx} for {num_cols} columns");
            } else if nodes[col_stack[col_idx].to_usize()].row == row_idx {
                panic!("matrix row specified the same column index {col_idx} more than once");
            }

            let col_head = &mut col_headers[col_idx];
            let node_idx = NodeIndex::try_from(nodes.len()).map_err(BuildError::too_many_nodes)?;

            // - The node above us is simply the most recent node in this column.
            // - If we're the first thing in the row, then our left pointer is ourselves; otherwise, the previous node.
            // - Our right pointer is the tentative index of the next node after us; if we turn out to be the last thing
            //   in the row, this pointer will get fixed after the loop.
            let prev_up = col_stack[col_idx];
            let prev_left = prev_idx.unwrap_or_else(|| *first_idx.get_or_insert(node_idx));
            let next_right = NodeIndex::try_from(nodes.len() + 1).ok().or(first_idx).unwrap();

            nodes.push(Node {
                column: ColIndex::try_from(col_idx).unwrap(), // Unwrapping is fine: already checked against num_cols
                row: row_idx,
                up: prev_up,
                down: col_head.node,
                left: prev_left,
                right: next_right,
            });

            col_head.choices += 1;

            nodes[prev_up.to_usize()].down = node_idx;
            col_stack[col_idx] = node_idx;

            prev_idx = Some(node_idx);
        }

        // If we inserted at least one node, then the last node of the row needs to now point back around to the first
        // node, and it needs to point left to the last one.
        if let Some((first, last)) = first_idx.zip(prev_idx) {
            nodes[last.to_usize()].right = first;
            nodes[first.to_usize()].left = last;
        } else {
            // If we didn't put anything in the row, we can technically pop the header off: it will never be needed.
            // however, since we're going to allow accessing things using row indices, we avoid silently removing an
            // index.
            /* row_headers.pop(); */
        }

        Ok(())
    }

    pub fn finish(self) -> DLXMatrix {
        // Once all rows have been placed, the last step is to loop down the column headers one last time and connect
        // them up to the things at the bottom of each column.
        let Self {
            col_headers,
            row_headers,
            col_stack,
            mut nodes,
        } = self;

        for node_idx in col_stack {
            let node = &mut nodes[node_idx.to_usize()];
            let head = col_headers[node.column.to_usize()].node;
            node.down = head;
            nodes[head.to_usize()].up = node_idx;
        }

        DLXMatrix {
            col_headers,
            row_headers: row_headers.into_boxed_slice(),
            nodes: nodes.into_boxed_slice(),
        }
    }
}

/// Errors that may occur during building of a [`DLXMatrix`].
#[rustfmt::skip]
pub mod error {
    use super::super::index::{ColOverflowError, NodeOverflowError, RowOverflowError};

    #[derive(Debug, Clone, Copy, thiserror::Error)]
    #[error("could not build matrix: {kind}")]
    pub struct BuildError {
        kind: BuildErrorKind,
    }

    #[derive(Debug, Clone, Copy, thiserror::Error)]
    pub enum BuildErrorKind {
        #[error(transparent)] TooManyRows(RowOverflowError),
        #[error(transparent)] TooManyCols(ColOverflowError),
        #[error(transparent)] TooManyNodes(NodeOverflowError),
    }

    impl BuildError {
        pub(super) fn too_many_rows(inner: RowOverflowError) -> Self {
            Self { kind: BuildErrorKind::TooManyRows(inner) }
        }

        pub(super) fn too_many_cols(inner: ColOverflowError) -> Self {
            Self { kind: BuildErrorKind::TooManyCols(inner) }
        }

        pub(super) fn too_many_nodes(inner: NodeOverflowError) -> Self {
            Self { kind: BuildErrorKind::TooManyNodes(inner) }
        }
    }
}
