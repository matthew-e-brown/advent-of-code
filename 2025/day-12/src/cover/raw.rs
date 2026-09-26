#![allow(dead_code)]

pub mod build;
pub mod error;
#[cfg(test)] mod tests;

/// A matrix that implements a modified version of Donald Knuth's _Algorithm X._
///
/// Rows and columns are identified by their indices.
#[derive(Clone, Debug)]
pub struct Matrix {
    nodes: Box<[Node]>,
    col_headers: Box<[ColHeader]>,
    row_headers: Box<[RowHeader]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    column: ColIndex,
    row: RowIndex,
    up: NodeIndex,
    down: NodeIndex,
    left: NodeIndex,
    right: NodeIndex,
}

#[derive(Debug, Clone)]
struct RowHeader {
    /// This row's index.
    index: RowIndex,
    /// Whether or not this row is currently being used in the solution. Used for debugging.
    #[cfg(debug_assertions)]
    chosen: bool,
}

#[derive(Debug, Clone)]
struct ColHeader {
    /// This column's index.
    index: ColIndex,
    /// The number of rows still remaining in this column.
    choices: usize,
    /// The remaining number of times this column must be covered before it is considered done.
    count: usize,
    /// The index of the head node of this column's list of nodes.
    node: NodeIndex,
}

impl ColHeader {
    /// Gets the index of this column.
    pub fn index(&self) -> ColIndex {
        self.index
    }
}

impl RowHeader {
    /// Gets the index of this row.
    pub fn index(&self) -> RowIndex {
        self.index
    }

    #[cfg(debug_assertions)]
    fn assert_unchosen(&mut self) {
        match self.chosen {
            true => panic!("attempted to use the same row twice in the same solution"),
            false => self.chosen = true,
        };
    }

    #[cfg(debug_assertions)]
    fn assert_chosen(&mut self) {
        match self.chosen {
            false => panic!("attempted to restore the same row more than once"),
            true => self.chosen = false,
        }
    }
}

use std::fmt::Debug;

/// An index into [`Matrix::nodes`].
///
/// These are used as the main links to create the linked-lattice between the nodes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct NodeIndex(pub u32);

impl NodeIndex {
    /// The index of the root node. Always zero.
    pub const ROOT: NodeIndex = NodeIndex(0);

    /// The maximum valid node index.
    pub const MAX: NodeIndex = NodeIndex(u32::MAX);

    pub const fn to_usize(self) -> usize {
        self.0 as usize
    }
}

/// An index that refers to a specific column in a [`Matrix`][Matrix].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct ColIndex(pub u32);

impl ColIndex {
    /// The [`ColIndex`] used by the root node (`h`) to denote that it does not have a column header.
    pub const NONE: ColIndex = ColIndex(u32::MAX);

    /// The maximum valid column index.
    pub const MAX: ColIndex = ColIndex(u32::MAX - 1);

    pub const fn to_usize(self) -> usize {
        self.0 as usize
    }
}

/// An index that refers to a specific row in a [`Matrix`][Matrix].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct RowIndex(pub u32);

impl RowIndex {
    /// The [`RowIndex`] used by the nodes in the header row to denote that they do not have a row header.
    pub const NONE: RowIndex = RowIndex(u32::MAX);

    /// The maximum valid row index.
    pub const MAX: RowIndex = RowIndex(u32::MAX - 1);

    pub const fn to_usize(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, thiserror::Error)]
#[error("node index too high: overflow occurred")]
pub struct NodeOverflowError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, thiserror::Error)]
#[error("column index too high: overflow occurred")]
pub struct ColOverflowError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, thiserror::Error)]
#[error("rows index too high: overflow occurred")]
pub struct RowOverflowError;

macro_rules! index_conversions {
    ($($wrapper:ident as $inner:ty, $err_name:ident;)*) => {
        $(
            impl From<$wrapper> for usize {
                fn from(index: $wrapper) -> usize {
                    index.to_usize()
                }
            }

            impl TryFrom<usize> for $wrapper {
                type Error = $err_name;

                fn try_from(n: usize) -> Result<$wrapper, $err_name> {
                    if n > (($wrapper::MAX).0 as usize) {
                        Err($err_name)
                    } else {
                        Ok($wrapper(n as $inner))
                    }
                }
            }
        )*
    };
}

