use super::error::BuilderError;
use super::index::{ColIndex, NodeIndex, RowIndex};
use super::{ColHeader, Matrix, Node, RowHeader};


pub type BuilderResult<T> = Result<T, BuilderError>;

#[derive(Debug, Clone)]
pub struct Column {
    pub count: usize,
    pub optional: bool,
}

impl Column {
    /// Creates a new required column.
    pub const fn required() -> Self {
        Column { count: 1, optional: false }
    }

    /// Creates a new optional column.
    pub const fn optional() -> Self {
        Column { count: 1, optional: true }
    }

    /// Adjusts this column's _count_.
    pub const fn count(self, n: usize) -> Self {
        Self { count: n, ..self }
    }
}

#[derive(Debug, Clone)]
pub struct MatrixBuilder {
    col_headers: Box<[ColHeader]>,
    row_headers: Vec<RowHeader>,
    col_stack: Box<[NodeIndex]>,
    nodes: Vec<Node>,
}

impl MatrixBuilder {
    /// Gets the number of columns that were specified for this matrix builder.
    pub fn num_columns(&self) -> usize {
        self.col_headers.len()
    }

    /// Finish building this [matrix][Matrix].
    pub fn finish(self) -> Matrix {
        Matrix {
            col_headers: self.col_headers,
            row_headers: self.row_headers.into_boxed_slice(),
            nodes: self.nodes.into_boxed_slice(),
        }
    }

    /// Creates a new [`MatrixBuilder`] out of a list of column specifications.
    ///
    /// The order of these columns is important: they are later identified by their index.
    ///
    /// # Errors
    ///
    /// This function can fail if too many columns are specified. Currently, the limit is <code>[u32::MAX] - 1</code>.
    /// Not that the practical limit is much lower, since
    pub fn from_columns(columns: impl IntoIterator<Item = Column>) -> BuilderResult<MatrixBuilder> {
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
        for Column { count, optional } in columns {
            let row_idx = RowIndex::NONE;
            let col_idx = ColIndex::try_from(col_headers.len())?;
            let node_idx = NodeIndex::try_from(nodes.len())?;

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

            if !optional {
                // We point backwards (left) at the last required node; it points forwards (right) at us.
                node.left = prev_req;
                nodes[prev_req.index()].right = node_idx;
                prev_req = node_idx;
            }

            nodes.push(node);

            col_headers.push(ColHeader {
                count,
                choices: 0,
                index: col_idx,
                node: node_idx,
            });
        }

        // To finish up, we connect the start of the chain to the end of the chain:
        nodes[prev_req.index()].right = NodeIndex::ROOT;
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

    /// Adds a new row to the matrix.
    ///
    /// Each row is specified by listing the indices of the columns it contains.
    ///
    /// # Errors
    ///
    /// Internally, columns, rows, and nodes are all indexed using `u32` (to keep the size down). This function will
    /// return an error if the number of nodes (or rows) exceeds the maximum allowed size.
    ///
    /// # Panics
    ///
    /// This function panics in the event of an ill-defined list of columns indices:
    ///
    /// - The same column is specified more than once within the same row.
    /// - An index greater than or equal to [`Self::num_columns`] is specified.
    pub fn add_row(&mut self, column_indices: impl IntoIterator<Item = usize>) -> BuilderResult<()> {
        let num_cols = self.num_columns();
        let Self {
            col_headers,
            row_headers,
            col_stack,
            nodes,
        } = self;

        let row_idx = RowIndex::try_from(row_headers.len())?;
        row_headers.push(RowHeader {
            index: row_idx,
            #[cfg(debug_assertions)]
            chosen: false,
        });

        // Once we get to the end of the row, we will need to point the `right` pointer of the last node back around to
        // the first one, and vice versa.
        let mut first_idx = None;
        let mut prev_idx = None;
        for col_idx in column_indices {
            // 1. Is this column index in-bounds?
            // 2. Does this column already end with a node within this newly-added row?
            if col_idx >= num_cols {
                panic!("matrix row specified a column index out of bounds: index was {col_idx} for {num_cols} columns");
            } else if nodes[col_stack[col_idx].index()].row == row_idx {
                panic!("matrix row specified the same column index {col_idx} more than once");
            }

            let col_head = &mut col_headers[col_idx];
            let node_idx = NodeIndex::try_from(nodes.len())?;

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

            nodes[prev_up.index()].down = node_idx;
            col_stack[col_idx] = node_idx;

            prev_idx = Some(node_idx);
        }

        // If we inserted at least one node, then the last node of the row needs to now point back around to the first
        // node, and it needs to point left to the last one.
        if let Some((first, last)) = first_idx.zip(prev_idx) {
            nodes[last.index()].right = first;
            nodes[first.index()].left = last;
        } else {
            // If we *didn't* put anything in the row, we could technically pop the header off. But, if we're going to
            // allow accessing things with indices... we should probably avoid silently removing an index.
            /* row_headers.pop(); */
        }

        Ok(())
    }
}
