use super::*;

impl Matrix {
    // [TODO] A way to pass preliminary modifications before doing the proper search (and then undo them afterwards).
    pub fn search(&mut self) -> Option<Vec<usize>> {
        let mut solution = Vec::new();
        if self.search_recursive(&mut solution) {
            Some(solution)
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

        let mut solution_found = false;

        // 2. Start picking rows for this column.
        let col_head_idx: NodeIndex = self.column(col).node;
        let mut r: NodeIndex = self.node(col_head_idx).down;
        while r != col_head_idx {
            solution.push(self.node(r).row.into());

            // 3. Remove this row from all its columns. They can no longer see the row. That includes the column we
            // started in. This also covers the row.
            let mut j = r;
            loop {
                self.remove_up_down(j); // Remove row from column
                let j_header = self.column_for_node_mut(j);
                j_header.choices -= 1; // Column no longer has access to this row
                j_header.count -= 1; // Column has been covered by choosing this row
                if j_header.count == 0 {
                    let j_head_idx = j_header.node;

                    // Since this was the last covering this column needed (its count has hit zero), the column gets
                    // removed from the list of columns.
                    self.remove_left_right(j_head_idx);

                    // Also, all remaining rows in this column (which does *not* include the current `r` row) now need
                    // to be removed from their other columns. This does *not* cover the column; it means those columns
                    // can no longer select these rows.
                    let mut r = self.node(j_head_idx).down;
                    while r != j_head_idx {
                        let mut j = self.node(r).right;
                        while j != r {
                            self.remove_up_down(j);
                            self.column_for_node_mut(j).choices -= 1;
                            j = self.node(j).right;
                        }
                        r = self.node(r).down;
                    }
                }

                j = self.node(j).right;
                if j == r {
                    break;
                }
            }

            // Now that we've removed our chosen row, see if there is a solution:
            solution_found = self.search_recursive(solution);

            // Now re-add this row to the matrix for the next iteration (even if we found a solution, since the matrix
            // itself should be re-usable over multiple searches).
            let mut j = r;
            loop {
                let j_header = self.column_for_node_mut(j);
                j_header.choices += 1;
                j_header.count += 1;
                if j_header.count == 1 {
                    let j_head_idx = j_header.node;

                    // This column has just been uncovered from zero. Invert the removal we did above.
                    let mut r = self.node(j_head_idx).up;
                    while r != j_head_idx {
                        let mut j = self.node(r).left;
                        while j != r {
                            self.column_for_node_mut(j).choices += 1;
                            self.restore_up_down(j);
                            j = self.node(j).left;
                        }
                        r = self.node(r).up;
                    }

                    // It also gets re-added to the column list.
                    self.restore_left_right(j_head_idx);
                }
                self.restore_up_down(j);

                j = self.node(j).left;
                if j == r {
                    break;
                }
            }

            if solution_found {
                break;
            } else {
                // Try the next row:
                solution.pop();
                r = self.node(r).down;
            }
        }

        solution_found
    }
}
