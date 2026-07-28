// [TODO] Re-add doc-comment.
#![allow(dead_code)]

pub mod builder;
pub mod error;
mod index;

use self::builder::MatrixBuilder;
pub use self::index::*;


/// A matrix that implements a modified version of Donald Knuth's _Algorithm X._
///
/// Rows and columns are identified by their indices.
pub struct Matrix {
    nodes: Box<[Node]>,
    col_headers: Box<[ColHeader]>,
    row_headers: Box<[RowHeader]>,
}

#[derive(Debug, Clone)]
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
    /// Creates a new empty [`MatrixBuilder`].
    pub fn build() -> MatrixBuilder {
        MatrixBuilder::new()
    }

    // [TODO] A way to pass preliminary modifications before doing the proper search (and then undo them afterwards).
    #[allow(unused)]
    pub fn search(&mut self) -> Option<Vec<RowIndex>> {
        let mut solution = Vec::new();
        if self.search_recursive(&mut solution) {
            let rows = solution.into_iter().map(|r| self.row_header(r).index()).collect();
            Some(rows)
        } else {
            None
        }
    }

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

    fn search_recursive(&mut self, solution: &mut Vec<RowIndex>) -> bool {
        // 1. Find the next column to cover.
        let col = match self.select_next_column() {
            ColumnResult::Next(col) => col,
            ColumnResult::SearchSuccess => return true,
            ColumnResult::SearchFailure => return false,
        };

        // 2. Mark this column as being covered. It technically hasn't actually been covered yet, but that's what we're
        //    working on right now; it will be covered once we actually make our choice of row.
        self.cover_column(col);

        // 3. Attempt all rows within this column. (TODO: maybe sort them first?)
        let mut solution_found = false;

        let start = self.column(col).node;
        let mut r = self.node(start).down;
        while r != start {
            // 1. First, take note that we are attempting this row.
            solution.push(self.node(r).row);

            // 2. Cover all the columns this row is a part of; they have now had their criteria met.
            self.choose_row(r);

            // 3. Before we descend recursively, we need to ensure that this row is not selected again. This is not a
            //    problem in the base algorithm since, in that version, the current column would have been removed
            //    entirely.
            self.remove_up_down(r);

            // 4. Do we find a solution after covering this row?
            solution_found = self.search_recursive(solution);

            // 5. Even if we found a solution, be sure to restore the matrix before we check `success` and move on. We
            //    need to ensure that the matrix is back in its original state so that it can be reused for another
            //    puzzle.
            self.restore_up_down(r);
            self.unchoose_row(r);

            // 6. If we did find a solution, we can break out of here! No need to check any more rows.
            if solution_found {
                break;
            }

            solution.pop();
            r = self.node(r).down;
        }

        // 4. Regardless of if we saw success or not, we need to leave the matrix in the same state we found it in
        //    before returning.
        self.uncover_column(col);

        solution_found
    }

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

    fn cover_column(&mut self, col: ColIndex) {
        // In our version of DLX, covering a column starts only by decrementing the `count` of the column. Only if the
        // count hits zero do we actually consider the criteria fully met. This way, all the rows in this column remain
        // in this column as valid choices for the next step of the algorithm. Then, before we "choose" each row, we
        // remove that specific one
        self.column_mut(col).count -= 1;
        if self.column(col).count == 0 {
            let start: NodeIndex = self.column(col).node;

            // Once the column's criteria has been fully met, that means that all rows within this column are no longer
            // valid choices for any other columns. Additionally, it means that this column should never be selected
            // again. So, we start by removing this column's header from the header list (left/right).
            self.remove_left_right(start);

            // Next, we loop down the rows of this column and remove their nodes from all other column lists. NOTE THAT
            // THIS IS A ONE-WAY OPERATION. It means that all the *other* rows above and below this one no longer point
            // into this row, but we still point outwards to them (which allows the next phase of the algorithm, where
            // we select one of these rows as our next choice, to continue).
            let mut r: NodeIndex = self.node(start).down;
            while r != start {
                let mut j: NodeIndex = self.node(r).right;
                while j != r {
                    self.column_for_node_mut(j).choices -= 1;
                    self.remove_up_down(j);
                    j = self.node(j).right;
                }

                r = self.node(r).down;
            }
        }
    }

    fn uncover_column(&mut self, col: ColIndex) {
        // Uncovering a column is the inverse of marking it as covered. Again, in our version of DLX, that means that we
        // only need to do any proper changes if the column starts as being "completely" covered (i.e., had a count of
        // zero before we re-incremented => now has a count of 1).
        self.column_mut(col).count += 1;
        if self.column(col).count == 1 {
            // For all rows within this column, going up this time...
            let start: NodeIndex = self.column(col).node;
            let mut r: NodeIndex = self.node(start).up;
            while r != start {
                // ...and for all cells within this row (except this particular node), going left this time...
                let mut j: NodeIndex = self.node(r).left;
                while j != r {
                    // ...re-add them to their column list.
                    self.restore_up_down(j);
                    self.column_for_node_mut(j).choices += 1;
                    j = self.node(j).left;
                }

                r = self.node(r).up;
            }

            self.restore_left_right(start);
        }
    }

    fn choose_row(&mut self, start: NodeIndex) {
        let node = self.node(start);
        let mut j = node.right;

        #[cfg(debug_assertions)]
        self.row_header_mut(node.row).assert_unchosen();

        while j != start {
            let col = self.node(j).column;
            self.column_mut(col).choices -= 1;
            self.cover_column(col);
            self.remove_up_down(j);

            j = self.node(j).right;
        }
    }

    fn unchoose_row(&mut self, start: NodeIndex) {
        let node = self.node(start);
        let mut j = node.left;

        #[cfg(debug_assertions)]
        self.row_header_mut(node.row).assert_chosen();

        while j != start {
            let col = self.node(j).column;
            self.restore_up_down(j);
            self.uncover_column(col);
            self.column_mut(col).choices += 1;

            j = self.node(j).left;
        }
    }
}