index_conversions! {
    NodeIndex as u32, NodeOverflowError;
    ColIndex as u32, ColOverflowError;
    RowIndex as u32, RowOverflowError;
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


impl Matrix {
    pub fn search(&mut self) -> Option<Vec<usize>> {
        let mut solution = Vec::new();
        if self.search_recursive(&mut solution) {
            Some(solution)
        } else {
            None
        }
    }

    pub fn prepared_search<F>(&mut self, mut f: F) -> Option<Vec<usize>>
    where
        F: FnMut(usize) -> usize,
    {
        let mut covered_columns = Vec::new();
        for i in 0..self.col_headers.len() {
            let header = &mut self.col_headers[i];
            header.count = f(i);
            if header.count == 0 {
                let index = header.index;
                self.cover_column(index);
                covered_columns.push(index);
            }
        }

        let mut solution = Vec::new();
        let solution = if self.search_recursive(&mut solution) {
            Some(solution)
        } else {
            None
        };

        // Uncover all the columns we covered, but in the other direction:
        for index in covered_columns.into_iter().rev() {
            self.uncover_column(index);
        }

        solution
    }


    pub fn set_column_counts<F>(&mut self, mut f: F)
    where
        F: FnMut(usize) -> usize,
    {
        for i in 0..self.col_headers.len() {
            let col_idx = ColIndex::try_from(i).unwrap(); // Cannot be more than ColIndex::MAX headers

            let old_count = self.col_headers[i].count;
            let new_count = f(i);

            match (old_count == 0, new_count == 0) {
                // The count wasn't zero, but is now: column should be covered.
                (false, true) => {
                    self.cover_column(col_idx);
                },
                // The count was zero, but is no longer: column should be un-covered.
                (true, false) => {
                    self.uncover_column(col_idx);
                },
                _ => {},
            }

            self.col_headers[i].count = new_count;
        }
    }
}

enum ColumnResult {
    Next(ColIndex),
    SearchSuccess,
    SearchFailure,
}

impl Matrix {
    fn root(&self) -> &Node {
        self.node(NodeIndex::ROOT)
    }

    fn node(&self, i: NodeIndex) -> &Node {
        &self.nodes[i.to_usize()]
    }

    fn node_mut(&mut self, i: NodeIndex) -> &mut Node {
        &mut self.nodes[i.to_usize()]
    }

    fn column(&self, i: ColIndex) -> &ColHeader {
        &self.col_headers[i.to_usize()]
    }

    fn column_mut(&mut self, i: ColIndex) -> &mut ColHeader {
        &mut self.col_headers[i.to_usize()]
    }

    fn row_header(&self, i: RowIndex) -> &RowHeader {
        &self.row_headers[i.to_usize()]
    }

    fn row_header_mut(&mut self, i: RowIndex) -> &mut RowHeader {
        &mut self.row_headers[i.to_usize()]
    }

    fn column_for_node(&self, i: NodeIndex) -> &ColHeader {
        self.column(self.node(i).column)
    }

    fn column_for_node_mut(&mut self, i: NodeIndex) -> &mut ColHeader {
        self.column_mut(self.node(i).column)
    }
}

impl Matrix {
    /// Removes the given node from its row by modifying its left/right siblings to point to one another.
    fn remove_left_right(&mut self, index: NodeIndex) {
        let left = self.node(index).left;
        let right = self.node(index).right;
        self.node_mut(right).left = left;
        self.node_mut(left).right = right;
    }

    /// Removes the given node from its column by modifying its up/down siblings to point to one another.
    fn remove_up_down(&mut self, index: NodeIndex) {
        let u = self.node(index).up;
        let d = self.node(index).down;
        self.node_mut(d).up = u;
        self.node_mut(u).down = d;
    }

    /// Restores the given node to its row list by modifying its left/right siblings to once again point to it.
    fn restore_left_right(&mut self, index: NodeIndex) {
        let l = self.node(index).left;
        let r = self.node(index).right;
        self.node_mut(r).left = index;
        self.node_mut(l).right = index;
    }

    /// Restores the given node to its column list by modifying its up/down siblings to once again point to it.
    fn restore_up_down(&mut self, index: NodeIndex) {
        let u = self.node(index).up;
        let d = self.node(index).down;
        self.node_mut(d).up = index;
        self.node_mut(u).down = index;
    }

    /// Finds the column with the smallest _branching factor_ to cover next.
    fn select_next_column(&self) -> ColumnResult {
        // Look for the column with the smallest branching factor.
        let mut min = None;
        let mut idx = self.root().right;

        while idx != NodeIndex::ROOT {
            let col_index = self.node(idx).column;
            let col_header = self.column(col_index);

            // If there are any columns that need to be covered some `n` more times, but which do not actually have `n`
            // more rows to choose from, then this entire branch of the search is invalid, and can be pruned.
            if col_header.choices < col_header.count {
                return ColumnResult::SearchFailure;
            }

            let branch_factor = col_header.count * col_header.choices;
            if min.is_none_or(|(_, min_bf)| branch_factor < min_bf) {
                min = Some((col_index, branch_factor));
            }

            idx = self.node(idx).right;
        }

        // If there are no more columns, we have covered all the required criteria. We're done!
        match min {
            Some((col, _)) => ColumnResult::Next(col),
            None => ColumnResult::SearchSuccess,
        }
    }

    // Returns `true` if this branch of the recursive search process identified a solution. Returns `false` if no
    // solution could be found before running out of valid rows.
    fn search_recursive(&mut self, solution: &mut Vec<usize>) -> bool {
        // 1. Find the next column we wish to attempt to cover.
        let col: ColIndex = match self.select_next_column() {
            ColumnResult::Next(col) => col,
            ColumnResult::SearchSuccess => return true,
            ColumnResult::SearchFailure => return false,
        };

        // 2. Start picking rows for this column.
        let col_head_idx: NodeIndex = self.column(col).node;
        let mut r: NodeIndex = self.node(col_head_idx).down;
        while r != col_head_idx {
            solution.push(self.node(r).row.into());

            // 3. Remove this row from all its columns. They can no longer see the row. That includes the column we
            //    started in. This also covers the row.
            self.choose_row(r);

            // 4. Now that we've removed our chosen row, see if there is a solution:
            let solution_found = self.search_recursive(solution);

            // 5. Now re-add this row to the matrix for the next iteration (even if we found a solution, since the
            //    matrix itself should be re-usable over multiple searches).
            self.unchoose_row(r);

            if solution_found {
                return true;
            }

            // Try the next row:
            solution.pop();
            r = self.node(r).down;
        }

        false
    }

    fn choose_row(&mut self, r: NodeIndex) {
        let mut j = r;
        loop {
            self.remove_up_down(j);

            let header = self.column_for_node_mut(j);
            header.choices -= 1;
            header.count -= 1;
            if header.count == 0 {
                let col = header.index;
                self.cover_column(col);
            }

            j = self.node(j).right;
            if j == r {
                break;
            }
        }
    }

    fn unchoose_row(&mut self, r: NodeIndex) {
        let mut j = r;
        loop {
            let header = self.column_for_node_mut(j);
            header.choices += 1;
            header.count += 1;
            if header.count == 1 {
                let col = header.index;
                self.uncover_column(col);
            }

            self.restore_up_down(j);

            j = self.node(j).left;
            if j == r {
                break;
            }
        }
    }

    fn cover_column(&mut self, col: ColIndex) {
        let head = self.column(col).node;

        // This column has received its last required "covering." So it gets removed from the header list.
        self.remove_left_right(head);

        // Also, all remaining rows in this column now need to be removed from their other columns. This does *not*
        // cover those columns; it means those columns can no longer select those rows.
        let mut r = self.node(head).down;
        while r != head {
            let mut j = self.node(r).right;
            while j != r {
                self.remove_up_down(j);
                self.column_for_node_mut(j).choices -= 1;
                j = self.node(j).right;
            }
            r = self.node(r).down;
        }
    }

    fn uncover_column(&mut self, col: ColIndex) {
        let head = self.column(col).node;

        let mut r = self.node(head).up;
        while r != head {
            let mut j = self.node(r).left;
            while j != r {
                self.restore_up_down(j);
                self.column_for_node_mut(j).choices += 1;
                j = self.node(j).left;
            }
            r = self.node(r).up;
        }

        self.restore_left_right(head);
    }
}
