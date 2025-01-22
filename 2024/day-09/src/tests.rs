/*
 * This one got tests because my actual test input resulted in some thing far too large to debug with print statements
 * (and I don't feel like setting up Rust debugging right now). My real puzzle input created a map that was some 94,000.
 * Even though I ended up solving the issue without the help of the tests, I figured it couldn't hurt to keep them
 * around.
 *
 * I fixed it by adding the `k < limit` loop condition in `find_slot_range` that I *thought* was optional, having
 * already been covered by `while start < limit`, not realizing that it was very much *not* already covered.
 */

use super::*;

#[test]
fn input_parsing1() {
    const INPUT: &str = "12345";

    #[rustfmt::skip]
    let expected = &[
        Some(0),
        None, None,
        Some(1), Some(1), Some(1),
        None, None, None, None,
        Some(2), Some(2), Some(2), Some(2), Some(2),
    ];

    let (actual, largest) = parse_input(&INPUT);
    assert_eq!(2, largest, "largest ID");
    assert_eq!(expected, &actual[..], "parsed input");
}

#[test]
fn input_parsing2() {
    const INPUT: &str = "2333133121414131402";

    // `00...111...2...333.44.5555.6666.777.888899`
    #[rustfmt::skip]
    let expected = &[
        Some(0), Some(0),
        None, None, None,
        Some(1), Some(1), Some(1),
        None, None, None,
        Some(2),
        None, None, None,
        Some(3), Some(3), Some(3),
        None,
        Some(4), Some(4),
        None,
        Some(5), Some(5), Some(5), Some(5),
        None,
        Some(6), Some(6), Some(6), Some(6),
        None,
        Some(7), Some(7), Some(7),
        None,
        Some(8), Some(8), Some(8), Some(8),
        Some(9), Some(9),
    ];

    let (actual, largest) = parse_input(&INPUT);
    assert_eq!(9, largest, "largest ID");
    assert_eq!(expected, &actual[..], "parsed input");
}

#[test]
fn trim_nones() {
    let input = &mut [
        Some(1),
        Some(2),
        Some(3),
        None,
        None,
        Some(4),
        Some(4),
        None,
        None,
        None,
    ];
    let expected = &mut [Some(1), Some(2), Some(3), None, None, Some(4), Some(4)];
    assert_eq!(expected, super::trim_nones(input));
}

// 00...111...2...333.44.5555.6666.777.888899
// ^         ^         ^         ^         ^
// 012345678901234567890123456789012345678901
//           1         2         3         4

#[test]
fn file_scanning() {
    const INPUT: &str = "2333133121414131402";
    let (map, _) = parse_input(&INPUT);

    assert_eq!(next_file(&map, map.len() - 1), Some(41));
    assert_eq!(next_file(&map, 40), Some(40));
    assert_eq!(next_file(&map, 14), Some(11));
    assert_eq!(next_file(&map, 0), Some(0));

    // Under normal use, the only way this function returns `None` is if it runs off the start. But we should also
    // always have a file right at the start.
    assert_eq!(next_file(&map, map.len() + 4), None);
}

#[test]
fn slot_scanning() {
    const INPUT: &str = "2333133121414131402";
    let (map, _) = parse_input(&INPUT);

    assert_eq!(next_slot(&map, 0), Some(2));
    assert_eq!(next_slot(&map, 23), Some(26));
    assert_eq!(next_slot(&map, 33), Some(35));
    assert_eq!(next_slot(&map, 36), None);
}

// 00...111...2...333.44.5555.6666.777.888899
// ^         ^         ^         ^         ^
// 012345678901234567890123456789012345678901
//           1         2         3         4

#[test]
fn slot_range_scanning() {
    const INPUT: &str = "2333133121414131402";
    let (map, _) = parse_input(&INPUT);

    assert_eq!(find_slot_range(&map, 2, 0, map.len()), Some(2..4));
    assert_eq!(find_slot_range(&map, 3, 0, map.len()), Some(2..5));
    assert_eq!(find_slot_range(&map, 5, 0, map.len()), None); // No gaps with size 5
    assert_eq!(find_slot_range(&map, 2, 20, map.len()), None);
    assert_eq!(find_slot_range(&map, 1, 24, map.len()), Some(26..27));
    assert_eq!(find_slot_range(&map, 3, 6, 9), None);
    assert_eq!(find_slot_range(&map, 3, 6, 11), Some(8..11));

    // This assertion has the `limit` in a weird spot that would technically never happen (the end of a file block), but
    // it catches the `k < limit` case I mentioned in the comment above. Without the extra check on `k`, the `next_slot`
    // call will return `Some(8)`, then start scanning. Since it manages to find a space before the loop ever restarts
    // to hit the `start < limit` condition, the old version of the code breaks here.
    assert_eq!(find_slot_range(&map, 3, 5, 7), None);
}

#[test]
fn file_range_scanning() {
    const INPUT: &str = "2333133121414131402";
    let (map, _) = parse_input(&INPUT);

    assert_eq!(find_file_range(&map, 9, map.len() - 1), Some(40..42));
    assert_eq!(find_file_range(&map, 8, map.len() - 1), Some(36..40));
    assert_eq!(find_file_range(&map, 8, 35), None);
    assert_eq!(find_file_range(&map, 1, 20), Some(5..8));
    assert_eq!(find_file_range(&map, 0, map.len() - 1), Some(0..2));
}
