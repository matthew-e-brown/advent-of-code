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
struct ColIndex(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RowIndex(usize);

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
    name: R,
}

#[derive(Debug)]
struct ColHeader<C> {
    name: C,
    size: usize,
    count: usize,
    node: NodeIndex,
}

impl NodeIndex {
    pub const ROOT: NodeIndex = NodeIndex(0);
}

impl ColIndex {
    pub const NONE: ColIndex = ColIndex(usize::MAX);
}

impl RowIndex {
    pub const NONE: RowIndex = RowIndex(usize::MAX);
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
        &self.col_headers[i.0]
    }

    fn column_mut(&mut self, i: ColIndex) -> &mut ColHeader<C> {
        &mut self.col_headers[i.0]
    }

    fn row_header(&self, i: RowIndex) -> &RowHeader<R> {
        &self.row_headers[i.0]
    }

    fn row_header_mut(&mut self, i: RowIndex) -> &mut RowHeader<R> {
        &mut self.row_headers[i.0]
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
            let rows = solution.into_iter().map(|r| &self.row_header(r).name).collect();
            Some(rows)
        } else {
            None
        }
    }

    fn search_recursive(&mut self, solution: &mut Vec<RowIndex>) -> bool {
        // 1. Find the next column to cover.
        let Some(col) = self.select_next_column() else {
            // There are no more columns! We have covered all the required criteria. We're done.
            return true;
        };

        // 2. Mark this column as being covered.
        self.cover_column(col);

        // 3. Attempt all rows within this column (TODO: sort them first).
        let mut success = false;

        let c = self.column(col).node;
        let mut r = self.node(c).down;
        while r != c {
            // 1. First, take note that we are attempting this row.
            solution.push(self.node(r).row);

            // 2. Go ahead and cover all the columns this row is a part of. Recall that the current column has already
            // been covered, so we start one to the right.
            let mut j = self.node(r).right;
            while j != r {
                self.cover_column(self.node(j).column);
                j = self.node(j).right;
            }

            // 3. Do we find a solution after covering this row?
            success = self.search_recursive(solution);

            // 4. Before we check that and move on, restore the matrix to how it was before.
            let mut j = self.node(r).left;
            while j != r {
                self.uncover_column(self.node(j).column);
                j = self.node(j).left;
            }

            // 5. If we did find a solution, we can break out of here! No need to check any more rows.
            if success {
                break;
            }

            solution.pop();
            r = self.node(r).down;
        }

        // Regardless of if we saw success or not, we need to leave the matrix in the same state we found it in before
        // returning.
        self.uncover_column(col);

        success
    }

    fn select_next_column(&self) -> Option<ColIndex> {
        // Look for the column with the smallest branching factor.
        let mut min = None;
        let mut idx = self.root().right;

        while idx != NodeIndex::ROOT {
            let col_index = self.node(idx).column;
            let col_header = self.column(col_index);
            let branch_factor = col_header.count * col_header.size;

            if min.is_none_or(|(_, min_bf)| branch_factor < min_bf) {
                min = Some((col_index, branch_factor));
            }

            idx = self.node(idx).right;
        }

        Some(min?.0)
    }

    fn cover_column(&mut self, col: ColIndex) {
        let c = self.column(col).node;

        // [TODO] Handle size/count here.

        // Remove node `c` from the header list by making its L/R pointers go around it
        let l = self.node(c).left;
        let r = self.node(c).right;
        self.node_mut(r).left = l;
        self.node_mut(l).right = r;

        // For all rows within this column...
        let mut i: NodeIndex = self.node(c).down;
        while i != c {
            // ... and for all nodes within this row (EXCEPT this particular node) ...
            let mut j: NodeIndex = self.node(i).right;
            while j != i {
                // [TODO] Handle size/count here.

                // ...remove them from their column list. This means that those columns no longer contain this row.
                let u = self.node(j).up;
                let d = self.node(j).down;
                self.node_mut(d).up = u;
                self.node_mut(u).down = d;

                j = self.node(j).right;
            }

            i = self.node(i).down;
        }
    }

    fn uncover_column(&mut self, col: ColIndex) {
        let c = self.column(col).node;

        // For all rows within this column, going up this time...
        let mut i = self.node(c).up;
        while i != c {
            // ...and for all cells within this row (EXCEPT this particular node), going left this time...
            let mut j = self.node(i).left;
            while j != i {
                // [TODO] Handle size/count here.

                // ...re-add them to their column list.
                let u = self.node(j).up;
                let d = self.node(j).down;
                self.node_mut(d).up = j;
                self.node_mut(u).down = j;

                j = self.node(j).left;
            }

            i = self.node(i).up;
        }

        // [TODO] Handle size/count here.

        // Now we can re-add the column header back into the list.
        let r = self.node(c).left;
        let l = self.node(c).right;
        self.node_mut(r).left = c;
        self.node_mut(l).right = c;
    }
}
