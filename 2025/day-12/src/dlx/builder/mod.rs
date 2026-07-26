pub mod error;

use std::collections::HashSet;

#[allow(unused)]
pub use self::error::{BuilderError, BuilderErrorKind};
use super::*;


pub type BuilderResult<T> = Result<T, BuilderError>;

/// A builder for a [DLX Matrix][Matrix].
#[derive(Debug, Clone)]
pub struct MatrixBuilder<State = Header> {
    // We want all methods to be infallible up until the very end, when they run `.build()`. So, we keep track of the
    // result, and let them keep running builder methods until the end. Only then do we actually return the error.
    state: Result<State, BuilderError>,
}

/// [`MatrixBuilder`] state representing a builder that has not yet finished specifying its columns.
#[derive(Debug, Clone)]
pub struct Header {
    columns: Vec<Column>,
}

/// [`MatrixBuilder`] state representing a builder that is in the process of creating its rows.
#[derive(Debug, Clone)]
pub struct Body {
    /// During row insertion, keeps track of which indices have already been added to the current row (just so we can
    /// catch errors and panic). This is kept around between insertions to avoid having to reallocate for every row.
    col_idx_set: HashSet<ColIndex>,
    col_headers: Box<[ColHeader]>,
    row_headers: Vec<RowHeader>,
    nodes: Vec<Node>,
    /// Keeps track of the indices of the most recently added nodes in each of the columns.
    stack: Box<[NodeIndex]>,
}

/// Describes a column to be added to a [DLX Matrix][Matrix].
#[derive(Debug, Clone)]
pub struct Column {
    /// The number of times this column's criteria must be covered for the column to be considered **completely**
    /// covered.
    pub count: usize,
    /// Whether or not this column must be complete for the overall matrix to be considered solved.
    pub required: bool,
}

impl Column {
    /// Creates a new required column.
    ///
    /// A required column is one which *must* be covered before the [DLX Matrix][Matrix] can be considered solved.
    pub const fn required() -> Self {
        Column { count: 1, required: true }
    }

    /// Creates a new column optional column.
    ///
    /// An optional column does not need to be covered for the [DLX Matrix][Matrix] to be considered solved; however,
    /// when one is covered, conflicting rows are removed from consideration.
    pub const fn optional() -> Self {
        Column { count: 1, required: false }
    }

    /// Adjusts the _count_ of this column. This requires the column to be covered multiple times before it is actually
    /// considered "covered" for the purposes of a [DLX Matrix][Matrix].
    pub const fn count(self, n: usize) -> Self {
        Self { count: n, ..self }
    }
}

impl MatrixBuilder<Header> {
    /// Creates a new [`MatrixBuilder`].
    pub fn new() -> Self {
        Self {
            state: Ok(Header { columns: Vec::new() }),
        }
    }

    /// Pushes an existing column specification into the list of columns.
    pub fn column(&mut self, column: Column) -> &mut Self {
        if let Ok(Header { columns }) = &mut self.state {
            columns.push(column);
            if let Err(e) = ColIndex::try_from(columns.len()) {
                self.state = Err(e.into())
            }
        }

        self
    }

    /// Extends the list of column specifications with all of those in `specs`.
    pub fn columns(&mut self, columns: impl IntoIterator<Item = Column>) -> &mut Self {
        if let Ok(Header { columns: col_list }) = &mut self.state {
            col_list.extend(columns);
            if let Err(error) = ColIndex::try_from(col_list.len()) {
                self.state = Err(error.into())
            }
        }

        self
    }

    /// Complete the header of the matrix and move on to constructing the body.
    pub fn finish_columns(self) -> MatrixBuilder<Body> {
        MatrixBuilder {
            state: self.state.and_then(|Header { columns }| try_finish_columns(columns)),
        }
    }
}

impl MatrixBuilder<Body> {
    /// Adds a row to this matrix, made of one node in each of the indicated columns.
    ///
    /// # Panics
    ///
    /// This function panics if:
    ///
    /// - Any indices are out of range of the number of columns; or
    /// - Any indices are provided more than once per row.
    pub fn row<I, C>(&mut self, column_indices: I)
    where
        I: IntoIterator<Item = C>,
        C: TryInto<ColIndex>,
        BuilderError: From<C::Error>,
    {
        if let Ok(body) = &mut self.state {
            if let Err(error) = try_add_row(body, column_indices) {
                self.state = Err(error);
            }
        }
    }

