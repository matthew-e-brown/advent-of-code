// Next [TODO]: Get the *construction* of the matrix working so that we can build a simple example in a `#[test]` and
// see if this code works on the basic example from Wikipedia. *Then* we can worry about handling size/count.

// #![allow(unused)]
//! Custom implementation of Donald Knuth's _Algorithm X._
//!
//! # Overview
//!
//! 1.  Some columns describe optional criteria. These are criteria that do not need to be met; but, if they are, all
//!     rows that conflict with them are covered.
//!     - In our specific case, that's one column for each tile in the [`Region`].
//! 2.  Some columns describe criteria that must be met exactly `N` times.
//!     - In our case, that's one criteria for each [`PresentShape`], with `N` equal to their count in the [`Region`].
//!
//! To make 2. work, we have all rows within that column share a common count (stored within the column header). When an
//! `N` column is "covered", the count is decremented. The column remains in the primary list of columns until this
//! count hits zero. This essentially allows the column to act as multiple columns in one.
//!
//! The `count` on a multi-column is different from its `size` column; that one dictates how many *rows* still remain.
//! When using the `S` heuristic from Knuth's paper, the `size` of a column still represents the "branching factor" of
//! the column; it's just that, now, it also represents the branching factor of `N-1` other "pseudo-columns".
//!
//! To make 1. work, we need to have a quick way to check if the list of columns is empty, but while ignoring the
//! optional columns. The way Knuth does this is by excluding the secondary columns from the main list entirely; their
//! left and right pointers simply point to themselves. This is elegant in that it lets the algorithm continue
//! completely as normal. However, in our case, we want to give the outside user a way to run a preliminary pass over
//! all the columns. This... actually, should work totally fine? We already have a way to iterate over all column
//! headers, even if they aren't attached: looping over the array!
//!
//! # Implementation details
//!
//! ## Column layout
//!
//! - Every single column gets a `count` property to handle multi-columns; the non-multi-columns just get it set to 1. A
//!   multi-column with a count of `n` is equivalent to having `n` copies of the rows all with a 1 in their cell.
//! - Optional columns have their `left`/`right` properties set to point to themselves. This makes them reachable while
//!   covering a row but not while selecting a column to cover. They still have a count, like usual.
//! - Matrix looks like:
//!   - One non-optional column for each possible puzzle piece (present shape).
//!   - One optional column for every single cell in the largest possible region.
//!   - Then, for each puzzle piece, one row is generated for each of the possible orientations
//!
//! ## Before running
//!
//! - The differences between all the regions are: (1) they have a different size, and (2) they require different
//!   numbers of pieces. So, before each run of the algorithm, we let the caller iterate over all column headers. This
//!   lets them (a) adjust the counts for the puzzle columns, and (b) pre-cover the tile columns that fall outside the
//!   next region.
//!
//! ## Algorithm
//!
//! 1.   Select the next column to cover according to some deterministic heuristic.
//!      - We select the column with the smallest total "branching factor". The branching factor for a column is the
//!        product of its `count` and its `size`.
//!      - This choice is never backtracked. Once we select a column, we find the best possible
//! 2.   "Cover" the chosen column:
//!      1.  Decrement this column's `count` by one. If the count reaches zero, remove it from the list of column
//!          headers.
//!      2.  For each node in this column, run along its respective row and remove each node from all the other column
//!          lists they appear in. When you do, decrement the `size` of that node's column.
//!      -   In a sense, we haven't *actually* covered this column yet: we mark it as covered so that, when we step down
//!          to the next layer of the algorithm, the subproblem we end up considering is "all the cases where this
//!          column is already handled." This column will be truly covered once we reach the end of the recursive step.
//! 3.   Order the rows within the chosen column by some other heuristic.
//!      - In theory, as long as we allow for the choice of row to be backtracked, this algorithm will eventually
//!        enumerate all solutions. But we don't want to find *all* solutions, we just need to find *one*. So, we will
//!        select a row which appears to (a) minimize the branching factor and (b) have the most likely chance of being
//!        correct.
//! 4.   Iterate over all rows in the chosen column. This represents "attempting" each row's choice. For each row:
//!      1.  Iterate over all nodes in the row; every column in which a node appears has now had its criteria met. Mark
//!          it as covered:
//!          1.  Same as before: decrement the column's `count` by one, and if it is zero, remove it from the column
//!              list.
//!
//! ...TODO

