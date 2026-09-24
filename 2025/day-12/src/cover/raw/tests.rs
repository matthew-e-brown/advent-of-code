use super::Node;
use super::build::{Constraint, DLXBuilder};
use super::index::*;

/// Creates a new `Node { ... }` literal by manually specifying indices in `U, D, L, R` order.
///
/// Rows and columns are specified using `r: ...` and `c: ...` after the other indices. Row/column can be omitted to set
/// it to `RowIndex::NONE` or `ColIndex::NONE`.
macro_rules! node {
    // 'r: ...' and 'c: ...' are optional at the end of the four, in either order.
    ($u:expr, $d:expr, $l:expr, $r:expr $(, r: $row:expr)? $(, c: $col:expr)?) => {
        node!(@impl: $u, $d, $l, $r, row: node!(@r: $($row)?), col: node!(@c: $($col)?))
    };
    ($u:expr, $d:expr, $l:expr, $r:expr $(, c: $col:expr)? $(, r: $row:expr)?) => {
        node!(@impl: $u, $d, $l, $r, row: node!(@r: $($row)?), col: node!(@c: $($col)?))
    };
    // These internal arms resolve a missing 'r: ...' or 'c: ...' into NONE:
    (@r:) => (RowIndex::NONE.0);
    (@c:) => (ColIndex::NONE.0);
    // If the 'r: ...' or 'c: ...' weren't missing, they get passed through as-is.
    (@r: $row:expr) => ($row);
    (@c: $col:expr) => ($col);
    (@impl: $u:expr, $d:expr, $l:expr, $r:expr, row: $row:expr, col: $col:expr) => {
        Node {
            column: ColIndex($col),
            row: RowIndex($row),
            up: NodeIndex($u),
            down: NodeIndex($d),
            left: NodeIndex($l),
            right: NodeIndex($r),
        }
    };
}

/// Tests building the matrix for a simple example:
///
/// ```txt
///     | 0 | 1 | 2 | 3 |
/// |---|---|---|---|---|
/// | A | 1 |   | 1 |   |
/// | B | 1 |   | 1 | 1 |
/// | C |   | 1 |   |   |
/// | D |   |   | 1 | 1 |
/// ```
#[test]
fn build_simple() {
    let constraints = Constraint::required().repeat(4);
    let matrix = DLXBuilder::from_constraints(constraints)
        .row([0, 2])
        .row([0, 2, 3])
        .row([1])
        .row([2, 3])
        .finish();

    // To ensure valid construction, we manually specify what the nodes should look like:
    #[rustfmt::skip]
    let expected_nodes: &[Node] = &[
        // Root node:
        /*  0 */ node!( 0,  0,  4,  1),
        // Header nodes:
        /*  1 */ node!( 7,  5,  0,  2, c: 0),
        /*  2 */ node!(10, 10,  1,  3, c: 1),
        /*  3 */ node!(11,  6,  2,  4, c: 2),
        /*  4 */ node!(12,  9,  3,  0, c: 3),
        // Body nodes:
        /*  5 */ node!( 1,  7,  6,  6, r: 0, c: 0),
        /*  6 */ node!( 3,  8,  5,  5, r: 0, c: 2),
        /*  7 */ node!( 5,  1,  9,  8, r: 1, c: 0),
        /*  8 */ node!( 6, 11,  7,  9, r: 1, c: 2),
        /*  9 */ node!( 4, 12,  8,  7, r: 1, c: 3),
        /* 10 */ node!( 2,  2, 10, 10, r: 2, c: 1),
        /* 11 */ node!( 8,  3, 12, 12, r: 3, c: 2),
        /* 12 */ node!( 9,  4, 11, 11, r: 3, c: 3),
    ];

    assert_eq!(&matrix.nodes[..], expected_nodes);
}

/// Solves the "simple" example from above.
#[test]
fn solve_simple() {
    let constraints = Constraint::required().repeat(4);
    let mut matrix = DLXBuilder::from_constraints(constraints)
        .row([0, 2])
        .row([0, 2, 3])
        .row([1])
        .row([2, 3])
        .finish();

    let solution = matrix.search();
    println!("Solution: {solution:?}");
}

/// Solves the example from Wikipedia.
///
/// https://en.wikipedia.org/w/index.php?title=Knuth%27s_Algorithm_X&oldid=1267470602#Example
///
/// In the example from Wikipedia, there are 7 columns and six rows (renumbered here to start from 0):
///
/// ```txt
///     | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
/// |---|---|---|---|---|---|---|---|
/// | A | 1 |   |   | 1 |   |   | 1 |
/// | B | 1 |   |   | 1 |   |   |   |
/// | C |   |   |   | 1 | 1 |   | 1 |
/// | D |   |   | 1 |   | 1 | 1 |   |
/// | E |   | 1 | 1 |   |   | 1 | 1 |
/// | F |   | 1 |   |   |   |   | 1 |
/// ```
#[test]
fn solve_wikipedia() {
    let constraints = Constraint::required().repeat(7);
    let mut matrix = DLXBuilder::from_constraints(constraints)
        .row([0, 3, 6])
        .row([0, 3])
        .row([3, 4, 6])
        .row([2, 4, 5])
        .row([1, 2, 5])
        .row([1, 6])
        .finish();

    let solution = matrix.search();
    println!("Solution: {solution:?}");
}

/// Solves an example from my notebook.
#[test]
fn solve_notebook() {
    let constraints = [
        Constraint::required().with_count(2),
        Constraint::required().with_count(1),
    ]
    .into_iter()
    .chain(Constraint::optional().repeat(8));

    let rows: [&[usize]; 9] = [
        &[0, 2, 3],
        &[0, 3, 4],
        &[0, 4, 5],
        &[0, 5, 6],
        &[0, 6, 7],
        &[0, 8, 9],
        &[1, 3, 6, 7],
        &[1, 4, 7, 8],
        &[1, 5, 8, 9],
    ];

    let mut builder = DLXBuilder::from_constraints(constraints);
    for row in rows {
        builder.push_row(row.into_iter().copied());
    }

    let mut matrix = builder.finish();
    let solution = matrix.search();
    println!("Solution: {solution:?}");
}