    /// Complete the building of the [DLX Matrix][Matrix].
    pub fn build(self) -> BuilderResult<Matrix> {
        self.state.map(|body| Matrix {
            nodes: body.nodes.into_boxed_slice(),
            col_headers: body.col_headers,
            row_headers: body.row_headers.into_boxed_slice(),
        })
    }
}

fn try_finish_columns(columns: Vec<Column>) -> BuilderResult<Body> {
    MatrixBuilder::new().finish_columns().row([ColIndex::NONE]);

    const INITIAL_ROW_CAP: usize = 8;

    let num_columns = columns.len();
    let mut col_headers = Vec::with_capacity(num_columns);
    let mut nodes = Vec::with_capacity(num_columns * INITIAL_ROW_CAP);
    let mut stack = Vec::with_capacity(num_columns);

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
    for Column { count, required } in columns {
        let row_idx = RowIndex::NONE;
        let col_idx = ColIndex::try_from(col_headers.len())?;
        let node_idx = NodeIndex::try_from(nodes.len())?;

        col_headers.push(ColHeader {
            count,
            choices: 0,
            index: col_idx,
            node: node_idx,
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
            nodes[prev_idx.to_usize()].right = node_idx;
            prev_idx = node_idx;
        }

        stack.push(node_idx);
    }

    // To finish up, grab the last required node and make it point to the root. Also make the root point back to it.
    nodes[prev_idx.to_usize()].right = NodeIndex::ROOT;
    nodes[0].left = prev_idx;

    Ok(Body {
        col_idx_set: HashSet::with_capacity(num_columns),
        col_headers: col_headers.into_boxed_slice(),
        row_headers: Vec::with_capacity(INITIAL_ROW_CAP),
        nodes,
        stack: stack.into_boxed_slice(),
    })
}

fn try_add_row<I, C>(body: &mut Body, column_indices: I) -> BuilderResult<()>
where
    I: IntoIterator<Item = C>,
    C: TryInto<ColIndex>,
    BuilderError: From<C::Error>, // Needs to be specified in terms of `From` because that lets us use `?`
{
    #[rustfmt::skip]
    let Body { col_idx_set, col_headers, row_headers, nodes, stack } = body;

    col_idx_set.clear();

    let row_idx = RowIndex::try_from(row_headers.len())?;
    row_headers.push(RowHeader {
        index: row_idx,
        #[cfg(debug_assertions)]
        in_solution: false,
    });

    // Once we get to the end of the row, we will need to point the `right` pointer of the last node back at the first
    // one we insert.
    let mut first_idx = None;
    let mut prev_idx = None;
    for col_idx in column_indices {
        let col_idx = col_idx.try_into()?; // Index of the column header.

        if col_idx.to_usize() >= col_headers.len() {
            panic!("matrix row specified a column index out of bounds: {}", col_idx.to_usize());
        } else if !col_idx_set.insert(col_idx) {
            panic!("matrix row specified the same column index {} more than once", col_idx.to_usize());
        }

        let col_head = &mut col_headers[col_idx.to_usize()]; // The column's head node.
        let node_idx = NodeIndex::try_from(nodes.len())?; // Index of this new node.

        // - What was the last node in this column? That gives us the one above us.
        // - If we're the first thing in the row, then our left pointer is be ourselves; otherwise, the previous node.
        // - Our right pointer should be the next index up.
        // - If we are the last thing in the row, then our pointers will get fixed up after the loop.
        let prev_up = stack[col_idx.to_usize()];
        let prev_left = prev_idx.unwrap_or_else(|| *first_idx.get_or_insert(node_idx));
        let next_right = NodeIndex::try_from(nodes.len() + 1).ok().or(first_idx).unwrap();

        nodes.push(Node {
            column: col_idx,
            row: row_idx,
            up: prev_up,
            down: col_head.node,
            left: prev_left,
            right: next_right,
        });

        col_head.choices += 1;

        nodes[prev_up.to_usize()].down = node_idx;
        stack[col_idx.to_usize()] = node_idx;

        prev_idx = Some(node_idx);
    }

    // If we inserted at least one node, then the last node of the row needs to now point back around to the first node,
    // and it needs to point left to the last one.
    if let Some((first, last)) = first_idx.zip(prev_idx) {
        nodes[last.to_usize()].right = first;
        nodes[first.to_usize()].left = last;
    } else {
        // If we *didn't* put anything in the row, though, we can pop the header off.
        row_headers.pop();
    }

    Ok(())
}