pub struct Matrix<R, C = ()> {
    nodes: Box<[Node]>,
    col_headers: Box<[ColHeader<C>]>,
    row_headers: Box<[RowHeader<R>]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ColIndex(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RowIndex(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NodeIndex(u32);

#[derive(Debug)]
struct Node {
    column: ColIndex,
    row: RowIndex,
    up: NodeIndex,
    down: NodeIndex,
    left: NodeIndex,
    right: NodeIndex,
}

#[derive(Debug)]
struct RowHeader<R> {
    /// User-provided data used to identify this row.
    name: R,
    /// Whether or not this row is used in the solution. Used for debugging.
    #[cfg(debug_assertions)]
    in_solution: bool,
}

#[derive(Debug)]
struct ColHeader<C> {
    /// User-provided data used to identify this column.
    name: C,
    /// The number of rows still remaining in this column.
    choices: usize,
    /// The remaining number of times this column must be covered before it is considered done.
    count: usize,
    /// The index of the dummy-node at the start of this column's list of nodes.
    node: NodeIndex,
}

impl NodeIndex {
    pub const ROOT: NodeIndex = NodeIndex(0);
}

impl ColIndex {
    /// The [`ColIndex`] used by the root node (`h`) to denote that it does not have a column header.
    pub const NONE: ColIndex = ColIndex(u32::MAX);
}

impl RowIndex {
    /// The [`RowIndex`] used by the nodes in the header row to denote that they do not have a row header.
    pub const NONE: RowIndex = RowIndex(u32::MAX);
}

impl<C> ColHeader<C> {
    /// Gets the name of this column.
    pub fn name(&self) -> &C {
        &self.name
    }
}

impl<R> RowHeader<R> {
    /// Gets the name of this row.
    pub fn name(&self) -> &R {
        &self.name
    }

    #[cfg(debug_assertions)]
    fn mark_chosen_assert(&mut self) {
        match self.in_solution {
            true => panic!("attempted to use the same row twice in the same solution"),
            false => self.in_solution = true,
        };
    }

    #[cfg(debug_assertions)]
    fn mark_unchosen_assert(&mut self) {
        match self.in_solution {
            false => panic!("attempted to restore the same row more than once"),
            true => self.in_solution = false,
        }
    }
}

enum ColumnResult {
    Next(ColIndex),
    SearchSuccess,
    SearchFailure,
}

impl<R, C> Matrix<R, C> {
    fn root(&self) -> &Node {
        self.node(NodeIndex::ROOT)
    }

    fn node(&self, i: NodeIndex) -> &Node {
        &self.nodes[i.0 as usize]
    }

    fn node_mut(&mut self, i: NodeIndex) -> &mut Node {
        &mut self.nodes[i.0 as usize]
    }

    fn column(&self, i: ColIndex) -> &ColHeader<C> {
        &self.col_headers[i.0 as usize]
    }

    fn column_mut(&mut self, i: ColIndex) -> &mut ColHeader<C> {
        &mut self.col_headers[i.0 as usize]
    }

    fn row_header(&self, i: RowIndex) -> &RowHeader<R> {
        &self.row_headers[i.0 as usize]
    }

    fn row_header_mut(&mut self, i: RowIndex) -> &mut RowHeader<R> {
        &mut self.row_headers[i.0 as usize]
    }

    fn column_for_node(&self, i: NodeIndex) -> &ColHeader<C> {
        self.column(self.node(i).column)
    }

    fn column_for_node_mut(&mut self, i: NodeIndex) -> &mut ColHeader<C> {
        self.column_mut(self.node(i).column)
    }
}

impl<R, C> Matrix<R, C> {
    // [TODO] A way to pass preliminary modifications before doing the proper search (and then undo them afterwards).
    #[allow(unused)]
    pub fn search(&mut self) -> Option<Vec<&R>> {
        let mut solution = Vec::new();
        if self.search_recursive(&mut solution) {
            let rows = solution.into_iter().map(|r| self.row_header(r).name()).collect();
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
        self.row_header_mut(node.row).mark_chosen_assert();

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
        self.row_header_mut(node.row).mark_unchosen_assert();

        while j != start {
            let col = self.node(j).column;
            self.restore_up_down(j);
            self.uncover_column(col);
            self.column_mut(col).choices += 1;

            j = self.node(j).left;
        }
    }
}
