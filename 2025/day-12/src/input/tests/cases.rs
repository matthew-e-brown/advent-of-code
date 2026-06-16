pub use in_out::CASES as IN_OUT;
pub use transforms::CASES as TRANSFORMS;

use super::*;

// Ending an `indoc` with a newline causes that trailing newline to be included in the string. We don't want that. This
// macro forwards all tokens to `indoc`, but then trims off the end (which is thankfully const-compatible).
macro_rules! indoc {
    ($tt:tt) => {
        ::indoc::indoc! {$tt}.trim_ascii_end()
    };
}

mod in_out {
    use super::*;

    /// A test case for parsing/printing.
    pub struct TestCase {
        pub source: &'static str,
        pub expected_width: usize,
        pub expected_height: usize,
        pub expected_points: &'static [Point],
    }

    #[rustfmt::skip]
    pub const CASES: &[TestCase] = &[
        // Shapes from the day 12 example problem.
        TestCase {
            source: indoc! {"
                ###
                ##.
                ##.
            "},
            expected_width: 3,
            expected_height: 3,
            expected_points: &[
                    (0,0), (1,0), (2,0),
                    (0,1), (1,1),
                    (0,2), (1,2),
            ],
        },
        TestCase {
            source: indoc! {"
                ###
                ##.
                .##
            "},
            expected_width: 3,
            expected_height: 3,
            expected_points: &[
                (0,0), (1,0), (2,0),
                (0,1), (1,1),
                       (1,2), (2,2),
            ],
        },
        TestCase {
            source: indoc! {"
                .##
                ###
                ##.
            "},
            expected_width: 3,
            expected_height: 3,
            expected_points: &[
                       (1,0), (2,0),
                (0,1), (1,1), (2,1),
                (0,2), (1,2),
            ],
        },
        TestCase {
            source: indoc! {"
                ##.
                ###
                ##.
            "},
            expected_width: 3,
            expected_height: 3,
            expected_points: &[
                (0,0), (1,0),
                (0,1), (1,1), (2,1),
                (0,2), (1,2),
            ],
        },
        TestCase {
            source: indoc! {"
                ###
                #..
                ###
            "},
            expected_width: 3,
            expected_height: 3,
            expected_points: &[
                (0,0), (1,0), (2,0),
                (0,1),
                (0,2), (1,2), (2,2),
            ],
        },
        TestCase {
            source: indoc! {"
                ###
                .#.
                ###
            "},
            expected_width: 3,
            expected_height: 3,
            expected_points: &[
                (0,0), (1,0), (2,0),
                       (1,1),
                (0,2), (1,2), (2,2),
            ],
        },
        // Additional hand-written test cases for rectangular inputs:
        TestCase {
            source: indoc! {r"
                #.....
                ......
                .....#
            "},
            expected_width: 6,
            expected_height: 3,
            expected_points: &[(0,0), (5,2)],
        },
        TestCase {
            source: indoc! {"
                #
                .
                #
            "},
            expected_width: 1,
            expected_height: 3,
            expected_points: &[(0,0), (0,2)],
        },
    ];
}

mod transforms {
    use super::*;

    /// A test case for testing transformations.
    ///
    /// Since there are other tests verifying that parsing and printing work correctly, it's safe to use `to_string` to
    /// verify that transformations result in the correct shape.
    pub struct TestCase {
        /// A string describing the input shape.
        pub input: &'static str,
        /// An array of strings describing the result of each of the transformations
        pub results: [&'static str; Transform::VARIANTS.len()],
    }

    pub const CASES: &[TestCase] = &[
        // Asymmetrical square input:
        TestCase {
            input: indoc! {"
                #.#
                #..
                #..
            "},
            results: [
                // Identity
                indoc! {"
                    #.#
                    #..
                    #..
                "},
                // RotateCW
                indoc! {"
                    ###
                    ...
                    ..#
                "},
                // Rotate180
                indoc! {"
                    ..#
                    ..#
                    #.#
                "},
                // RotateCCW
                indoc! {"
                    #..
                    ...
                    ###
                "},
                // ReflectV
                indoc! {"
                    #.#
                    ..#
                    ..#
                "},
                // ReflectH
                indoc! {"
                    #..
                    #..
                    #.#
                "},
                // ReflectNE
                indoc! {"
                    ..#
                    ...
                    ###
                "},
                // ReflectSE
                indoc! {"
                    ###
                    ...
                    #..
                "},
            ],
        },
        // Annoyingly complicated rectangular example:
        TestCase {
            input: indoc! {"
                #..#..###.
                ####...#..
                ...#######
            "},
            results: [
                // Identity
                indoc! {"
                    #..#..###.
                    ####...#..
                    ...#######
                "},
                // RotateCW
                indoc! {"
                    .##
                    .#.
                    .#.
                    ###
                    #..
                    #..
                    #.#
                    ###
                    #.#
                    #..
                "},
                // Rotate180
                indoc! {"
                    #######...
                    ..#...####
                    .###..#..#
                "},
                // RotateCCW
                indoc! {"
                    ..#
                    #.#
                    ###
                    #.#
                    ..#
                    ..#
                    ###
                    .#.
                    .#.
                    ##.
                "},
                // ReflectV
                indoc! {"
                    .###..#..#
                    ..#...####
                    #######...
                "},
                // ReflectH
                indoc! {"
                    ...#######
                    ####...#..
                    #..#..###.
                "},
                // ReflectNE
                indoc! {"
                    #..
                    #.#
                    ###
                    #.#
                    #..
                    #..
                    ###
                    .#.
                    .#.
                    .##
                "},
                // ReflectSE
                indoc! {"
                    ##.
                    .#.
                    .#.
                    ###
                    ..#
                    ..#
                    #.#
                    ###
                    #.#
                    ..#
                "},
            ],
        },
        // Something with even width/height (no "middle" element to reflect/rotate around):
        TestCase {
            input: indoc! {"
                ####
                #.#.
                #...
                ####
            "},
            results: [
                // Identity
                indoc! {"
                    ####
                    #.#.
                    #...
                    ####
                "},
                // RotateCW
                indoc! {"
                    ####
                    #..#
                    #.##
                    #..#
                "},
                // Rotate180
                indoc! {"
                    ####
                    ...#
                    .#.#
                    ####
                "},
                // RotateCCW
                indoc! {"
                    #..#
                    ##.#
                    #..#
                    ####
                "},
                // ReflectV
                indoc! {"
                    ####
                    .#.#
                    ...#
                    ####
                "},
                // ReflectH
                indoc! {"
                    ####
                    #...
                    #.#.
                    ####
                "},
                // ReflectNE
                indoc! {"
                    #..#
                    #.##
                    #..#
                    ####
                "},
                // ReflectSE
                indoc! {"
                    ####
                    #..#
                    ##.#
                    #..#
                "},
            ],
        },
    ];
}
